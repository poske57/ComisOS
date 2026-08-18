mod settings;
mod initialize;

fn main() -> std::io::Result<()> {
    let settings = settings::Settings::load();
    initialize::initialize(&settings.repositories_path)?;
    Ok(())
}
