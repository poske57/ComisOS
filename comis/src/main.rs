mod settings;

fn main() {
    let settings = settings::Settings::load().expect("");
}
