// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(unused)]
mod database;
mod openai;

use database::{init, Chat};
use openai::chat::{self, ChatCompletion};
use rusqlite::{Connection, Error, Result};
use std::{
    process::exit,
    string,
    sync::{Arc, Mutex},
};
use tauri::{Manager, PageLoadPayload, State, Window};

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct AppChat {
    id: String,
    title: String,
    messages: Option<ChatCompletion>,
}

impl From<database::Chat> for AppChat {
    fn from(chat: database::Chat) -> Self {
        Self {
            id: chat.id,
            title: chat.title,
            messages: None,
        }
    }
}

#[derive(Debug)]
struct AppState {
    db: Mutex<Option<Connection>>,
    chats: Mutex<Option<Vec<AppChat>>>,
}

fn main() {
    tauri::Builder::default()
        .manage(AppState {
            db: Default::default(),
            chats: Default::default(),
        })
        .on_page_load(on_page_load)
        .invoke_handler(tauri::generate_handler![
            get_chats,
            get_chat_messages,
            new_chat
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn on_page_load(app: Window, _: PageLoadPayload) {
    let data_dir = app.app_handle().path_resolver().app_data_dir().unwrap();
    let mut state = app.state::<AppState>();

    *state.db.lock().unwrap() = Some(init(&data_dir.to_string_lossy()));
}

#[tauri::command]
fn get_chats(app: Window, state: State<'_, AppState>) -> Result<Vec<AppChat>, String> {
    let binding = state.db.lock().unwrap();
    let db = binding.as_ref().unwrap();

    let chats: std::result::Result<Vec<database::Chat>, Error> = Chat::load_all(&db);

    return chats
        .map(|chats| chats.into_iter().map(|chat| chat.into()).collect())
        .map_err(|err| err.to_string());
}

#[tauri::command]
fn get_chat_messages(
    chat_id: &str,
    app: Window,
    state: State<'_, AppState>,
) -> Result<ChatCompletion, String> {
    let binding = state.db.lock().unwrap();
    let db = binding.as_ref().unwrap();

    let messages = Chat::load(&db, chat_id).map_err(|e| e.to_string())?;

    Ok(ChatCompletion::from(messages))
}

fn message_chat() {
    unimplemented!();
}

#[tauri::command]
fn new_chat(app: Window, state: State<'_, AppState>) -> Result<AppChat, String> {
    let binding = state.db.lock().unwrap();
    let db = binding.as_ref().unwrap();

    let new_chat = Chat::new(db).map_err(|e| e.to_string())?;

    println!("New chat: {:?}", new_chat);

    Ok(AppChat::from(new_chat))
}
