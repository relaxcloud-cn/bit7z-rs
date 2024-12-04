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
    L {
        /// Archive file path
        archive_path: String,
    },
}

#[cfg(feature = "cli")]
fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    match &args.command {
        Commands::X { archive_path, output_dir, password } => {
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
        Commands::L { archive_path } => match listing(args.lib_path.clone(), archive_path.clone()) {
            Ok(()) => println!("{}", "Success".paint(Green)),
            Err(e) => println!("{}: {}", "Error".paint(Red), e),
        },
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
    let output_dir = options.output_dir.ok_or_else(|| anyhow::anyhow!("Output directory not specified"))?;
    let output_path = Path::new(&output_dir);

    if !output_path.exists() {
        fs::create_dir_all(output_path)?;
    } else {
        if output_path.read_dir()?.next().is_some() {
            anyhow::bail!("Output directory is not empty");
        }
    }
    let result_map = bit7z::extract(data, options.lib_path, options.password)?;
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

fn listing(lib_path: Option<String>, path: String) -> anyhow::Result<()> {
    let data = read_binary_file(path)?;
    let list_data = bit7z::list(data, lib_path)?;

    println!("{}", Paint::cyan("Archive properties"));
    println!("  {}: {}", Paint::blue("Items count"), list_data.items_count);
    println!("  {}: {}", Paint::blue("Folders count"), list_data.folders_count);
    println!("  {}: {}", Paint::blue("Files count"), list_data.files_count);
    println!("  {}: {}", Paint::blue("Size"), list_data.size);
    println!("  {}: {}", Paint::blue("Packed size"), list_data.packed_size);
    println!("  {}: {}", Paint::blue("Has encrypted items"), list_data.has_encrypted_items);
    println!("  {}: {}", Paint::blue("Is encrypted"), list_data.is_encrypted);
    println!("  {}: {}", Paint::blue("Volumes count"), list_data.volumes_count);
    println!("  {}: {}", Paint::blue("Is multi volume"), list_data.is_multi_volume);
    println!("  {}: {}\n", Paint::blue("Is solid"), list_data.is_solid);

    println!("{}", Paint::cyan("Archived items"));
    for item in &list_data.items {
        println!();
        println!("  {}: {}", Paint::blue("Item index"), item.index);
        println!("    {}: {}", Paint::blue("Name"), item.name);
        println!("    {}: {}", Paint::blue("Extension"), item.extension);
        println!("    {}: {}", Paint::blue("Path"), item.path);
        println!("    {}: {}", Paint::blue("Native path"), item.native_path);
        println!("    {}: {}", Paint::blue("Is directory"), item.is_dir);
        println!("    {}: {}", Paint::blue("Is symbolic link"), item.is_sym_link);
        println!("    {}: {}", Paint::blue("Size"), item.size);
        println!("    {}: {}", Paint::blue("Packed size"), item.packed_size);
        println!("    {}: {:x}", Paint::blue("CRC"), item.crc);
        println!("    {}: {}", Paint::blue("Is encrypted"), item.is_encrypted);
        println!("    {}: {}", Paint::blue("Attributes"), item.attributes);
        println!("    {}: {}", Paint::blue("Creation time"), item.creation_time);
        println!("    {}: {}", Paint::blue("Last access time"), item.last_access_time);
        println!("    {}: {}", Paint::blue("Last write time"), item.last_write_time);
    }

    Ok(())
}

pub fn read_binary_file<P: AsRef<Path>>(path: P) -> io::Result<Vec<u8>> {
    std::fs::read(path)
}

// ALL SUPPORTED TYPEs

/*
extern const BitInFormat Rar;       ///< RAR Archive Format
extern const BitInFormat Arj;       ///< ARJ Archive Format
extern const BitInFormat Z;         ///< Z Archive Format
extern const BitInFormat Lzh;       ///< LZH Archive Format
extern const BitInFormat Cab;       ///< CAB Archive Format
extern const BitInFormat Nsis;      ///< NSIS Archive Format
extern const BitInFormat Lzma;      ///< LZMA Archive Format
extern const BitInFormat Lzma86;    ///< LZMA86 Archive Format
extern const BitInFormat Ppmd;      ///< PPMD Archive Format
extern const BitInFormat Vhdx;      ///< VHDX Archive Format
extern const BitInFormat COFF;      ///< COFF Archive Format
extern const BitInFormat Ext;       ///< EXT Archive Format
extern const BitInFormat VMDK;      ///< VMDK Archive Format
extern const BitInFormat VDI;       ///< VDI Archive Format
extern const BitInFormat QCow;      ///< QCOW Archive Format
extern const BitInFormat GPT;       ///< GPT Archive Format
extern const BitInFormat Rar5;      ///< RAR5 Archive Format
extern const BitInFormat IHex;      ///< IHEX Archive Format
extern const BitInFormat Hxs;       ///< HXS Archive Format
extern const BitInFormat TE;        ///< TE Archive Format
extern const BitInFormat UEFIc;     ///< UEFIc Archive Format
extern const BitInFormat UEFIs;     ///< UEFIs Archive Format
extern const BitInFormat SquashFS;  ///< SquashFS Archive Format
extern const BitInFormat CramFS;    ///< CramFS Archive Format
extern const BitInFormat APM;       ///< APM Archive Format
extern const BitInFormat Mslz;      ///< MSLZ Archive Format
extern const BitInFormat Flv;       ///< FLV Archive Format
extern const BitInFormat Swf;       ///< SWF Archive Format
extern const BitInFormat Swfc;      ///< SWFC Archive Format
extern const BitInFormat Ntfs;      ///< NTFS Archive Format
extern const BitInFormat Fat;       ///< FAT Archive Format
extern const BitInFormat Mbr;       ///< MBR Archive Format
extern const BitInFormat Vhd;       ///< VHD Archive Format
extern const BitInFormat Pe;        ///< PE Archive Format
extern const BitInFormat Elf;       ///< ELF Archive Format
extern const BitInFormat Macho;     ///< MACHO Archive Format
extern const BitInFormat Udf;       ///< UDF Archive Format
extern const BitInFormat Xar;       ///< XAR Archive Format
extern const BitInFormat Mub;       ///< MUB Archive Format
extern const BitInFormat Hfs;       ///< HFS Archive Format
extern const BitInFormat Dmg;       ///< DMG Archive Format
extern const BitInFormat Compound;  ///< COMPOUND Archive Format
extern const BitInFormat Iso;       ///< ISO Archive Format
extern const BitInFormat Chm;       ///< CHM Archive Format
extern const BitInFormat Split;     ///< SPLIT Archive Format
extern const BitInFormat Rpm;       ///< RPM Archive Format
extern const BitInFormat Deb;       ///< DEB Archive Format
extern const BitInFormat Cpio;      ///< CPIO Archive Format

extern const BitInOutFormat Zip;        ///< ZIP Archive Format
extern const BitInOutFormat BZip2;      ///< BZIP2 Archive Format
extern const BitInOutFormat SevenZip;   ///< 7Z Archive Format
extern const BitInOutFormat Xz;         ///< XZ Archive Format
extern const BitInOutFormat Wim;        ///< WIM Archive Format
extern const BitInOutFormat Tar;        ///< TAR Archive Format
extern const BitInOutFormat GZip;       ///< GZIP Archive Format
*/
