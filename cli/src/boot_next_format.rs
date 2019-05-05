#[derive(Debug)]
pub enum BootNextFormat {
  Hex,
  Dec,
}

impl std::fmt::Display for BootNextFormat {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    match self {
      BootNextFormat::Hex => write!(f, "hex"),
      BootNextFormat::Dec => write!(f, "decimal"),
    }
  }
}

impl std::str::FromStr for BootNextFormat {
  type Err = &'static str;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "h" | "hex" | "hexadecimal" => Ok(BootNextFormat::Hex),
      "d" | "dec" | "decimal" => Ok(BootNextFormat::Dec),
      _ => Err("no match"),
    }
  }
}

impl BootNextFormat {
  pub fn radix(&self) -> u32 {
    match self {
      BootNextFormat::Hex => 16,
      BootNextFormat::Dec => 10,
    }
  }

  pub fn variants() -> &'static [&'static str] {
    &["hexadecimal", "hex", "h", "decimal", "dec", "d"]
  }

  pub fn parse_boot_next(
    &self,
    matches: &clap::ArgMatches,
    arg_name: &str,
  ) -> Result<u16, clap::Error> {
    if let Some(value) = matches.value_of(arg_name) {
      match u16::from_str_radix(value, self.radix()) {
        Ok(v) => Ok(v),
        Err(_) => Err(
          clap::Error::value_validation_auto(format!(
            "The argument '{}' isn't a valid {} value",
            value, &self
          ))
          .into(),
        ),
      }
    } else {
      Err(::clap::Error::argument_not_found_auto(arg_name))
    }
  }
}
