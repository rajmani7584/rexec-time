use std::{
    env,
    process::Command,
    time::{Duration, Instant},
};

use colored::Colorize;

enum TimeUnit {
    Seconds,
    Miliseconds,
    Microseconds,
    Nanoseconds,
}

impl TimeUnit {
    fn to_string(&self, duration: Duration) -> String {
        match self {
            TimeUnit::Seconds => format!("{:.6} s", duration.as_secs_f64()),
            TimeUnit::Miliseconds => format!("{:.3} ms", duration.as_millis()),
            TimeUnit::Microseconds => format!("{:.6} μs", duration.as_micros()),
            TimeUnit::Nanoseconds => format!("{:.6} ns", duration.as_nanos()),
        }
    }
}

fn main() {
    let mut time_unit = TimeUnit::Miliseconds;

    let cmds = env::args().skip(1).collect::<Vec<String>>();

    let mut args: Vec<String> = Vec::new();
    let mut cmd = String::new();
    let mut cmd_args: Vec<String> = Vec::new();

    for (idx, arg) in cmds.iter().enumerate() {
        match arg.as_str() {
            "--help" | "-h" => {
                print_help();
                return;
            }
            "--version" | "-v" => {
                print_version();
                return;
            }
            "-s" | "--seconds" => {
                time_unit = TimeUnit::Seconds;
            }
            "-ms" | "--miliseconds" => {
                time_unit = TimeUnit::Miliseconds;
            }
            "-us" | "--microseconds" => {
                time_unit = TimeUnit::Microseconds;
            }
            "-ns" | "--nanoseconds" => {
                time_unit = TimeUnit::Nanoseconds;
            }
            _ => {
                if !arg.starts_with("-") {
                    cmd = arg.to_string();
                    for a in cmds.iter().skip(idx + 1) {
                        cmd_args.push(a.to_string());
                    }
                    break;
                }
                args.push(arg.to_string());
            }
        }
    }

    if cmd.is_empty() {
        print_uptime();
        return;
    }

    let mut cmd_exec = Command::new(&cmd);
    for arg in cmd_args {
        cmd_exec.arg(arg);
    }

    let start = Instant::now();
    let status = match cmd_exec.spawn() {
        Ok(mut child) => match child.wait() {
            Ok(status) => status,
            Err(e) => {
                eprintln!("{}{}", "Failed to wait for command: ".red().bold(), e);
                return;
            }
        },
        Err(e) => {
            eprintln!("{}[\"{}\"]: {}", "Error ".red().bold(), cmd.green(), e);
            return;
        }
    };
    let end = Instant::now();

    if !status.success() {
        eprintln!(
            "{}{}{}",
            "Command execution failed. exit code: ".red().bold(),
            status.code().unwrap_or(1),
            "\n"
        );
        return;
    }

    println!(
        "\n{} {}",
        "Execution time:".green().bold(),
        time_unit.to_string(end - start)
    );
}

fn print_uptime() {
    let is_windows = cfg!(target_os = "windows");

    println!("{}", "System Uptime:".blue().bold());

    let mut cmd = if is_windows {
        Command::new("systeminfo | find \"\"\"Boot Time\"\"\"")
    } else {
        Command::new("uptime")
    };

    if let Ok(mut child) = cmd.spawn() {
        if let Ok(status) = child.wait() {
            if status.success() {
                return;
            }
        }
    }
}

fn print_version() {
    println!(
        "rexec-time\nVersion: 0.1.0\n\n\tby {} {}",
        "Rajmani7584".green().bold(),
        "@Github".blue().bold()
    );
}

fn print_help() {
    println!("Usage: rexec-time [args] [command] [command-args]");
    println!("Options:");
    println!("  -h, --help\t\t Show this help message and exit");
    println!("  -v, --version\t\t Show program version and exit");
    println!("  -s, --seconds\t\t Display execution time in seconds");
    println!("  -ms, --miliseconds\t Display execution time in miliseconds (default)");
    println!("  -us, --microseconds\t Display execution time in microseconds");
    println!("  -ns, --nanoseconds\t Display execution time in nanoseconds");
    println!("This program executes commands and displays execution time.");
}
