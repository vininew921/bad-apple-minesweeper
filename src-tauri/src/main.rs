// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::Mutex;

use game::Game;

pub mod game;

#[tauri::command]
fn next_frame(game: tauri::State<'_, MutexGame>) -> Vec<bool> {
    game.0.lock().unwrap().advance_frame();
    game.0.lock().unwrap().to_owned().get_current_frame()
}

struct MutexGame(Mutex<Game>);

fn main() {
    let mut game = Game::new();
    game.initialize_frames();

    tauri::Builder::default()
        .manage(MutexGame(Mutex::from(game)))
        .invoke_handler(tauri::generate_handler![next_frame])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
