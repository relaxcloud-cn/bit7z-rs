// src/lib.rs
use ffi_bridge::{cxx_extract, cxx_list, ListData};
use std::collections::HashMap;
use std::env;
use std::path::Path;

#[cxx::bridge]
mod ffi_bridge {
    struct FilenameAndData {
        filename: String,
        data: Vec<u8>,
    }

    struct ListData {
        items_count: u32,
        folders_count: u32,
        files_count: u32,
        size: u64,
        packed_size: u64,
        has_encrypted_items: bool,
        is_encrypted: bool,
        volumes_count: u32,
        is_multi_volume: bool,
        is_solid: bool,
        items: Vec<Item>,
    }

    struct Item {
        index: u32,
        is_dir: bool,
        is_sym_link: bool,
        name: String,
        extension: String,
        path: String,
        native_path: String,
        size: u64,
        creation_time: i64,
        last_access_time: i64,
        last_write_time: i64,
        attributes: u32,
        packed_size: u64,
        crc: u32,
        is_encrypted: bool,
    }

    unsafe extern "C++" {
        include!("bridge.hpp");

        fn cxx_extract(
            lib_path: String,
            data: &Vec<u8>,
            password: String,
        ) -> Result<Vec<FilenameAndData>>;

        fn cxx_list(lib_path: String, data: &Vec<u8>) -> Result<ListData>;
    }
}

pub fn extract(
    data: Vec<u8>,
    lib_path: Option<String>,
    password: Option<String>,
) -> anyhow::Result<HashMap<String, Vec<u8>>> {
    let password = password.unwrap_or(String::new());

    let valid_lib_path = get_lib_path(lib_path)?;

    let data = cxx_extract(valid_lib_path, &data, password)?;
    let mut result_map = HashMap::new();
    for kv in data {
        result_map.insert(kv.filename, kv.data);
    }

    Ok(result_map)
}

pub fn list(data: Vec<u8>, lib_path: Option<String>) -> anyhow::Result<ListData> {
    let valid_lib_path = get_lib_path(lib_path)?;
    Ok(cxx_list(valid_lib_path, &data)?)
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
