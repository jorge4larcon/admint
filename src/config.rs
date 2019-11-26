// Author: Jorge Alarcon Alvarez
// Email:  jorge4larcon@gmail.com
// This module is used to configure the application logging.

extern crate fern;
extern crate log;

pub fn setup_logging(log_level: &log::LevelFilter) -> Result<(), fern::InitError> {
    let colors = fern::colors::ColoredLevelConfig::new().info(fern::colors::Color::Green)
                                                        .warn(fern::colors::Color::Yellow)
                                                        .error(fern::colors::Color::Red)
                                                        .debug(fern::colors::Color::Blue)
                                                        .trace(fern::colors::Color::Magenta);
    fern::Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                "{}: {}",
                colors.color(record.level()),
                message
            ))
        })
        .level(*log_level)
        .chain(std::io::stdout())        
        .apply()?;        
    Ok(())
}
