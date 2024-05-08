use sys_locale::get_locale;

pub fn get_language() -> String {
    let locale: String = get_locale().unwrap_or_else(|| String::from("en-US"));
    let locale_parts: Vec<&str> = locale.split('-').collect();
    String::from(*locale_parts.first().unwrap())
}
