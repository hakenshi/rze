//! rze-infra: adapters for OS boundaries.

pub mod atomic_write;
pub mod curl_downloader;
pub mod env_detect;
pub mod ffmpeg_decoder;
pub mod paths;
pub mod process_runner;
pub mod symlink_deploy;

pub mod cosmic;
pub mod resets;
pub mod wallpaper;
