use crate::models::commands_traits::CommandsActions;

use clap::{Args, Subcommand};
use anyhow::Error;
use sysinfo::{System, Networks};

#[derive(Args, Debug)]
pub struct SystemCommands {
    #[command(subcommand)]
    command: SystemCommandsOptions,
}

#[derive(Subcommand, Debug)]
pub enum SystemCommandsOptions {
    /// Shows system configs options, if no flag are set, it will show all the system information
    Configs {
        /// Shows system related information
        #[arg(short, long = "sys")]
        system: bool,

        /// Show RAM related information
        #[arg(short, long = "mem")]
        memory: bool,

        /// Shows display processes ID, name and disk usage
        #[arg(short, long = "proc")]
        process: bool,

        /// Shows network interfaces name, total data received and total data transmitted
        #[arg(short, long)]
        network: bool,
    }
}

impl CommandsActions for SystemCommands {
    fn run(&self) -> Result<(), Error> {
        match &self.command {
            SystemCommandsOptions::Configs {
                system, memory,
                process, network
            } => {
                let mut sys: System = System::new_all();
                sys.refresh_all();

                fn get_system_info() {
                    match System::name() {
                        Some(name) => {
                            println!("System name: {}", name);
                        }
                        None => {
                            println!("WARNING: System name not found");
                        }
                    }
                    match System::kernel_version() {
                        Some(version) => {
                            println!("System kernel version: {}", version);
                        }
                        None => {
                            println!("WARNING: System kernel version not found");
                        }
                    }
                    match System::os_version() {
                        Some(version) => {
                            println!("System OS version: {}", version);
                        }
                        None => {
                            println!("WARNING: System OS version not found");
                        }
                    }
                    match System::host_name() {
                        Some(name) => {
                            println!("System host name: {}", name);
                        }
                        None => {
                            println!("WARNING: System host name not found");
                        }
                    }
                }
                fn get_memory_info(sys: &System) {
                    let total_memory: u64 = sys.total_memory();
                    let total_gib: f64 = total_memory as f64 / 1024.0 / 1024.0 / 1024.0;

                    let used_memory: u64 = sys.used_memory();
                    let used_gib: f64 = sys.used_memory() as f64 / 1024.0 / 1024.0 / 1024.0;

                    println!("Total RAM: [{} bytes] | [{:.2} gigabytes]", total_memory, total_gib);
                    println!("Used RAM: [{} bytes] | [{:.2} gigabytes]", used_memory, used_gib);
                }
                fn get_process_info(sys: &System) {
                    let processes = sys.processes();

                    if processes.is_empty() {
                        println!("WARNING: System processes not found");
                    }

                    for (pid, process) in processes {
                        println!("[PID: {pid}] | Process Name: {:?} | Disk Usage: {:?}", process.name(), process.disk_usage());
                    }
                }
                fn get_network_info() {
                    let networks: Networks = Networks::new_with_refreshed_list();
                    for (interface_name, data) in &networks {
                        println!(
                            "{interface_name}: {} bytes (received) / {} bytes (sent)",
                            data.total_received(),
                            data.total_transmitted(),
                        );
                    }
                }

                if !system.clone() && !memory.clone() &&
                   !process.clone() && !network.clone() {
                    get_system_info();
                    get_memory_info(&sys);
                    get_process_info(&sys);
                    get_network_info();
                }

                if system.clone() {
                    get_system_info();
                }
                if memory.clone() {
                    get_memory_info(&sys);
                }
                if process.clone() {
                    get_process_info(&sys);
                }
                if network.clone() {
                    get_network_info();
                }
            },
        }

        Ok(())
    }
}