use figlet_rs::FIGfont;
use std::env;
use std::fs::File;
use std::io::{self, Write};
use std::process::{Command, Stdio};

const APP_NAME: &str = "Media Sherlock";
const VERSION: &str = "0.1.2";
const AUTHOR: &str = "https://github.com/Avdushin";

fn main() -> io::Result<()> {
    // Получаем аргумент командной строки
    let args: Vec<String> = env::args().collect();

    // Default showcase
    if args.len() != 2 {
        logo(APP_NAME);
        println!("Usage:\n\tmediasherlock <путь_до_файла> - to open file info\n\t-v or --version - to show app info");
        return Ok(());
    }
    // Version showcase
    if args[1] == "-v" || args[1] == "--version" {
        logo(APP_NAME);
        println!("Version: {}\nAuthor: {}", VERSION, AUTHOR);
        return Ok(());
    }

    // Выполняем команду mediainfo с путем до файла
    let output = get_media_info(&args[1]);

    match output {
        Ok((video_info, audio_info)) => {
            // Создаем tmp-файл и записываем в него информацию
            let temp_file_path = create_temp_file(&video_info, &audio_info)?;

            // Открываем tmp-файл в блокноте
            open_file_in_notepad(&temp_file_path);

            // Удаляем временный файл
            if let Err(e) = std::fs::remove_file(&temp_file_path) {
                eprintln!("Ошибка при удалении временного файла: {:?}", e);
            }
        }
        Err(_) => {
            println!("Ошибка выполнения команды mediainfo");
        }
    }

    Ok(())
}

// Собираем информацию о файле
fn get_media_info(file_path: &str) -> io::Result<(Vec<String>, Vec<String>)> {
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
                            let display_aspect_ratio = track["DisplayAspectRatio"].as_str().unwrap_or("N/A");
                            let frame_rate = track["FrameRate"].as_str().unwrap_or("N/A");

                            let video_info_formatted = format!(
                                "{}, {}x{}p, {}, {} FPS, {}",
                                codec_id, width, height, display_aspect_ratio, frame_rate, formatted_bit_rate
                            );

                            video_info.push(video_info_formatted);
                        } else if track_type == "Audio" {
                            let sampling_rate = track["SamplingRate"].as_str().unwrap_or("N/A");
                            let channels = track["Channels"].as_str().unwrap_or("N/A");

                            let audio_info_formatted = format!(
                                "{}, {} kHz, {} ch, {}",
                                codec_id, sampling_rate, channels, formatted_bit_rate
                            );

                            audio_info.push(audio_info_formatted);
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


// конвертируем параметр Aspect Ratio в привычный формат
fn convert_display_aspect_ratio(raw_ratio: &str) -> String {
    match raw_ratio {
        "1.778" => "16:9".to_string(),
        "1.333" => "4:3".to_string(),
        "2.35" => "2.35:1".to_string(),
        "2.40" => "2.40:1".to_string(),
        "2.000" => "18:9".to_string(),
        "1.250" => "5:4".to_string(),
        "2.333" => "21:9".to_string(),
        "1.500" => "3:2".to_string(),
        "1.600" | "8:5" => "16:10".to_string(),
        "1.850" => "37:20".to_string(),
        "1.900" => "19:10".to_string(),
        "2.550" => "17:10".to_string(),
        "2.760" => "32:11".to_string(),
        "2.800" => "7:2".to_string(),
        "2.840" => "71:25".to_string(),
        "1.667" | "5:3" => "5:3".to_string(),
        "0.562" => "9:16".to_string(),
        _ => raw_ratio.to_string(),
    }
}

// Функция для форматирования битрейта в тысячах
fn format_bitrate(bitrate: &str) -> String {
    if let Ok(bitrate_val) = bitrate.parse::<f64>() {
        let formatted_bitrate = (bitrate_val / 1000.0).round();
        return format!("{:.0} kb/s", formatted_bitrate);
    }
    bitrate.to_string()
}

// Создаем TMP файл
fn create_temp_file(video_info: &Vec<String>, audio_info: &Vec<String>) -> io::Result<String> {
    let temp_dir = env::temp_dir();
    let temp_file_path = temp_dir.join("mediainfo.txt");

    let mut file = File::create(&temp_file_path)?;

    for info in video_info.iter() {
        write!(file, "{}\n", info)?;
    }
    for info in audio_info.iter() {
        write!(file, "{}\n", info)?;
    }

    Ok(temp_file_path.to_string_lossy().to_string())
}

// Открываем файл в блокноте
fn open_file_in_notepad(file_path: &str) {
    Command::new("notepad.exe")
        .arg(file_path)
        .stdout(Stdio::null())
        .spawn()
        .expect("Ошибка при открытии блокнота");

    // Подождать немного перед скрытием окна консоли
    std::thread::sleep(std::time::Duration::from_secs(1));

    // Запускаем NirCmd, чтобы скрыть окно консоли
    Command::new("nircmd")
        .args(&["win", "hide", "class", "ConsoleWindowClass"])
        .stdout(Stdio::null())
        .spawn()
        .expect("Ошибка при скрытии окна консоли");
}

// Show App's logotype
fn logo(name: &str) {
    let standard_font = FIGfont::standard().unwrap();
    let figure = standard_font.convert(name);
    assert!(figure.is_some());
    println!("{}", figure.unwrap());
}
