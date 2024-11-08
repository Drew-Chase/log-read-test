use log::{error, info};
use notify::{RecursiveMode, Watcher};
use std::error::Error;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::path::Path;

fn main() {
	// Set the logging level to debug
	std::env::set_var("RUST_LOG", "debug");
	// Initialize the logger
	env_logger::init();
	// Call read_console to process the log file and handle the results
	match read_console(
		"test.txt",
		// Define a callback to print the contents
		|contents| {
			println!("{}", contents);
		}
	)
	{
		Ok(_) => {
			info!("read_console() completed successfully");
		}
		Err(e) => {
			error!("Error: {:?}", e);
		}
	}
}

/// Reads from the given log file and calls the provided callback with its contents.
/// Continues to watch the file for changes and updates the callback with new contents.
///
/// # Arguments
/// * `log_file` - Path to the log file to read from.
/// * `callback` - A closure that takes a string slice and processes the log contents.
///
/// # Returns
/// * `Result<(), Box<dyn Error>>` - Ok() on success, otherwise an error.
pub fn read_console<F>(log_file: impl AsRef<Path>, callback: F) -> Result<(), Box<dyn Error>>
                       where
	                       F: Fn(&str) + Send + 'static,
{
	// Initially, read the contents of the file
	let mut file = File::open(&log_file)?;
	let mut contents = String::new();
	file.read_to_string(&mut contents)?;
	callback(&contents);

	// Track the byte position of the file to read new contents from
	let mut last_read = file.metadata()?.len();

	// Watch the log file for changes
	let (tx, rx) = std::sync::mpsc::channel();
	let mut watcher = notify::recommended_watcher(tx)?;
	watcher.watch(log_file.as_ref(), RecursiveMode::NonRecursive)?;

	// Update the callback with the new contents of the log file
	loop {
		match rx.recv() {
			Ok(event) => match event {
				Ok(_) => {
					// Re-open the file and seek to the last read position
					let mut file = File::open(&log_file)?;

					// Reset if file was truncated
					let size = file.metadata()?.len();
					if last_read > size { last_read = 0; }

					// Seek to the last read position
					file.seek(SeekFrom::Start(last_read))?;

					// Read new contents from the file
					let mut new_contents = String::new();
					file.read_to_string(&mut new_contents)?;

					// If new contents are present, call the callback and update last_read
					if !new_contents.is_empty() {
						callback(&new_contents);
						last_read = file.metadata()?.len(); // Update last_read
					}
				}
				Err(e) => {
					error!("Error: {:?}", e);
					break;
				}
			},
			Err(e) => {
				error!("Error: {:?}", e);
				break;
			}
		}
	}

	Ok(())
}