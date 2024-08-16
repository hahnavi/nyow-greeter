mod app_config;
mod callbacks;
mod greetd;
mod helpers;
mod setup_ui;
mod util;
mod vars;
mod widgets;

use clap::Parser;
use gtk::{prelude::*, Application, Settings};
use log::error;
use setup_ui::setup_ui;
use simplelog::{
    ColorChoice, CombinedLogger, Config as LogConfig, LevelFilter, TermLogger, TerminalMode,
    WriteLogger,
};
use std::{fs::OpenOptions, str::FromStr};
use vars::{SELECTED_USER, SESSIONS, USERS};

#[derive(Parser, Debug)]
#[command(version)]
struct Args {
    #[arg(short, long, default_value = "/etc/greetd/nyow-greeter.toml")]
    config: String,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let config = app_config::load_config(&args.config);

    let mut loggers: Vec<Box<dyn simplelog::SharedLogger>> = vec![TermLogger::new(
        LevelFilter::from_str(&config.log_level).unwrap(),
        LogConfig::default(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    )];

    match OpenOptions::new()
        .append(true)
        .create(true)
        .open(&config.log_file)
    {
        Ok(file) => {
            loggers.push(WriteLogger::new(
                LevelFilter::from_str(&config.log_level).unwrap(),
                LogConfig::default(),
                file,
            ));
        }
        Err(err) => {
            eprintln!("Failed to open log file: {:?}", err);
        }
    }

    CombinedLogger::init(loggers).unwrap();

    if let Err(err) = USERS.set(util::get_user_list()) {
        error!("Failed to set USERS: {:?}", err);
    }

    if let Err(err) = SESSIONS.set(util::get_session_types()) {
        error!("Failed to set SESSIONS: {:?}", err);
    }

    if let Err(err) = SELECTED_USER.set(0.into()) {
        error!("Failed to set SELECTED_USER: {:?}", err);
    }

    let theme = config.theme.to_string();
    let icon_theme = config.icon_theme.to_string();
    let font: String = config.font.to_string();

    let application = Application::builder()
        .application_id("id.my.nyow.greeter")
        .build();

    application.connect_activate(move |app| {
        if let Some(settings) = Settings::default() {
            settings.set_property("gtk-theme-name", theme.clone());
            settings.set_property("gtk-icon-theme-name", icon_theme.clone());
            settings.set_property("gtk-font-name", font.clone());
        }
        if let Err(err) = setup_ui(app) {
            error!("Failed to setup UI: {:?}", err);
        }
    });

    application.run_with_args::<&str>(&[]);
}
