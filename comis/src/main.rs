mod settings;
mod initialize;
use dialoguer;
use std::process::Command;
use std::env;

fn main() -> std::io::Result<()> {
    header();
    let settings = settings::Settings::load();
    initialize::initialize(&settings.repositories_path)?;

    env::set_current_dir(&settings.repositories_path)?;
    let choices = &["Chatを起動する", "設定", "終了"];
    let choice: usize = dialoguer::Select::new()
    .items(choices)
    .default(0)
    .interact()?;

    match choice {
        0 => launch_chat(&settings.agent),
        1 => config_menu(&settings),
        _ => println!("終了します"),
    }

    Ok(())
}

fn header() {
    println!("{}\n", include_str!("../assets/logo.txt"));
}

fn launch_chat(ai_agent: &String) {
    println!("AI Agent を起動");
    let status = Command::new(ai_agent)
        .status();
    if status.is_err() {
        println!("AI Agent not found");
    }
}

fn config_menu(settings: &settings::Settings) {
    println!("Path    : {}", settings.repositories_path.display());
    println!("AI agent: {}", settings.agent);
    println!("rebuild-command: {}", settings.rebuild_command);
}
