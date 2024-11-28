use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to 7z dynamic library
    #[arg(long, short = 'l')]
    lib_path: String,

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
            let mut options = bit7z::ExtractOptions::default();
            options.output_dir = output_dir.clone();
            options.password = password.clone();
            let _ = bit7z::extracting(args.lib_path, data, options);
        }
    }

    Ok(())
}
