#[cfg(feature = "cli")]
use clap::{Parser, Subcommand};
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::{fs, io};
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
            let mut options = ExtractingOptions::default();
            options.path = archive_path.clone();
            options.output_dir = output_dir.clone();
            options.password = password.clone();
            options.lib_path = args.lib_path.clone();
            match extracting(options) {
                Ok(success_msg) => println!("{}: {}", "Success".paint(Green), success_msg),
                Err(e) => println!("{}: {}", "Error".paint(Red), e),
            }
        }
    }

    Ok(())
}

#[derive(Default, Debug)]
pub struct ExtractingOptions {
    /// path
    pub path: String,
    /// Output directory path
    pub output_dir: Option<String>,
    /// Password for encrypted archives
    pub password: Option<String>,
    /// Dynamic lib path
    pub lib_path: Option<String>,
}

fn extracting(options: ExtractingOptions) -> anyhow::Result<String> {
    let data = read_binary_file(options.path)?;
    let output_dir = options
        .output_dir
        .ok_or_else(|| anyhow::anyhow!("Output directory not specified"))?;
    let output_path = Path::new(&output_dir);

    if !output_path.exists() {
        fs::create_dir_all(output_path)?;
    } else {
        if output_path.read_dir()?.next().is_some() {
            anyhow::bail!("Output directory is not empty");
        }
    }
    let result_map = bit7z::extracting(data, options.lib_path, options.password)?;
    for (filename, content) in &result_map {
        let file_path = output_path.join(filename);

        if let Some(parent) = file_path.parent() {
            if !parent.exists() {
                fs::create_dir_all(parent)?;
            }
        }

        let mut file = File::create(file_path)?;
        file.write_all(content)?;
        println!("{} - {} bytes written", filename.paint(yansi::Color::Cyan), content.len());
    }

    Ok(format!("Files extracted to: {}", output_dir))
}

pub fn read_binary_file<P: AsRef<Path>>(path: P) -> io::Result<Vec<u8>> {
    std::fs::read(path)
}
