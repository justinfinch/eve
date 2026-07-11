/// Drain every complete `\n`-terminated line from `buf`, returning them without
/// the trailing newline and leaving any partial (unterminated) tail in `buf`.
pub fn split_lines(buf: &mut Vec<u8>) -> Vec<String> {
    let mut lines = Vec::new();
    while let Some(pos) = buf.iter().position(|&b| b == b'\n') {
        let line: Vec<u8> = buf.drain(..=pos).collect();
        // `line` includes the trailing '\n'; trim it (and a stray '\r').
        let end = line.len().saturating_sub(1);
        let mut text = String::from_utf8_lossy(&line[..end]).into_owned();
        if text.ends_with('\r') {
            text.pop();
        }
        if !text.is_empty() {
            lines.push(text);
        }
    }
    lines
}
