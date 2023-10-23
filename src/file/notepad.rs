// Mod for work with files and notepad
use std::env;
use std::fs::File;
use std::io::{self, Write};
use std::process::{Command, Stdio};

// Создаем TMP файл
/// Создаем временный файл, в который будем выводить нужную информацию
pub fn create_temp_file(video_info: &Vec<String>, audio_info: &Vec<String>) -> io::Result<String> {
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
pub fn open_file_in_notepad(file_path: &str) {
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
