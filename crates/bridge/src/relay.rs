/// Drain every complete `\n`-terminated line from `buf`, returning them without
/// the trailing newline and leaving any partial (unterminated) tail in `buf`.
/// Framing is delegated to `contract::take_line` so the bridge and firmware split
/// the serial stream identically; here we additionally lossily decode to UTF-8 and
/// drop empty lines (a bare `\n` carries no frame for the browser).
pub fn split_lines(buf: &mut Vec<u8>) -> Vec<String> {
    let mut lines = Vec::new();
    while let Some(line) = contract::take_line(buf) {
        let text = String::from_utf8_lossy(&line).into_owned();
        if !text.is_empty() {
            lines.push(text);
        }
    }
    lines
}
