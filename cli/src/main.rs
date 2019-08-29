#![warn(rust_2018_idioms)]

#[macro_use]
extern crate clap;

use clap::{App, AppSettings, Arg, SubCommand};

use failure::Error;
use rebootinto_core as core;

type Result<T> = std::result::Result<T, Error>;

mod boot_next_format;
use boot_next_format::BootNextFormat;

fn main() {
    if let Err(err) = run() {
        match std::env::var("PANIC_ON_ERROR") {
            Ok(ref val) if val == "true" => panic!("Error: {}", err),
            _ => {}
        }

        eprintln!("Error: {}", err);
        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    let mut backend = core::Backend::init()?;

    let default_boot_next_format: &str = &format!("{}", BootNextFormat::Hex);

    let matches = App::new(env!("CARGO_PKG_NAME"))
        .version(crate_version!())
        .about("Reboot into the specified boot option")
        .setting(AppSettings::ArgRequiredElseHelp)
        .subcommand(
            SubCommand::with_name("list")
                .aliases(&["ls", "dir"])
                .about("Prints possible boot options"),
        )
        .subcommand(
            SubCommand::with_name("reboot")
                .about("Reboot into the specified boot option")
                .arg(
                    Arg::with_name("boot_next")
                        .required(true)
                        .index(1)
                        .help("The value to set BootNext to"),
                )
                .arg(
                    Arg::with_name("format")
                        .short("f")
                        .possible_values(&BootNextFormat::variants())
                        .default_value(default_boot_next_format)
                        .help("The format of the value"),
                ),
        )
        .get_matches();

    match matches.subcommand() {
        ("list", _) => {
            for load_option_result in backend.load_options() {
                let load_option = load_option_result?;
                println!("{:04X} {}", load_option.number, load_option.description);
            }
            Ok(())
        }
        ("reboot", Some(submatches)) => {
            let format = value_t_or_exit!(submatches, "format", BootNextFormat);
            let boot_next: u16 = format
                .parse_boot_next(submatches, "boot_next")
                .unwrap_or_else(|e| e.exit());

            let _ = backend.reboot_into(boot_next)?;

            println!("{:04X}", boot_next);
            Ok(())
        }
        _ => unreachable!(),
    }
}
