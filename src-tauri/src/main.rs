// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod rpc;
mod state;
mod commands {
    pub mod mining;
    pub mod node;
    pub mod wallet;
}

use commands::{mining, node, wallet};
use state::NodeState;
use tauri::{Manager, RunEvent};

fn main() {
    let app = tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_dialog::init())
        .manage(NodeState::new())
        .invoke_handler(tauri::generate_handler![
            // Node Launcher
            node::launch_node,
            node::stop_node,
            node::get_default_juno_paths, // This will work now
            // Mining
            mining::get_mining_info,
            mining::get_block_subsidy,
            mining::set_mining,
            mining::shield_coinbase,
            // Wallet
            wallet::get_balance,
            wallet::list_transactions,
            wallet::send_transaction,
            wallet::get_new_address,
            wallet::get_block_count,
            wallet::get_all_addresses,
        ])
        //.run(tauri::generate_context!())
        .build(tauri::generate_context!())
        .expect("error while running tauri application");

    app.run(|app_handle, event| {
        match event {
            RunEvent::ExitRequested { .. } => {
                // The app is about to close.
                // Get the state, lock the mutex, and kill the child process if it exists.
                let state = app_handle.state::<NodeState>();
                let mut guard = state.process.lock().unwrap();

                if let Some(mut child) = guard.take() {
                    println!("Tauri is exiting. Killing Juno Node...");
                    let _ = child.kill(); // Force kill the process
                }
            }
            _ => {}
        }
    });
}
