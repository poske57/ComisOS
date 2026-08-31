mod settings;
mod initialize;
use dialoguer;

fn main() -> std::io::Result<()> {
    header();
    let settings = settings::Settings::load();
    initialize::initialize(&settings.repositories_path)?;

    let choices = &["Chatを起動する", "設定を確認する", "巻き戻す"];
    let choice: usize = dialoguer::Select::new()
    .items(choices)
    .default(0)
    .interact()?;

    Ok(())
}

fn header() {
    println!("{}", include_str!("../assets/logo.txt"));
}
