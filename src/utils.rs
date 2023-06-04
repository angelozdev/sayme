pub fn format_text(s: String) -> String {
    let formatted_string = s.as_str().trim().replace("\n", "").to_string();
    formatted_string
}
