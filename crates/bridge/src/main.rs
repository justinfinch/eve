//! Host bridge: browser WebSocket <-> Pico USB serial (newline-framed JSON).
//! Usage: bridge [--port <serial>] [--ws <addr>] [--baud <n>]

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let mut port = default_port();
    let mut ws = "127.0.0.1:8766".to_string();
    let mut baud = 115_200u32;

    let mut args = std::env::args().skip(1);
    while let Some(a) = args.next() {
        match a.as_str() {
            "--port" => port = args.next().expect("--port needs a value"),
            "--ws" => ws = args.next().expect("--ws needs a value"),
            "--baud" => baud = args.next().expect("--baud needs a value").parse().unwrap(),
            other => eprintln!("ignoring unknown arg {other}"),
        }
    }

    // Race the relay against Ctrl-C so SIGINT is a clean exit(0) — otherwise cargo/just
    // report the child as "interrupted by SIGINT".
    tokio::select! {
        r = bridge::run(&ws, &port, baud) => r,
        _ = tokio::signal::ctrl_c() => {
            eprintln!("\nbridge: shutting down");
            Ok(())
        }
    }
}

/// Best-effort auto-detect: first serial port whose USB vendor id is Raspberry Pi (0x2E8A).
fn default_port() -> String {
    if let Ok(ports) = tokio_serial::available_ports() {
        for p in &ports {
            if let tokio_serial::SerialPortType::UsbPort(info) = &p.port_type {
                if info.vid == 0x2E8A {
                    return p.port_name.clone();
                }
            }
        }
        if let Some(p) = ports.first() {
            return p.port_name.clone();
        }
    }
    "/dev/tty.usbmodem0001".to_string()
}
