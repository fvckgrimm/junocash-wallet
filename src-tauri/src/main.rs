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
use tauri::{
    menu::{Menu, MenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    Manager, RunEvent, WindowEvent,
};

fn main() {
    let app = tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_dialog::init())
        .manage(NodeState::new())
        .setup(|app| {
            // Create tray menu items
            let quit = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
            let show = MenuItem::with_id(app, "show", "Show Window", true, None::<&str>)?;
            let hide = MenuItem::with_id(app, "hide", "Hide Window", true, None::<&str>)?;

            // Create the menu
            let menu = Menu::with_items(app, &[&show, &hide, &quit])?;

            // Build the tray icon
            let _tray = TrayIconBuilder::with_id("main-tray")
                .tooltip("Juno Cash Wallet")
                .icon(app.default_window_icon().unwrap().clone())
                .menu(&menu)
                .show_menu_on_left_click(false)
                .on_menu_event(move |app, event| match event.id.as_ref() {
                    "quit" => app.exit(0),
                    "show" => {
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                    }
                    "hide" => {
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.hide();
                        }
                    }
                    _ => {}
                })
                .on_tray_icon_event(|tray, event| {
                    if let TrayIconEvent::Click {
                        button: MouseButton::Left,
                        button_state: MouseButtonState::Up,
                        ..
                    } = event
                    {
                        let app = tray.app_handle();
                        if let Some(webview_window) = app.get_webview_window("main") {
                            if webview_window.is_visible().unwrap_or(false) {
                                let _ = webview_window.hide();
                            } else {
                                let _ = webview_window.show();
                                let _ = webview_window.set_focus();
                            }
                        }
                    }
                })
                .build(app)?;

            Ok(())
        })
        .on_window_event(|window, event| {
            // Intercept close event and hide window instead
            if let WindowEvent::CloseRequested { api, .. } = event {
                window.hide().unwrap();
                api.prevent_close();
            }
        })
        .invoke_handler(tauri::generate_handler![
            // Node Launcher
            node::launch_node,
            node::stop_node,
            node::get_default_juno_paths,
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
            wallet::get_spendable_addresses,
            wallet::get_operation_status,
            wallet::get_blockchain_info,
        ])
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
