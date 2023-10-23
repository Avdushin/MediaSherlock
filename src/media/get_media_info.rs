// Media module
// Mod, to parse media info from MediaInfo output
use std::io;
use std::process::{Command, Stdio};

use crate::format::convert::convert_display_aspect_ratio;
use crate::format::outputs::{format_bitrate, remove_prefixes};

pub fn get_media_info(file_path: &str) -> io::Result<(Vec<String>, Vec<String>)> {
    let output = Command::new("mediainfo")
        .arg(file_path)
        .arg("--Output=JSON")
        .stdout(Stdio::piped())
        .output();

    let mut video_info = Vec::new();
    let mut audio_info = Vec::new();

    match output {
        Ok(output) => {
            if output.status.success() {
                let json_output = String::from_utf8_lossy(&output.stdout);
                let parsed_data: serde_json::Value = serde_json::from_str(&json_output)?;

                if let Some(media_tracks) = parsed_data["media"]["track"].as_array() {
                    for track in media_tracks {
                        let track_type = track["@type"].as_str().unwrap_or("N/A");
                        let codec_id = track["CodecID"].as_str().unwrap_or("N/A");
                        let bit_rate = track["BitRate"].as_str().unwrap_or("N/A");
                        let formatted_bit_rate = format_bitrate(bit_rate);

                        if track_type == "Video" {
                            let width = track["Width"].as_str().unwrap_or("N/A");
                            let height = track["Height"].as_str().unwrap_or("N/A");
                            let display_aspect_ratio = convert_display_aspect_ratio(
                                track["DisplayAspectRatio"].as_str().unwrap_or("N/A"),
                            );
                            let frame_rate = track["FrameRate"].as_str().unwrap_or("N/A");

                            let video_info_formatted = format!(
                                "{}, {}x{}p, {}, {:.5} FPS, {}",
                                codec_id,
                                width,
                                height,
                                display_aspect_ratio,
                                frame_rate,
                                formatted_bit_rate
                            );
                            video_info.push(remove_prefixes(&video_info_formatted));
                        } else if track_type == "Audio" {
                            let sampling_rate = track["SamplingRate"].as_str().unwrap_or("N/A");
                            let channels = track["Channels"].as_str().unwrap_or("N/A");

                            let audio_info_formatted = format!(
                                "{}, {} kHz, {} ch, {}",
                                codec_id, sampling_rate, channels, formatted_bit_rate
                            );

                            audio_info.push(remove_prefixes(&audio_info_formatted));
                        }
                    }
                }
            }
        }
        Err(_) => {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "Ошибка выполнения команды mediainfo",
            ));
        }
    }

    if video_info.is_empty() {
        video_info.push("No video track found".to_string());
    }

    if audio_info.is_empty() {
        audio_info.push("No audio track found".to_string());
    }

    Ok((video_info, audio_info))
}
