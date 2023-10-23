// Mod with convert functions

// Конвертируем параметр Aspect Ratio в привычный формат
pub fn convert_display_aspect_ratio(raw_ratio: &str) -> String {
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
