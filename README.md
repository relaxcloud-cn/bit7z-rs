# 中文

## bit7z-rs 概况

`bit7z-rs` 用Rust封装 `bit7z` 项目。需要注意的是，`bit7z` 暂时不支持 7z 的静态链接，所以你在使用 `bit7z-rs` 时也需要显示地指定 `7z` 动态库的路径，`bit7z-rs` 也能通过环境变量 `LIB_PATH_7Z` 找到它。

# English

## How bit7z-rs works

`bit7z-rs` is a Rust wrapper for the `bit7z` project. Please note that `bit7z` currently does not support static linking with `7z`, so when using `bit7z-rs`, you need to explicitly specify the path to the `7z` dynamic library, or alternatively, you can set the environment variable `LIB_PATH_7Z` to help bit7z-rs locate it.
