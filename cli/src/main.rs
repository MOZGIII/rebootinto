//! A clap-based CLI app for rebootinto.

use clap::{Parser, Subcommand};

use rebootinto_core as core;

/// The command line app invocation.
#[derive(Parser)]
#[command(version)]
#[command(about = "Reboot into the specified boot option.", long_about = None)]
struct Invocation {
    /// The command that is invoked.
    #[command(subcommand)]
    command: Command,
}

/// CLI commands.
#[derive(Subcommand)]
enum Command {
    /// Prints possible boot options.
    List,
    /// Reboot into the specified boot option.
    Reboot {
        /// The value to set `BootNext` to.
        load_option: String,
        /// The format to expect the load option value in.
        #[arg(short, long, env = "LOAD_OPTION_FORMAT", value_enum, default_value_t = LoadOptionFormat::Hex)]
        format: LoadOptionFormat,
    },
}

fn main() {
    if let Err(err) = run() {
        match std::env::var("PANIC_ON_ERROR") {
            Ok(ref val) if val == "true" => panic!("Error: {err}"),
            _ => {}
        }

        eprintln!("Error: {err}");
        std::process::exit(1);
    }
}

/// Run the app and return the error.
fn run() -> Result<(), anyhow::Error> {
    let mut backend = core::Backend::init()?;

    let invocation = Invocation::parse();

    match invocation.command {
        Command::List => {
            for load_option_result in backend.load_options()? {
                let load_option = load_option_result?;
                println!("{:04X} {}", load_option.number, load_option.description);
            }
            Ok(())
        }
        Command::Reboot {
            load_option,
            format,
        } => {
            let load_option: u16 = format.parse_boot_next(&load_option)?;

            backend.reboot_into(load_option)?;

            println!("{load_option:04X}");
            Ok(())
        }
    }
}

/// The format of the `BootNext` value.
#[derive(Debug, Clone, Copy, PartialEq, Eq, clap::ValueEnum)]
pub enum LoadOptionFormat {
    /// The value is a hex number.
    #[value(alias("h"))]
    #[value(alias("hexadecimal"))]
    Hex,
    /// The value is a decimal number.
    #[value(alias("d"))]
    #[value(alias("decimal"))]
    Dec,
}

impl LoadOptionFormat {
    /// Radix of the underlying numeric format.
    pub fn radix(&self) -> u32 {
        match self {
            LoadOptionFormat::Hex => 16,
            LoadOptionFormat::Dec => 10,
        }
    }

    /// Parse the `BootNext` value using the format.
    pub fn parse_boot_next(&self, value: &str) -> Result<u16, anyhow::Error> {
        let val = u16::from_str_radix(value, self.radix()).map_err(|err| {
            anyhow::format_err!("unable to parse the boot option in {self} format: {err}")
        })?;
        Ok(val)
    }
}

impl std::fmt::Display for LoadOptionFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Dec => f.write_str("decimal"),
            Self::Hex => f.write_str("hex"),
        }
    }
}
