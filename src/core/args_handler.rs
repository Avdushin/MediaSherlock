// Mod with public functions to work with args
use crate::core::constants;
use crate::file::notepad::{create_temp_file, open_file_in_notepad};
use crate::format::outputs;
use crate::media::get_media_info::get_media_info;

pub fn handle_args(args: Vec<String>) -> Result<(), String> {
    if args.len() != 2 {
        outputs::logo(constants::APP_NAME);
        println!("Usage:\n\tmediasherlock <путь_до_файла> - to open file info\n\t-v or --version - to show app info");
        return Ok(());
    }
    if args[1] == "-v" || args[1] == "--version" {
        outputs::logo(constants::APP_NAME);
        println!(
            "Version: {}\nAuthor: {}",
            constants::VERSION,
            constants::AUTHOR
        );
        return Ok(());
    }

    // Handle the case of processing a media file
    if args.len() == 2 {
        let file_path = &args[1];
        let output = get_media_info(file_path);
        match output {
            Ok((video_info, audio_info)) => {
                let temp_file_path = create_temp_file(&video_info, &audio_info);
                if let Err(e) = temp_file_path {
                    eprintln!("Ошибка при создании временного файла: {:?}", e);
                } else {
                    let temp_file_path_str = temp_file_path.unwrap().to_string();
                    open_file_in_notepad(&temp_file_path_str);
                    if let Err(e) = std::fs::remove_file(&temp_file_path_str) {
                        eprintln!("Ошибка при удалении временного файла: {:?}", e);
                    }
                }
            }
            Err(_) => {
                println!("Ошибка выполнения команды mediainfo");
            }
        }
    }

    Ok(())
}
