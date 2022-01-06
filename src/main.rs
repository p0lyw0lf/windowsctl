mod constants;
mod efi_editor;
mod privilege_elevator;

use clap::{AppSettings, ArgEnum, Parser, Subcommand};
use windows::core::Result as WinResult;

#[derive(Parser)]
#[clap(author, version, about)]
#[clap(global_setting(AppSettings::PropagateVersion))]
#[clap(global_setting(AppSettings::UseLongFormatForHelpSubcommand))]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Get {
        #[clap(arg_enum)]
        var: Vars
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
    fn to_var_str(self) -> &'static str {
        match self {
            Vars::OneShot => constants::ONESHOT_VAR_NAME,
            Vars::Default => constants::DEFAULT_VAR_NAME,
        }
    }
}

fn get(var: Vars) -> WinResult<()> { 
    let data = efi_editor::read_efivar(
        var.to_var_str(),
        constants::SYSTEMD_LOADER_VENDOR_GUID,
        512,
    )?;
    let data = String::from(data);

    println!("{} is currently set to: {}", var.to_var_str(), data);

    Ok(())
}

fn set(var: Vars, val: &str) -> WinResult<()> { 
    efi_editor::write_efivar(
        var.to_var_str(),
        constants::SYSTEMD_LOADER_VENDOR_GUID,
        String::from(val),
    )?;

    println!("Set {} to: {}", var.to_var_str(), val);

    Ok(())
}


fn main() -> WinResult<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Get { var } => {
            privilege_elevator::elevate_privileges()?;
            get(*var)?;
        }
        Commands::Set { var, val } => {
            privilege_elevator::elevate_privileges()?;
            set(*var, val)?;
        }
    }

    Ok(())
}
