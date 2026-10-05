use std::path::PathBuf;

use bandcamp::TrackMetadata;
use clap::{arg, command};
mod bandcamp;
mod downloader;

fn main() {
    let matches = command!()
        .name("bandip")
        .version("0.2.0")
        .about("A simple bandcamp downloader.")
        .arg(
            arg!(<LINK>)
                .required(true)
                .id("link")
                .help("The bandcamp link to download from."),
        )
        .get_matches();
    let download_links = bandcamp::extract_audio_links(matches.get_one::<String>("link").unwrap());
    if download_links.is_err() {
        println!("Error parsing bandcamp site.");
        return;
    }
    let download_links = download_links.unwrap();
    println!(
        "Starting download. Found {} tracks...",
        download_links.len()
    );
    for (link, metadata) in download_links.to_owned() {
        let downloaded_file = match downloader::download_from_link(link) {
            Ok(path) => path,
            Err(err) => {
                eprintln!("Download failed: {err}");
                return;
            }
        };
        if let Err(err) = downloader::move_and_tag_file(downloaded_file, metadata.to_owned()) {
            eprintln!("Failed to save track: {err}");
            return;
        }
        println!(
            "Downloaded {} {} by {}",
            metadata.track_number, metadata.name, metadata.artist
        );
    }
    match get_download_dir(&download_links.first().unwrap().1) {
        Ok(dir) => println!("Finished downloading to {:?}", dir.into_os_string()),
        Err(err) => eprintln!("Tracks downloaded, but could not resolve output dir: {err}"),
    }
}

fn get_download_dir(metadata: &TrackMetadata) -> Result<PathBuf, Box<dyn std::error::Error + Send + Sync>> {
    let mut download_dir = downloader::music_dir()?;
    download_dir.push("bandrip");
    download_dir.push(&metadata.artist);
    download_dir.push(&metadata.album);
    Ok(download_dir)
}
