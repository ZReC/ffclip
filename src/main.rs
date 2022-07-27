/**
 * Copyright (c) 2022 ZReC and others
 * MIT License
 */
use arboard::Clipboard;
use std::env;
use std::io::prelude::*;
use std::process::{Command, Stdio};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut cb = Clipboard::new()?;
    let img = cb.get_image()?;

    let args = env::args().collect::<Vec<String>>();

    let mut ffmpeg = Command::new("ffmpeg")
        .args(vec![
            String::from("-f"),
            String::from("pam_pipe"),
            String::from("-i"),
            String::from("-"),
        ])
        .args(&args[1..])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;

    let mut stdin = ffmpeg.stdin.take().ok_or("Failed to open stdin")?;

    stdin.write_all(
        format!(
            "P7\nWIDTH {}\nHEIGHT {}\nDEPTH 4\nMAXVAL 255\nTUPLTYPE RGB_ALPHA\nENDHDR\n",
            img.width, img.height
        )
        .as_bytes(),
    )?;
    stdin.write_all(&img.bytes)?;
    drop(stdin);

    ffmpeg.wait()?;

    Ok(())
}
