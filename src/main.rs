mod error;
mod logger;
mod runner;
mod shellcode;
mod thread;
mod virtual_memory;

use crate::{
    error::ShellcodeRunnerError,
    logger::{info, key_value, ok, step, title},
    runner::Runner,
};
use clap::{Parser, ValueHint};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(
    name = "n3xt_shellcode_runner_cli_rs",
    version,
    about = "N3xT Shellcode Runner CLI"
)]
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
    println!(
        r#"
+==============================================================+
|                                                              |
|           _____     _     ____   ____                        |
|     _ __ |___ /_  _| |_  / ___| / ___|  _ __ _   _ _ __      |
|    | '_ \  |_ \ \/ / __| \___ \| |     | '__| | | | '_ \     |
|    | | | |___) >  <| |_   ___) | |___  | |  | |_| | | | |    |
|    |_| |_|____/_/\_\\__| |____/ \____| |_|   \__,_|_| |_|    |
|                                                              |
+==============================================================+"#
    );
    let arch_bits = std::mem::size_of::<usize>() * 8;
    info(&format!("N3xT Shellcode Runner CLI {}-bit", arch_bits));
}

fn print_parameter(cli: &Cli) {
    println!();
    title("Input Parameters");
    step("Parsing CLI arguments…");
    ok("CLI arguments parsed.");

    key_value("Shellcode File Path", cli.file_path.display().to_string());

    let start_offset = cli.start_offset;
    key_value(
        "Shellcode Start Offset",
        format!("{:#X} ({})", start_offset, start_offset),
    );

    let fmt_memory_size = cli
        .memory_size
        .map(|size| format!("{:#X} ({})", size, size))
        .unwrap_or_else(|| "None".to_string());

    key_value("Shellcode Memory Size", fmt_memory_size);
}

fn app(cli: Cli) -> Result<(), ShellcodeRunnerError> {
    let runner = Runner::from_file(cli.file_path, cli.start_offset, cli.memory_size)?;
    runner.run()
}

fn main() {
    let cli = Cli::parse();
    print_banner();
    print_parameter(&cli);
    if let Err(e) = app(cli) {
        println!("[-] Error::{}", e);
        std::process::exit(1);
    }
}
