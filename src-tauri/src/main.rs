mod command;

use rumlibrs::{config::Config, library::LibraryFetcher};
use tokio::sync::Mutex;
use std::sync::Arc;

/// Main entry point of the application.
fn main() {
    // Set environment variable to force hardware acceleration for better performance.
    std::env::set_var("WEBKIT_FORCE_COMPOSITING_MODE", "1");

    // Initialize the configuration and library fetcher.
    let config = Config::new("rum".into());
    let library = LibraryFetcher::new();
    let _ = config.save();

    // Wrap the configuration and library fetcher in Arc<Mutex> for thread-safe access.
    let config = Arc::new(Mutex::new(config));
    let library = Arc::new(Mutex::new(library));

    // Initialize the application context.
    let context = generate_context();

    // Run the application with the given context.
    run_application(context, config, library).expect("error while running application");
}

/// Generates the application context.
///
/// # Returns
///
/// * `Context` - The application context.
fn generate_context() -> Context {
    // Placeholder for the actual context generation logic.
    Context::new()
}

/// Runs the application with the given context and state.
///
/// # Arguments
///
/// * `context` - The application context.
/// * `config` - An `Arc<Mutex<Config>>` to manage the state of the configuration.
/// * `library` - An `Arc<Mutex<LibraryFetcher>>` to manage the state of the library fetcher.
///
/// # Returns
///
/// * `Result<(), String>` - Returns `Ok(())` if successful, otherwise returns an error message.
fn run_application(context: Context, config: Arc<Mutex<Config>>, library: Arc<Mutex<LibraryFetcher>>) -> Result<(), String> {
    // Placeholder for the actual application run logic.
    // This would typically involve setting up event loops, handling commands, etc.
    Ok(())
}

/// Placeholder for the actual context struct.
struct Context {
    // Define fields as needed.
}

impl Context {
    /// Creates a new instance of `Context`.
    fn new() -> Self {
        Context {}
    }
}
