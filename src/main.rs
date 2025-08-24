mod error;
mod logger;
mod runner;
mod shellcode;
mod thread;
mod virtual_memory;

use crate::{error::ShellcodeRunnerError, logger::format_size_option, runner::Runner};
use clap::{Parser, ValueHint};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "n3xt_shellcode_runner", version, about = "Shellcode Runner")]
struct Cli {
    #[arg(short = 'f', long = "file-path", value_hint = ValueHint::FilePath)]
    file_path: PathBuf,
    #[arg(short = 's', long = "start-offset",value_parser = parse_usize)]
    start_offset: usize,
    #[arg(short = 'm', long = "mem-size", value_parser = parse_usize)]
    memory_size: Option<usize>,
}

pub fn parse_usize(s: &str) -> Result<usize, String> {
    if let Some(hex) = s.strip_prefix("0x").or_else(|| s.strip_prefix("0X")) {
        usize::from_str_radix(hex, 16).map_err(|e| e.to_string())
    } else {
        s.parse::<usize>().map_err(|e| e.to_string())
    }
}

fn print_banner() {
    const WIDTH: usize = 102;
    println!("{}", "─".repeat(WIDTH));
    println!(
        "[*] N3xT Shellcode Runner {}",
        std::mem::size_of::<usize>() * 8
    );
    println!("{}", "─".repeat(WIDTH));
}

fn app() -> Result<(), ShellcodeRunnerError> {
    let cli = Cli::parse();

    println!("[>] Input Parameters");
    println!(
        "  {:<28} {}",
        "Shellcode File Path",
        cli.file_path.display()
    );
    println!(
        "  {:<28} {:#X} ({})",
        "Shellcode Start Offset", cli.start_offset, cli.start_offset
    );
    println!(
        "  {:<28} {}",
        "Shellcode Memory Size",
        format_size_option(cli.memory_size)
    );

    let runner = Runner::from_file(cli.file_path, cli.start_offset, cli.memory_size)?;
    runner.run()
}

fn main() {
    print_banner();
    if let Err(e) = app() {
        eprintln!("\x1b[31m[-] Error: {}\x1b[0m", e);
        std::process::exit(1);
    }
}
