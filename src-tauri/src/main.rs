// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod printing;

fn main() {
  tauri::Builder::default()
    .run(tauri::generate_context!())
    .expect("error while running tauri application");

    println!("I was invoked from JS!");
      let _ = printing::test();

  
}

#[tauri::command]
fn my_custom_command() {
  println!("I was invoked from JS!");
  let _ = printing::test();
}