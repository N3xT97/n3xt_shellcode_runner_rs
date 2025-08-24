use std::time::{SystemTime, UNIX_EPOCH};

#[allow(dead_code)]
pub enum Level {
    Info,
    Step,
    Ok,
    Warn,
    Err,
}

pub struct Logger {
    use_color: bool,
}

impl Logger {
    pub fn new() -> Self {
        Self { use_color: true }
    }

    fn ts() -> String {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default();
        let secs = now.as_secs();
        let ms = now.subsec_millis();
        let (h, m, s) = ((secs / 3600) % 24, (secs / 60) % 60, secs % 60);
        format!("{:02}:{:02}:{:02}.{:03}", h, m, s, ms)
    }

    fn tag_color(lv: &Level) -> (&'static str, &'static str) {
        match lv {
            Level::Info => ("[i]", "\x1b[37m"),
            Level::Step => ("[>]", "\x1b[36m"),
            Level::Ok => ("[+]", "\x1b[32m"),
            Level::Warn => ("[!]", "\x1b[33m"),
            Level::Err => ("[-]", "\x1b[31m"),
        }
    }

    pub fn println(&self, lv: Level, msg: impl AsRef<str>) {
        let (tag, color) = Self::tag_color(&lv);
        if self.use_color {
            println!(
                "{} {} {}{}{}",
                tag,
                Self::ts(),
                color,
                msg.as_ref(),
                "\x1b[0m"
            );
        } else {
            println!("{} {} {}", tag, Self::ts(), msg.as_ref());
        }
    }

    pub fn hr(&self, title: &str) {
        const WIDTH: usize = 100;
        let line_len = WIDTH.saturating_sub(title.len() + 3);
        let line = "─".repeat(line_len);
        println!("── {} {}", title, line);
    }
}

pub fn format_size_option(opt: Option<usize>) -> String {
    match opt {
        Some(size) => format!("{:#X} ({})", size, size),
        None => "None".to_string(),
    }
}

pub fn format_size(n: usize) -> String {
    format!("{:<12} ({:#X})", n, n)
}

pub fn preview_hex_bytes(bytes: &[u8], max: usize) -> String {
    use std::fmt::Write;
    let mut s = String::new();
    for (i, b) in bytes.iter().take(max).enumerate() {
        if i > 0 {
            s.push(' ');
        }
        write!(s, "{:02X}", b).unwrap();
    }
    s
}
