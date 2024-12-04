// src/lib.rs
use ffi_bridge::cxx_extract;
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

        fn cxx_extract(
            lib_path: String,
            data: &Vec<u8>,
            password: String,
        ) -> Result<Vec<FilenameAndData>>;

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
