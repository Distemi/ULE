use crate::Color;
use fern::colors::ColoredLevelConfig;
use std::fs;

// Logger's initialize(fern, color and log)
pub fn setup_logger() -> Result<(), fern::InitError> {
    // Removing latest log if exists
    fs::remove_file("latest.log");
    // Setting colors
    let colors = ColoredLevelConfig::new()
        .info(Color::BrightBlack)
        .warn(Color::Yellow)
        .error(Color::Red)
        .trace(Color::BrightRed);
    // Setting fern
    fern::Dispatch::new()
        // Setting custom format to logging
        .format(move |out, message, record| {
            out.finish(format_args!(
                "{} [{}] {}",
                chrono::Local::now().format("[%m-%d %H:%M:%S]"),
                colors.color(record.level()),
                message
            ))
        })
        // Setting log-level
        .level(log::LevelFilter::Info)
        // Setting target's logger
        .chain(std::io::stdout())
        // Setting log's file
        .chain(fern::log_file("latest.log")?)
        // Applying settings
        .apply()?;
    // If successful setting - returning ok
    Ok(())
}
