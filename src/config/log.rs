use chrono::Local;
use env_logger::Builder;
use log::LevelFilter;
use std::io::Write;

pub fn init_logger(level: String) {
    // Log only for module `simple_shop_backend`
    // And format into `[{timestamp(local)} {level} {source}] {args}`

    Builder::new()
        .filter(
            "simple_shop_backend".into(),
            match level.as_str() {
                "info" => LevelFilter::Info,
                "debug" => LevelFilter::Debug,
                "warn" => LevelFilter::Warn,
                "error" => LevelFilter::Error,
                _ => LevelFilter::Info,
            },
        )
        .format(|buf, record| {
            let local_timestamp = Local::now().format("%Y-%m-%dT%H:%M:%S");
            let warn_style = buf.default_level_style(record.level());
            let source_file = match (record.file(), record.line()) {
                (Some(f), Some(l)) => format!("{}:{}", f, l),
                (Some(f), None) => f.to_string(),
                _ => "".into(),
            };
            writeln!(
                buf,
                "[{} {warn_style}{:<5}{warn_style:#} {}] {}",
                local_timestamp,
                record.level(),
                source_file,
                record.args(),
            )
        })
        .target(env_logger::Target::Stdout)
        .init();
}
