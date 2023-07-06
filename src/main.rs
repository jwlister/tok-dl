// TODO: Automate collecting of URLs using down arrow keys on TikTok website.

use clap::Parser;
use env_logger::Env;
use log::{debug, error, info, warn};
use regex::Regex;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

#[derive(Parser, Debug)]
struct UserCommand {
    #[arg(short = 'i', long, help = "Path to user's HTML page")]
    html_page: PathBuf,

    #[arg(short, long, help = "Path of the directory to download the videos to")]
    download_dir: PathBuf,
}

#[derive(Parser, Debug)]
struct BatchUserCommand {
    #[arg(
        short = 'i',
        long,
        help = "Path to directory containing all users' HTML pages"
    )]
    html_dir: PathBuf,

    #[arg(short, long, help = "Path of the directory to download the videos to")]
    download_dir: PathBuf,
}

#[derive(Parser, Debug)]
struct ListFileCommand {
    #[arg(
        short = 'i',
        long,
        help = "Path to plain-text file containing line-separated TikTok video URLs"
    )]
    list_file: PathBuf,

    #[arg(short, long, help = "Path of the directory to download the videos to")]
    download_dir: PathBuf,
}

#[derive(Debug, Parser)]
#[command(
    author = "Jeremiah Lister",
    version = "0.1.0",
    about = "A program that partially automates the mass downloading of TikTok videos"
)]
enum Cli {
    #[command(
        about = "Use the provided web page to download all videos from the user that the page belongs to"
    )]
    User(UserCommand),

    #[command(
        about = "Use the web pages in the provided directory to download all videos from the users that the pages belong to"
    )]
    BatchUser(BatchUserCommand),

    #[command(about = "Download all videos listed in the provided file")]
    ListFile(ListFileCommand),
}

fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let args = Cli::parse();
    match args {
        Cli::User(cmd) => {
            debug!("user command initiated");
            download_user(&cmd.html_page, &cmd.download_dir);
        }
        Cli::BatchUser(cmd) => {
            debug!("batch-user command initiated");
            batch_download_user(&cmd.html_dir, &cmd.download_dir);
        }
        Cli::ListFile(cmd) => {
            debug!("list-file command initiated");
            download_from_list_file(&cmd.list_file, &cmd.download_dir);
        }
    }
}

fn batch_download_user(html_dir_path: &Path, dst: &Path) {
    for html_page_path in fs::read_dir(html_dir_path)
        .expect("Invalid html-dir path")
        .map(|entry| entry.unwrap())
        .filter(|entry| entry.file_type().unwrap().is_file())
        .map(|entry| entry.path())
    {
        let html_page = fs::read_to_string(&html_page_path)
            .expect("Invalid HTML file path in batch-user command");
        let username = get_username(&html_page);
        let urls = scrape_video_urls(&html_page, username);

        info!("Downloading {username}'s {} videos", urls.len());

        let mut dst = dst.to_owned();
        dst.push(username);
        for url in urls {
            if download_video(url, &dst).is_ok() {
                info!("SUCCESS: {url}");
            } else {
                warn!("FAILURE: {url}");
            }
        }
    }
}

fn download_user(html_page_path: &Path, dst: &Path) {
    let html_page = fs::read_to_string(html_page_path).expect("Invalid html-page path");
    let username = get_username(&html_page);
    let urls = scrape_video_urls(&html_page, username);

    info!("Downloading {username}'s {} videos", urls.len());

    for url in urls {
        if download_video(url, dst).is_ok() {
            info!("SUCCESS: {url}");
        } else {
            warn!("FAILURE: {url}");
        }
    }
}

fn scrape_video_urls<'a>(s: &'a str, username: &str) -> Vec<&'a str> {
    Regex::new(&format!(r"https://www.tiktok.com/@{username}/video/\d+"))
        .unwrap()
        .find_iter(s)
        .map(|mat| mat.as_str())
        .collect()
}

fn get_username(s: &str) -> &str {
    Regex::new(r"<title>.*? \(@([\w.]+)\).*?</title>")
        .unwrap()
        .captures(s)
        .expect("Could not find username in HTML file")
        .get(1)
        .unwrap()
        .as_str()
}

fn download_from_list_file(list_file_path: &Path, dst: &Path) {
    for url in fs::read_to_string(list_file_path)
        .expect("Invalid list-file path")
        .lines()
        .collect::<Vec<_>>()
    {
        if download_video(url, dst).is_ok() {
            info!("SUCCESS: {url}");
        } else {
            warn!("FAILURE: {url}");
        }
    }
}

enum DownloadError {
    UnableToCreateProcess,
    Failed,
}

fn download_video(url: &str, dst: &Path) -> Result<(), DownloadError> {
    fs::create_dir_all(dst).expect("Could not create destination directory");

    let res = download_video_with_args(dst, &[url]);
    match res {
        Err(DownloadError::Failed) => {
            debug!("First attempt failed. Trying again with name truncation...");
            download_video_with_args(dst, &["-o", "%(id)s.%(ext)s", url])
        }
        _ => res,
    }
}

fn download_video_with_args(dst: &Path, args: &[&str]) -> Result<(), DownloadError> {
    match Command::new("yt-dlp").current_dir(dst).args(args).output() {
        Ok(output) => {
            if output.status.success() {
                Ok(())
            } else {
                Err(DownloadError::Failed)
            }
        }
        Err(err) => {
            error!("Failed to create yt-dlp process: {err}");
            Err(DownloadError::UnableToCreateProcess)
        }
    }
}
