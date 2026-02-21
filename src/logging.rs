use crate::cli::ColorChoice;
use env_logger::Builder;
use log::LevelFilter;
use std::io::Write;

/// Initialize logging based on CLI flags
pub fn init_logging(quiet: bool, verbose: u8, color: ColorChoice) {
    let mut builder = Builder::new();

    // Determine log level based on verbosity flags
    let level = if quiet {
        LevelFilter::Error
    } else {
        match verbose {
            0 => LevelFilter::Warn,
            1 => LevelFilter::Info,
            2 => LevelFilter::Debug,
            _ => LevelFilter::Trace,
        }
    };

    builder.filter_level(level);

    // Configure colored output
    match color {
        ColorChoice::Always => {
            builder.write_style(env_logger::WriteStyle::Always);
        }
        ColorChoice::Never => {
            builder.write_style(env_logger::WriteStyle::Never);
        }
        ColorChoice::Auto => {
            builder.write_style(env_logger::WriteStyle::Auto);
        }
    }

    // Custom format for cleaner output
    builder.format(|buf, record| {
        let level_style = buf.default_level_style(record.level());
        writeln!(
            buf,
            "[{level_style}{}{level_style:#}] {}",
            record.level(),
            record.args()
        )
    });

    // Allow environment variable override
    if let Ok(env_filter) = std::env::var("RUST_LOG") {
        builder.parse_filters(&env_filter);
    }

    builder.init();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init_logging_does_not_panic() {
        // Just ensure initialization doesn't panic
        // We can't test much else without complex setup
        init_logging(false, 0, ColorChoice::Never);
    }
}
