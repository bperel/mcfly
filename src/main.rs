extern crate bash_wizard;

use bash_wizard::interface::Interface;
use bash_wizard::history::History;
use bash_wizard::settings::Settings;
use bash_wizard::settings::Mode;
use bash_wizard::fake_typer;

fn handle_addition(settings: &Settings, history: &mut History) {
    if !settings.command.starts_with('#') { // Ignore commented lines
        history.add(&settings.command, &settings.when, &settings.exit_code, &settings.dir, &settings.old_dir);
    }
}

fn handle_search(settings: &Settings, history: &History) {
    let command = Interface::new(settings, history).select();
    if command.len() > 0 {
        fake_typer::use_tiocsti(&command);
        fake_typer::use_tiocsti(&"\n".to_string());
    }
}

fn main() {
    let settings = Settings::parse_args();

    let mut history = History::load();

    match settings.mode {
        Mode::Add => {
            handle_addition(&settings, &mut history);
        },
        Mode::Search => {
            handle_search(&settings, &history);
        }
    }
}