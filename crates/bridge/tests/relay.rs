use bridge::relay::split_lines;

#[test]
fn split_lines_drains_complete_lines_and_keeps_the_tail() {
    let mut buf = b"one\ntwo\npar".to_vec();
    let lines = split_lines(&mut buf);
    assert_eq!(lines, vec!["one".to_string(), "two".to_string()]);
    assert_eq!(buf, b"par".to_vec()); // partial line stays buffered
}

#[test]
fn split_lines_returns_empty_when_no_newline_yet() {
    let mut buf = b"partial".to_vec();
    let lines = split_lines(&mut buf);
    assert!(lines.is_empty());
    assert_eq!(buf, b"partial".to_vec());
}
