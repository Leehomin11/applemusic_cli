extern crate colored;

use colored::Colorize;
use std::io::Write;
use std::process::Command;
use tempfile::NamedTempFile;
use std::{thread, time::Duration};

fn main() {
    let script = r#"
        tell application "Music"
            if player state is playing then
                try
                    set trackName to name of current track
                on error
                    set trackName to "Unknown Title"
                end try

                try
                    set artistName to artist of current track
                on error
                    set artistName to "Unknown Artist"
                end try

                try
                    set albumName to album of current track
                on error
                    set albumName to "Unknown Album"
                end try

                try
                    set playerPos to player position
                on error
                    set playerPos to 0
                end try

                return trackName & "|" & artistName & "|" & albumName & "|" & playerPos
            else
                return "Nothing Playing"
            end if
        end tell
    "#;

    let mut temp_script = NamedTempFile::new().expect("❌ 임시 파일 생성 실패");
    write!(temp_script, "{}", script).expect("❌ 스크립트 작성 실패");
    let script_path = temp_script.path();

    let mut prev_title = String::new();
    let mut prev_artist = String::new();
    let mut prev_album = String::new();
    let mut first_render = true;
    let mut not_playing_count = 0;

    
    loop {
        match get_music_info(script_path) {
            Some((title, artist, album, pos)) => {
                not_playing_count = 0;
                let minutes = pos / 60;
                let seconds = pos % 60;
    
                if title != prev_title || artist != prev_artist || album != prev_album || first_render {
                    if !first_render {
                        print!("\x1b[6A");
                        for _ in 0..6 {
                            print!("\x1b[2K\x1b[1B");
                        }
                        print!("\x1b[6A");
                    }
    
                    println!("🎶 {}\n", "Now Playing - Apple Music".bold());
                    println!("🎵 {} {}", "Title:".cyan(), title);
                    println!("🎤 {} {}", "Artist:".magenta(), artist);
                    println!("💿 {} {}", "Album:".blue(), album);
                    println!("⏱️ {} {:02}:{:02}", "Elapsed:".yellow(), minutes, seconds);
    
                    prev_title = title.clone();
                    prev_artist = artist.clone();
                    prev_album = album.clone();
                    first_render = false;
                } else {
                    print!("\x1b[1A\x1b[2K");
                    println!("⏱️ {} {:02}:{:02}", "Elapsed:".yellow(), minutes, seconds);
                }
    
                std::io::stdout().flush().unwrap();
            }
            None => {
                not_playing_count += 1;
                if not_playing_count >= 2 {
                    print!("\x1b[6A");
                    for _ in 0..6 {
                        print!("\x1b[2K\x1b[1B");
                    }
                    print!("\x1b[6A");
                    println!("🚫 {}", "노래가 정지되었거나 종료되었습니다.".red());
                    std::io::stdout().flush().unwrap();
                    break;
                } else {
                }
            }
        }
    
        thread::sleep(Duration::from_secs(1));
    }
}

fn get_music_info(script_path: &std::path::Path) -> Option<(String, String, String, u64)> {
    let output = Command::new("osascript")
        .arg(script_path)
        .output()
        .ok()?;

    let stdout_str = String::from_utf8_lossy(&output.stdout).trim().to_string();

    if stdout_str == "Nothing Playing" {
        return None;
    }

    let parts: Vec<&str> = stdout_str.split('|').collect();
    if parts.len() != 4 {
        return None;
    }

    let title = parts[0].to_string();
    let artist = parts[1].to_string();
    let album = parts[2].to_string();
    let pos = parts[3].parse::<f64>().unwrap_or(0.0);

    Some((title, artist, album, pos.floor() as u64))
}
