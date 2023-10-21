use figlet_rs::FIGfont;
use std::env;
use std::fs::File;
use std::io::{self, Write};
use std::process::abort;
use std::process::{Command, Stdio};

const APP_NAME: &str = "Media Sherlock";
const VERSION: &str = "0.1.0";
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
    if args[1] == ("-v") || args[1] == ("--version") {
        logo(APP_NAME);
        println!("Version: {VERSION}\nAuthor: {AUTHOR}");
        abort();
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

// Собираем нужную информацию о файле
// Видео дорожка: Codec ID, Width; Height, Display aspect ratio, Frame rate, Bit rate
// Ауадио дорожка:  Codec ID, Sampling rate, Channel(s), Bit rate
fn get_media_info(file_path: &str) -> io::Result<(String, String)> {
    let output = Command::new("mediainfo")
        .arg(file_path)
        .arg("--Output=JSON")
        .stdout(Stdio::piped())
        .output();

    match output {
        Ok(output) => {
            if output.status.success() {
                let json_output = String::from_utf8_lossy(&output.stdout);
                let parsed_data: serde_json::Value = serde_json::from_str(&json_output)?;

                let video_info = &parsed_data["media"]["track"][1];
                let audio_info = &parsed_data["media"]["track"][2];

                let video_codec_id = video_info["CodecID"].as_str().unwrap_or("N/A");
                let video_width = video_info["Width"].as_str().unwrap_or("N/A");
                let video_height = video_info["Height"].as_str().unwrap_or("N/A");
                let display_aspect_ratio =
                    video_info["DisplayAspectRatio"].as_str().unwrap_or("N/A");
                let frame_rate = video_info["FrameRate"].as_str().unwrap_or("N/A");
                let video_bit_rate = video_info["BitRate"].as_str().unwrap_or("N/A");

                // Преобразуем DisplayAspectRatio
                let display_aspect_ratio_formatted =
                    convert_display_aspect_ratio(display_aspect_ratio);

                let audio_codec_id = audio_info["CodecID"].as_str().unwrap_or("N/A");
                let sampling_rate = audio_info["SamplingRate"].as_str().unwrap_or("N/A");
                let channels = audio_info["Channels"].as_str().unwrap_or("N/A");
                let audio_bit_rate = audio_info["BitRate"].as_str().unwrap_or("N/A");

                let video_info_formatted = format!(
                    "{}, {}x{}p, {}, {} FPS, {} kb/s",
                    video_codec_id,
                    video_width,
                    video_height,
                    display_aspect_ratio_formatted,
                    frame_rate,
                    video_bit_rate
                );
                let audio_info_formatted = format!(
                    "{}, {} kHz, {}, {} kb/s",
                    audio_codec_id, sampling_rate, channels, audio_bit_rate
                );

                Ok((video_info_formatted, audio_info_formatted))
            } else {
                Err(io::Error::new(
                    io::ErrorKind::Other,
                    "Ошибка выполнения команды mediainfo",
                ))
            }
        }
        Err(_) => Err(io::Error::new(
            io::ErrorKind::Other,
            "Ошибка выполнения команды mediainfo",
        )),
    }
}

// конвертируем параметр Aspect Ratio в привычный формат (16:9 или 4:3 и тп)
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

// Create TMP file...
fn create_temp_file(video_info: &str, audio_info: &str) -> io::Result<String> {
    let temp_dir = env::temp_dir();
    let temp_file_path = temp_dir.join("mediainfo.txt");

    let mut file = File::create(&temp_file_path)?;

    write!(file, "{}\n{}\n", video_info, audio_info)?;

    Ok(temp_file_path.to_string_lossy().to_string())
}

// Open info at the notepad...
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
