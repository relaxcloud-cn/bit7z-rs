// src/lib.rs
use ffi_bridge::cxx_extracting;
use std::collections::HashMap;
use std::env;
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

pub fn extracting(
    data: Vec<u8>,
    lib_path: Option<String>,
    password: Option<String>,
) -> anyhow::Result<HashMap<String, Vec<u8>>> {
    let password = password.unwrap_or("".to_owned());

    let valid_lib_path = get_lib_path(lib_path)?;

    let data = cxx_extracting(valid_lib_path, &data, password)?;
    let mut result_map = HashMap::new();
    for kv in data {
        result_map.insert(kv.filename, kv.data);
    }

    Ok(result_map)
}

fn get_lib_path(lib_path: Option<String>) -> anyhow::Result<String> {
    let path = match lib_path {
        Some(path) => path,
        None => env::var("LIB_PATH_7Z")
            .map_err(|_| anyhow::anyhow!("Environment variable 'LIB_PATH_7Z' not found"))?,
    };

    if !Path::new(&path).exists() {
        anyhow::bail!(format!("Library path '{}' does not exist", path));
    }

    Ok(path)
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
