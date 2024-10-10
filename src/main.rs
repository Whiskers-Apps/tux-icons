use std::path::PathBuf;

use clap::{Parser, Subcommand};
use tux_icons::icon_fetcher::IconFetcher;

#[derive(Parser, Debug)]
struct Cli {
    #[arg(short, long)]
    icon_pack: Option<String>,

    #[arg(short, long)]
    return_target_path: bool,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    FromDesktop { file_path: PathBuf },

    SearchIcon { icon_name: String },
}

fn main() {
    let cli = Cli::parse();
    let mut fetcher = IconFetcher::new().set_return_target_path(cli.return_target_path);

    if let Some(icon_pack) = &cli.icon_pack {
        fetcher = fetcher.set_icon_pack(icon_pack);
    }

    match cli.command {
        Commands::FromDesktop { file_path } => {
            if let Some(icon_path) = fetcher.get_icon_path_from_desktop(file_path) {
                println!("{}", icon_path.into_os_string().into_string().unwrap())
            } else {
                println!("Couldn't find icon path");
            }
        }
        Commands::SearchIcon { icon_name } => {
            if let Some(icon_path) = fetcher.get_icon_path(icon_name) {
                println!("{}", icon_path.into_os_string().into_string().unwrap())
            } else {
                println!("Couldn't find icon path");
            }
        }
    }
}
