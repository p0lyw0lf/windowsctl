mod constants;
mod efi_editor;
mod privilege_elevator;
mod traits;

use clap::{ArgEnum, Parser, Subcommand};
use std::ffi::CStr;
use windows::core::Result as WinResult;

use crate::traits::ToPCSTR;

#[derive(Parser)]
#[clap(author, version, about, long_about = None, propagate_version = true)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Get {
        #[clap(arg_enum)]
        var: Vars,
    },
    Set {
        #[clap(arg_enum)]
        var: Vars,
        val: String,
    },
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ArgEnum)]
enum Vars {
    OneShot,
    Default,
}

impl Vars {
    fn to_var_str(self) -> &'static CStr {
        match self {
            Vars::OneShot => &constants::ONESHOT_VAR_NAME,
            Vars::Default => &constants::DEFAULT_VAR_NAME,
        }
    }
}

fn get(var: Vars) -> WinResult<()> {
    let data = efi_editor::read_efivar(
        var.to_var_str().to_pcstr(),
        constants::SYSTEMD_LOADER_VENDOR_GUID.to_pcstr(),
        1024,
    )?;
    let data = String::from(data);

    println!(
        "{} is currently set to: {}",
        var.to_var_str().to_str().unwrap(),
        data
    );

    Ok(())
}

fn set(var: Vars, val: &str) -> WinResult<()> {
    efi_editor::write_efivar(
        var.to_var_str().to_pcstr(),
        constants::SYSTEMD_LOADER_VENDOR_GUID.to_pcstr(),
        String::from(val),
    )?;

    println!("Set {} to: {}", var.to_var_str().to_str().unwrap(), val);

    Ok(())
}

fn main() -> WinResult<()> {
    let cli = Cli::parse();

    privilege_elevator::elevate_thread_to_system()?;

    match &cli.command {
        Commands::Get { var } => {
            get(*var)?;
        }
        Commands::Set { var, val } => {
            set(*var, val)?;
        }
    }

    Ok(())
}
