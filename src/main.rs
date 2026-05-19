// ============================================================
// Game SDK Dumper - Rust
// Scans the game process and auto-generates the SDK folder
// ============================================================

mod process;
mod scanner;
mod dumper;
mod sdk_generator;

use colored::*;
use std::io::{self, Write};

fn print_banner() {
    println!("{}", r#"
  ██████╗  █████╗ ███╗   ███╗███████╗    ██████╗ ██╗   ██╗███╗   ███╗██████╗ ███████╗██████╗
 ██╔════╝ ██╔══██╗████╗ ████║██╔════╝    ██╔══██╗██║   ██║████╗ ████║██╔══██╗██╔════╝██╔══██╗
 ██║  ███╗███████║██╔████╔██║█████╗      ██║  ██║██║   ██║██╔████╔██║██████╔╝█████╗  ██████╔╝
 ██║   ██║██╔══██║██║╚██╔╝██║██╔══╝      ██║  ██║██║   ██║██║╚██╔╝██║██╔═══╝ ██╔══╝  ██╔══██╗
 ╚██████╔╝██║  ██║██║ ╚═╝ ██║███████╗    ██████╔╝╚██████╔╝██║ ╚═╝ ██║██║     ███████╗██║  ██╗
  ╚═════╝ ╚═╝  ╚═╝╚═╝     ╚═╝╚══════╝    ╚═════╝  ╚═════╝ ╚═╝     ╚═╝╚═╝     ╚══════╝╚═╝  ╚═╝
    "#.cyan());
    println!("{}", "  SDK Dumper v1.0 - Automatic SDK Generator".yellow().bold());
    println!("{}", "  By El Escarabajo - github.com/Escarabajoz".bright_cyan());
    println!("{}", "  ═════════════════════════════════════════════".bright_cyan());
    println!();
}

fn main() {
    print_banner();

    print!("{} ", "[?] Process name (e.g. Ravenswatch.exe):".green().bold());
    io::stdout().flush().unwrap();
    let mut process_name = String::new();
    io::stdin().read_line(&mut process_name).unwrap();
    let process_name = process_name.trim();

    if process_name.is_empty() {
        println!("{}", "[!] Empty process name, exiting...".red());
        return;
    }

    println!();
    println!("{} {}", "[*] Looking for process:".cyan(), process_name.yellow());

    let proc = match process::GameProcess::open(process_name) {
        Ok(p) => {
            println!(
                "{} PID: {} | Base: 0x{:X} | Size: 0x{:X}",
                "[✓] Process found!".green().bold(),
                p.pid,
                p.base_address,
                p.module_size
            );
            p
        }
        Err(e) => {
            println!("{} {}", "[✗] Error:".red().bold(), e);
            println!("{}", "[!] Make sure the game is running and execute as Administrator.".yellow());
            wait_exit();
            return;
        }
    };

    println!();
    println!("{}", "[*] Starting process dump...".cyan().bold());
    println!("{}", "═".repeat(60).bright_cyan());

    println!("{}", "[*] Reading module memory...".cyan());
    let memory = match proc.read_module_memory() {
        Ok(m) => {
            println!("{} {} bytes read", "[✓]".green().bold(), m.len());
            m
        }
        Err(e) => {
            println!("{} {}", "[✗] Error reading memory:".red(), e);
            wait_exit();
            return;
        }
    };

    let mut game_dumper = dumper::GameDumper::new(&proc, &memory);

    println!();
    println!("{}", "[*] Scanning byte patterns...".cyan().bold());
    game_dumper.scan_all_patterns();

    println!();
    println!("{}", "[*] Scanning game strings...".cyan().bold());
    game_dumper.scan_all_strings();

    println!();
    println!("{}", "[*] Resolving VTables from constructors...".cyan().bold());
    game_dumper.resolve_vtables();

    println!();
    println!("{}", "[*] Generating SDK folder...".cyan().bold());
    let output_dir = format!("SDK_dumped_{}", process_name.replace(".exe", ""));
    match sdk_generator::generate_sdk(&output_dir, &game_dumper) {
        Ok(count) => {
            println!("{} {} files generated in '{}'",
                "[✓] SDK generated successfully!".green().bold(),
                count, output_dir.yellow());
        }
        Err(e) => println!("{} {}", "[✗] Error:".red(), e),
    }

    println!();
    game_dumper.print_summary();
    wait_exit();
}

fn wait_exit() {
    println!();
    print!("{}", "[*] Press Enter to exit...".bright_cyan());
    io::stdout().flush().unwrap();
    let mut _input = String::new();
    io::stdin().read_line(&mut _input).unwrap();
}
