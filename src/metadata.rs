use anyhow::{bail, Context, Result};
use serde_json::Value;
use std::{fs::File, io::Write, path::Path, process::Command};

pub fn run_ffprobe_json(input: &str) -> Result<Value> {
    let output = Command::new("ffprobe")
        .args(&[
            "-v",
            "error",
            "-print_format",
            "json",
            "-show_format",
            "-show_streams",
            "-show_chapters",
            input,
        ])
        .output()
        .context("Failed to execute ffprobe")?;

    if !output.status.success() {
        bail!(
            "ffprobe failed: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }

    let json: Value =
        serde_json::from_slice(&output.stdout).context("Failed to parse ffprobe JSON")?;
    Ok(json)
}

pub fn write_metadata_files(output_dir: &str, json: &Value) -> Result<()> {
    let metadata_path = Path::new(output_dir).join("metadata.json");
    let mut metadata_file = File::create(&metadata_path)?;
    serde_json::to_writer_pretty(&mut metadata_file, json)?;

    let format_tags = json
        .get("format")
        .and_then(|f| f.get("tags"))
        .cloned()
        .unwrap_or_else(|| serde_json::json!({}));

    let mut chapters_map = serde_json::Map::new();
    if let Some(chapters) = json["chapters"].as_array() {
        for chapter in chapters {
            if let Some(id) = chapter["id"].as_i64() {
                let key = format!("Chapter_{}", id);
                let val = chapter["tags"]["title"].as_str().unwrap_or("Untitled");
                chapters_map.insert(key, serde_json::json!(val));
            }
        }
    }

    let mut merged = serde_json::Map::new();
    if let Some(map) = format_tags.as_object() {
        for (k, v) in map.iter() {
            merged.insert(k.clone(), v.clone());
        }
    }
    for (k, v) in chapters_map {
        merged.insert(k, v);
    }

    let tags_yaml = serde_yaml::to_string(&merged)?;

    let tags_path = Path::new(output_dir).join("tags.yaml");
    let mut tags_file = File::create(tags_path)?;
    tags_file.write_all(tags_yaml.as_bytes())?;

    Ok(())
}

pub fn extract_cover(input: &str, output_dir: &str, json: &Value) -> Result<()> {
    println!("\n🖼️ Detecting cover image stream...");
    let cover_stream_index = json["streams"]
        .as_array()
        .context("No streams found")?
        .iter()
        .find_map(|stream| {
            if stream["disposition"]["attached_pic"].as_i64() == Some(1) {
                stream["index"].as_i64()
            } else {
                None
            }
        });

    if let Some(idx) = cover_stream_index {
        println!("Extracting cover image from stream {} to folder.jpg", idx);
        let output_path = format!("{}/folder.jpg", output_dir);
        let status = Command::new("ffmpeg")
            .args(&[
                "-hide_banner",
                "-loglevel",
                "quiet",
                "-y",
                "-i",
                input,
                "-map",
                &format!("0:{}", idx),
                "-c",
                "copy",
                &output_path,
            ])
            .status()
            .context("Failed to run ffmpeg for cover extraction")?;

        if status.success() {
            println!("✅ Cover image extracted successfully.");
        } else {
            eprintln!("⚠️ Failed to extract cover image.");
        }
    } else {
        println!("⚠️ No cover image stream found, skipping cover extraction.");
    }

    Ok(())
}
