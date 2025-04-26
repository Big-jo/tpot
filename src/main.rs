mod app;
mod events;
mod models;

use app::App;

use serde::{Deserialize, Serialize};
use std::{io, sync::mpsc, thread, time::Duration};
use events::event::Event;
use models::{
    task_data, user_data::UserData
};

fn handle_input_events(tx: mpsc::Sender<Event>) {
    loop {
        match crossterm::event::read().unwrap() {
            crossterm::event::Event::Key(key_event) => tx.send(Event::Input(key_event)).unwrap(),
            _ => {}
        }
    }
}

fn run_background_thread(tx: mpsc::Sender<Event>) {
    let mut progress = 0_f64;
    let increment = 0.01_f64;
    loop {
        std::thread::sleep(Duration::from_millis(100));
        progress += increment;
        progress = progress.min(1_f64);
        tx.send(Event::Progress(progress)).unwrap();
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ActivityData {
    friends: Vec<UserData>,
    profile: UserData,
}

// ====== MAIN FUNCTION ======
fn main() -> io::Result<()> {
    // Initialize terminal
    let mut terminal = ratatui::init();

    // Load data from JSON file
    let file = std::fs::File::open("./data/db.json")?;
    let data: ActivityData = serde_json::from_reader(file)?;

    // Create app instance
    let mut app = App::new(data);

    // Set up event channels
    let (event_tx, event_rx) = mpsc::channel::<Event>();

    // Spawn input handling thread
    let tx_for_input = event_tx.clone();
    thread::spawn(move || {
        handle_input_events(tx_for_input);
    });

    // Spawn background processing thread
    let tx_for_background = event_tx.clone();
    thread::spawn(move || {
        run_background_thread(tx_for_background);
    });

    // Run the app
    let app_result = app.run(&mut terminal, event_rx);

    // Restore terminal
    ratatui::restore();

    app_result
}
