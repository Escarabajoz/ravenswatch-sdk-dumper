// ============================================================
// Game SDK Dumper - Rust
// Escanea el proceso del juego y autogenera la carpeta SDK
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
    println!("{}", "  SDK Dumper v1.0 - Generador automático de SDK".yellow().bold());
    println!("{}", "  ═══════════════════════════════════════════════".bright_cyan());
    println!();
}

fn main() {
    print_banner();

    // Pedir nombre del proceso
    print!("{} ", "[?] Nombre del proceso (ej: game.exe):".green().bold());
    io::stdout().flush().unwrap();
    let mut process_name = String::new();
    io::stdin().read_line(&mut process_name).unwrap();
    let process_name = process_name.trim();

    if process_name.is_empty() {
        println!("{}", "[!] Nombre de proceso vacío, saliendo...".red());
        return;
    }

    println!();
    println!("{} {}", "[*] Buscando proceso:".cyan(), process_name.yellow());

    // Abrir proceso
    let proc = match process::GameProcess::open(process_name) {
        Ok(p) => {
            println!(
                "{} PID: {} | Base: 0x{:X} | Size: 0x{:X}",
                "[✓] Proceso encontrado!".green().bold(),
                p.pid,
                p.base_address,
                p.module_size
            );
            p
        }
        Err(e) => {
            println!("{} {}", "[✗] Error:".red().bold(), e);
            println!("{}", "[!] Asegúrate de que el juego esté abierto y ejecuta como administrador.".yellow());
            wait_exit();
            return;
        }
    };

    println!();
    println!("{}", "[*] Iniciando dump del proceso...".cyan().bold());
    println!("{}", "════════════════════════════════════════════".bright_cyan());

    // Leer memoria del módulo
    println!("{}", "[*] Leyendo memoria del módulo...".cyan());
    let memory = match proc.read_module_memory() {
        Ok(m) => {
            println!(
                "{} {} bytes leídos",
                "[✓]".green().bold(),
                m.len()
            );
            m
        }
        Err(e) => {
            println!("{} {}", "[✗] Error leyendo memoria:".red(), e);
            wait_exit();
            return;
        }
    };

    // Crear el dumper y ejecutar
    let mut game_dumper = dumper::GameDumper::new(&proc, &memory);

    println!();
    println!("{}", "[*] Escaneando patrones de bytes...".cyan().bold());
    println!("{}", "────────────────────────────────────────────".bright_cyan());
    game_dumper.scan_all_patterns();

    println!();
    println!("{}", "[*] Escaneando strings del juego...".cyan().bold());
    println!("{}", "────────────────────────────────────────────".bright_cyan());
    game_dumper.scan_all_strings();

    println!();
    println!("{}", "[*] Resolviendo VTables desde constructores...".cyan().bold());
    println!("{}", "────────────────────────────────────────────".bright_cyan());
    game_dumper.resolve_vtables();

    // Generar SDK
    println!();
    println!("{}", "[*] Generando carpeta SDK...".cyan().bold());
    println!("{}", "════════════════════════════════════════════".bright_cyan());

    let output_dir = format!("SDK_dumped_{}", process_name.replace(".exe", ""));
    match sdk_generator::generate_sdk(&output_dir, &game_dumper) {
        Ok(count) => {
            println!();
            println!(
                "{} {} archivos generados en '{}'",
                "[✓] SDK generado exitosamente!".green().bold(),
                count,
                output_dir.yellow()
            );
        }
        Err(e) => {
            println!("{} {}", "[✗] Error generando SDK:".red(), e);
        }
    }

    // Resumen final
    println!();
    println!("{}", "════════════════════════════════════════════".bright_cyan());
    game_dumper.print_summary();
    println!("{}", "════════════════════════════════════════════".bright_cyan());

    wait_exit();
}

fn wait_exit() {
    println!();
    print!("{}", "[*] Presiona Enter para salir...".bright_cyan());
    io::stdout().flush().unwrap();
    let mut _input = String::new();
    io::stdin().read_line(&mut _input).unwrap();
}
