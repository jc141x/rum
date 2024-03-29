//Conditional compilation attribute. If the target OS is windows and debug assertions are not enabled, then the windows subsystem is set to windows.
#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod command;

use rumlibrs::{config::Config, library::LibraryFetcher};
use tauri::async_runtime::Mutex;

fn main() {
    /* This variable is set to 1, which means that when a page is loaded, it will be forced to use hardware acceleration. 
    This will improve the speed and performance of the application as it allows for better graphics rendering.
    */
    std::env::set_var("WEBKIT_FORCE_COMPOSITING_MODE", "1");
    let config = Config::new("rum".into());

    let library = LibraryFetcher::new();
    let _ = config.save();

    tauri::Builder::default()
        /* A Mutex is used to make sure that only one thread can access a shared resource at a time
        This prevents any race conditions (undesirable situation that occurs when a device or system attempts to perform two or more operations at the same time) 
        that may occur if multiple threads were to access the same resource at the same time.
        */
        .manage(Mutex::new(library)) // Lock the "library" variable in a mutex and manage it with the builder
        .manage(Mutex::new(config))  // Lock the "config" variable in a mutex and manage it with the builder
        /* A handler is used to bind functions to an application so that they can be executed when the application is run.
        This allows the application to access and modify various settings related to the library, config, and other miscellaneous functions. 
        It ensures that the necessary functions are executed when the application is run and the necessary data is accessible. 
        */
        .invoke_handler(tauri::generate_handler![ // invokes the handler for the following commands:
            // Library
            command::library::library_get_games,
            command::library::library_reload_games,
            command::library::library_run_game,
            command::library::library_open_terminal,
            command::library::library_open_folder,
            command::library::library_set_banner,
            command::library::library_remove_banner,
            command::library::library_save_game_config,
            command::library::library_read_game_config,
            // Config
            command::config::config_save,
            command::config::config_set,
            command::config::config_set_data_path,
            command::config::config_set_library_paths,
            command::config::config_set_terminal,
            command::config::config_get,
            command::config::config_get_data_path,
            command::config::config_get_library_paths,
            command::config::config_get_terminal,
            // Misc
            command::misc_get_reqs_markdown,
            command::misc_get_wiki_page,
            command::misc_init_bg_process,
        ])
         /* Run the tauri application with the given context. 
         This method takes the context that was generated by the tauri::generate_context!() macro, and runs the application with it. 
         The context generated by the macro contains information about the application such as window size, event loop, etc. 
         This information is necessary for the application to run correctly, and thus must be provided when running the application.
         */
        .run(tauri::generate_context!())
        /* Defines the expected outcome of a function. The expect statement is used to ensure that the tauri application runs without any errors. 
        If an error does occur, the expect statement will stop the application and throw an error with the given message. 
        */
        .expect("error while running tauri application");
}
