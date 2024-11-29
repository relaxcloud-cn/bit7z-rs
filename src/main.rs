#[cfg(feature = "cli")]
use clap::{Parser, Subcommand};
use yansi::Color::{Green, Red};
use yansi::Paint;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to 7z dynamic library
    #[arg(long, short = 'l')]
    lib_path: Option<String>,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Extract archive with full paths
    X {
        /// Archive file path
        archive_path: String,

        /// Output directory (optional)
        #[arg(short = 'o')]
        output_dir: Option<String>,

        /// Password (optional)
        #[arg(short = 'p')]
        password: Option<String>,
    },
}

#[cfg(feature = "cli")]
fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    match &args.command {
        Commands::X {
            archive_path,
            output_dir,
            password,
        } => {
            let data = bit7z::read_binary_file(archive_path)?;
            let mut options = bit7z::ExtractingOptions::default();
            options.output_dir = output_dir.clone();
            options.password = password.clone();
            options.lib_path = args.lib_path.clone();
            match bit7z::extracting(data, options) {
                Ok(success_msg) => println!("{}: {}", "Success".paint(Green), success_msg),
                Err(e) => println!("{}: {}", "Error".paint(Red), e),
            }
        }
    }

    Ok(())
}
