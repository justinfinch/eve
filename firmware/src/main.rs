#![no_std]
#![no_main]

use contract::SelfId;
use embassy_executor::Spawner;
use embassy_rp::gpio::{Level, Output};
use embassy_time::Timer;
use embedded_alloc::LlffHeap as Heap;
use panic_halt as _;

#[global_allocator]
static HEAP: Heap = Heap::empty();

// RP2350 boot image definition. On power-up the chip's boot ROM scans the first
// block of flash for this signature to decide our program is a bootable image.
// It lands in the `.start_block` section that firmware/memory.x reserves; without
// it, the board would enumerate as a USB drive but never actually run our code.
#[link_section = ".start_block"]
#[used]
pub static IMAGE_DEF: embassy_rp::block::ImageDef = embassy_rp::block::ImageDef::secure_exe();

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    // Initialize the heap so the contract's alloc-backed types link/run.
    {
        use core::mem::MaybeUninit;
        const HEAP_SIZE: usize = 4096;
        static mut HEAP_MEM: [MaybeUninit<u8>; HEAP_SIZE] = [MaybeUninit::uninit(); HEAP_SIZE];
        unsafe { HEAP.init(core::ptr::addr_of_mut!(HEAP_MEM) as usize, HEAP_SIZE) }
    }

    let p = embassy_rp::init(Default::default());

    // Prove the firmware shares the single-source-of-truth contract (FR-6).
    let _id = SelfId {
        id: "mol-001".into(),
        name: "Mini-Molecule".into(),
        fw_version: "0.1.0".into(),
        capabilities: Default::default(),
    };

    // Rung 0 — external LED on GP15. `Output` configures the pad as a push-pull
    // output; driving it high sources ~3.3V through the current-limiting resistor,
    // driving it low sinks it to 0V. The 250ms delays give a visible 2Hz blink.
    let mut led = Output::new(p.PIN_15, Level::Low);
    loop {
        led.set_high();
        Timer::after_millis(100).await;
        led.set_low();
        Timer::after_millis(100).await;
    }
}
