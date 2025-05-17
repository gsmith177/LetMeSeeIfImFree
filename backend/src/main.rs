mod db;
mod models;

use db::{init_db, get_all_events};
use dotenvy::dotenv;
use eframe::egui;
use models::Event;
use rusqlite::Connection;
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok(); // Load environment variables from `.env`

    let db = init_db()?;

    // -- COMMENTED OUT: API integration not implemented yet --
    // let google_id = env::var("GOOGLE_CLIENT_ID").unwrap_or_else(|_| "MISSING".into());
    // println!("Loaded Google Client ID: {google_id}");

    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Let Me See If I'm Free",
        options,
        Box::new(|_cc| Ok(Box::new(MyApp::new(db)) as Box<dyn eframe::App>)),
    )?;
    Ok(())
}

struct MyApp {
    db: Connection,
    events: Vec<Event>,
}

impl MyApp {
    fn new(db: Connection) -> Self {
        let events = get_all_events(&db).unwrap_or_default();
        Self { db, events }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Welcome to Let Me See If I'm Free!");
            ui.separator();
            for event in &self.events {
                ui.label(format!(
                    "{}: {} to {}",
                    event.title,
                    event.start_time,
                    event.end_time
                ));
            }
        });
    }
}
