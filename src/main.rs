use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to 7z dynamic library
    #[arg(long, short = 'l')]
    lib_path: String,

    /// Path to archive file
    #[arg(long, short = 'p')]
    archive_path: String,
}

#[cfg(feature = "cli")]
fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let data = bit7z::read_binary_file(args.archive_path)?;
    let _ = bit7z::extracting(args.lib_path, data);
    Ok(())
}
