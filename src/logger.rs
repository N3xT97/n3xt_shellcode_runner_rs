/// 구분선과 섹션 타이틀 출력
pub fn title(title: &str) {
    const WIDTH: usize = 60;
    let pad = WIDTH.saturating_sub(title.len() + 3);
    println!("── {} {}", title, "─".repeat(pad));
}

/// key-value 라인 출력 (정렬)
pub fn key_value(key: &str, value: impl AsRef<str>) {
    println!("  {:<28} {}", key, value.as_ref());
}

/// 로그 레벨별 간단 출력
pub fn step(msg: &str) {
    println!("[>] {msg}");
}
pub fn ok(msg: &str) {
    println!("[+] {msg}");
}
pub fn info(msg: &str) {
    println!("[i] {msg}");
}
pub fn warn(msg: &str) {
    println!("[!] {msg}");
}
