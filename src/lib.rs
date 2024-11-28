// src/lib.rs
use ffi_bridge::cxx_extracting;
use std::collections::HashMap;
use std::error::Error;
use std::fs::{self, File};
use std::io;
use std::io::Write;
use std::path::Path;

#[cxx::bridge]
mod ffi_bridge {
    struct FilenameAndData {
        filename: String,
        data: Vec<u8>,
    }

    unsafe extern "C++" {
        include!("bridge.hpp");

        fn cxx_extracting(
            lib_path: String,
            data: &Vec<u8>,
            password: String,
        ) -> Result<Vec<FilenameAndData>>;

    }
}

#[derive(Default, Debug)]
pub struct ExtractOptions {
    /// Output directory path
    pub output_dir: Option<String>,
    /// Password for encrypted archives
    pub password: Option<String>,
}

pub fn extracting(
    lib_path: String,
    data: Vec<u8>,
    options: ExtractOptions,
) -> Result<String, Box<dyn Error>> {
    let password = options.password.unwrap_or_default();
    let output_dir = options.output_dir.ok_or("Output directory not specified")?;
    let output_path = Path::new(&output_dir);

    if !output_path.exists() {
        fs::create_dir_all(output_path)?;
    } else {
        if output_path.read_dir()?.next().is_some() {
            return Err("Output directory is not empty".into());
        }
    }

    let data = cxx_extracting(lib_path, &data, password)?;
    let mut result_map = HashMap::new();
    for kv in data {
        result_map.insert(kv.filename, kv.data);
    }

    for (filename, content) in &result_map {
        let file_path = output_path.join(filename);

        if let Some(parent) = file_path.parent() {
            if !parent.exists() {
                fs::create_dir_all(parent)?;
            }
        }

        let mut file = File::create(file_path)?;
        file.write_all(content)?;
        println!("{} - {} bytes written", filename, content.len());
    }

    Ok(format!("Files extracted to: {}", output_dir))
}

pub fn read_binary_file<P: AsRef<Path>>(path: P) -> io::Result<Vec<u8>> {
    std::fs::read(path)
}

// fn write_binary_file<P: AsRef<Path>>(path: P, data: &[u8]) -> io::Result<()> {
//     let mut file = File::create(path)?;
//     file.write_all(data)
// }

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
