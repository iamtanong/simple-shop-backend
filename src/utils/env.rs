use std::env;

pub fn read_str_env(key: &str, default: String) -> String {
    env::var(key).unwrap_or(default)
}

pub fn read_int_env(key: &str, default: i16) -> i16 {
    env::var(key)
        .unwrap_or(default.to_string())
        .parse::<i16>()
        .unwrap_or(default)
}
