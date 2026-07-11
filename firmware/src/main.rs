#![no_std]
#![no_main]

extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;
use contract::{Command, DeviceMsg};
use embassy_executor::Spawner;
use embassy_futures::join::join;
use embassy_futures::select::{select, Either};
use embassy_rp::bind_interrupts;
use embassy_rp::gpio::{Level, Output};
use embassy_rp::peripherals::USB;
use embassy_rp::usb::{Driver, InterruptHandler};
use embassy_time::Timer;
use embassy_usb::class::cdc_acm::{CdcAcmClass, State};
use embassy_usb::driver::EndpointError;
use embassy_usb::{Builder, Config};
use embedded_alloc::LlffHeap as Heap;
use panic_halt as _;

#[global_allocator]
static HEAP: Heap = Heap::empty();

// RP2350 boot image signature; see firmware/memory.x `.start_block`.
#[link_section = ".start_block"]
#[used]
pub static IMAGE_DEF: embassy_rp::block::ImageDef = embassy_rp::block::ImageDef::secure_exe();

bind_interrupts!(struct Irqs {
    USBCTRL_IRQ => InterruptHandler<USB>;
});

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    // Heap for the contract's alloc-backed types + serde_json.
    {
        use core::mem::MaybeUninit;
        const HEAP_SIZE: usize = 8192;
        static mut HEAP_MEM: [MaybeUninit<u8>; HEAP_SIZE] = [MaybeUninit::uninit(); HEAP_SIZE];
        unsafe { HEAP.init(core::ptr::addr_of_mut!(HEAP_MEM) as usize, HEAP_SIZE) }
    }

    let p = embassy_rp::init(Default::default());
    let mut led = Output::new(p.PIN_15, Level::Low);

    // Boot-blink: 3 quick flashes on GP15 so every (re)flash is visibly confirmed. The
    // LED is otherwise idle until commanded, which once hid a stale-flash bug on the bench.
    for _ in 0..6 {
        led.toggle();
        Timer::after_millis(80).await;
    }
    led.set_low();

    // --- USB CDC-ACM setup ---
    let driver = Driver::new(p.USB, Irqs);

    let mut config = Config::new(0x2e8a, 0x0009); // Raspberry Pi vendor id
    config.manufacturer = Some("Mini-Molecule");
    config.product = Some("Mini-Molecule");
    config.serial_number = Some("mol-001");
    config.max_power = 100;
    config.max_packet_size_0 = 64;

    let mut config_descriptor = [0u8; 256];
    let mut bos_descriptor = [0u8; 256];
    let mut control_buf = [0u8; 64];
    let mut state = State::new();

    let mut builder = Builder::new(
        driver,
        config,
        &mut config_descriptor,
        &mut bos_descriptor,
        &mut [], // no MS OS descriptors
        &mut control_buf,
    );

    let mut class = CdcAcmClass::new(&mut builder, &mut state, 64);
    let mut usb = builder.build();

    let usb_fut = usb.run();

    // Announce + serve commands forever. If the USB endpoint errors (host reset),
    // loop and start a fresh comms cycle.
    let comms_fut = async {
        loop {
            let _ = run_comms(&mut class, &mut led).await;
        }
    };

    join(usb_fut, comms_fut).await;
}

/// Announce immediately, then serve commands while re-announcing on a 1s timer.
///
/// Why re-announce? The host bridge reopens the serial port on each browser connection,
/// but the firmware can't see an OS-level port close, so a freshly-connected host would
/// otherwise never receive a SelfId. The timer guarantees any listener sees us within 1s.
/// DTR is deliberately NOT used as a gate — a raw serial host may never assert it.
async fn run_comms<'d, D: embassy_usb::driver::Driver<'d>>(
    class: &mut CdcAcmClass<'d, D>,
    led: &mut Output<'static>,
) -> Result<(), EndpointError> {
    // Build the invariant announce line and capability list once per comms cycle.
    // Both are constant for the device's life, so re-announcing on the timer reuses
    // `hello` instead of rebuilding + re-serializing the SelfId every second.
    let self_id = contract::default_self_id();
    let hello = serde_json::to_string(&DeviceMsg::SelfId(self_id.clone())).unwrap();
    let caps = self_id.capabilities;
    let mut line: Vec<u8> = Vec::new();
    let mut packet = [0u8; 64];

    write_line(class, &hello).await?;

    loop {
        match select(class.read_packet(&mut packet), Timer::after_millis(1000)).await {
            // A USB packet arrived — accumulate and process complete commands.
            Either::First(res) => {
                let n = res?;
                line.extend_from_slice(&packet[..n]);
                while let Some(raw) = contract::take_line(&mut line) {
                    let reply = match serde_json::from_slice::<Command>(&raw) {
                        Ok(cmd) => match contract::resolve_gpio_set(&caps, &cmd) {
                            Ok(level) => {
                                led.set_level(if level { Level::High } else { Level::Low });
                                DeviceMsg::Ack {
                                    ok: true,
                                    error: None,
                                }
                            }
                            Err(e) => DeviceMsg::Ack {
                                ok: false,
                                error: Some(e),
                            },
                        },
                        Err(_) => DeviceMsg::Ack {
                            ok: false,
                            error: Some(String::from("invalid command json")),
                        },
                    };
                    let out = serde_json::to_string(&reply).unwrap();
                    write_line(class, &out).await?;
                }
                // Cap any unterminated tail: a host streaming without a newline must
                // not grow `line` past the 8 KB heap and halt the device.
                if line.len() > contract::MAX_LINE {
                    line.clear();
                }
            }
            // Timer fired — re-announce so late-connecting hosts see us.
            Either::Second(()) => write_line(class, &hello).await?,
        }
    }
}

/// Write a JSON string + '\n', chunked to the 64-byte CDC packet size.
async fn write_line<'d, D: embassy_usb::driver::Driver<'d>>(
    class: &mut CdcAcmClass<'d, D>,
    text: &str,
) -> Result<(), EndpointError> {
    let mut bytes = Vec::with_capacity(text.len() + 1);
    bytes.extend_from_slice(text.as_bytes());
    bytes.push(b'\n');
    for chunk in bytes.chunks(64) {
        class.write_packet(chunk).await?;
    }
    // If the last chunk was exactly 64 bytes, send a zero-length packet so the
    // host sees the transfer boundary.
    if bytes.len() % 64 == 0 {
        class.write_packet(&[]).await?;
    }
    Ok(())
}
