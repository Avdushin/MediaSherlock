use figlet_rs::FIGfont;
use regex::Regex;

// Show App's logotype
pub fn logo(name: &str) {
    let standard_font = FIGfont::standard().unwrap();
    let figure = standard_font.convert(name);
    assert!(figure.is_some());
    println!("{}", figure.unwrap());
}

// Функция для форматирования битрейта в тысячах
pub fn format_bitrate(bitrate: &str) -> String {
    if let Ok(bitrate_val) = bitrate.parse::<f64>() {
        let formatted_bitrate = (bitrate_val / 1000.0).round();
        return format!("{} kb/s", formatted_bitrate);
    }
    bitrate.to_string()
}

// Removing of "V_" и "A_" prefixes
pub fn remove_prefixes(input: &str) -> String {
    let re = Regex::new(r"[VA]_").unwrap();
    re.replace_all(input, "").to_string()
}
