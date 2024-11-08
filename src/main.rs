use log::{error, info};
use notify::{RecursiveMode, Watcher};
use std::error::Error;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::path::Path;

fn main() {
	std::env::set_var("RUST_LOG", "debug");
	env_logger::init();
	match read_console(
		"test.txt",
		|contents|
			{
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


pub fn read_console<F>(log_file: impl AsRef<Path>, callback: F) -> Result<(), Box<dyn Error>>
                       where
	                       F: Fn(&str) + Send + 'static,
{
	// Initially, read the contents of the file
	let mut file = File::open(&log_file)?;
	let mut contents = String::new();
	file.read_to_string(&mut contents)?;
	callback(&contents);

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
					let mut file = File::open(&log_file)?;
					file.seek(SeekFrom::Start(last_read))?;

					let mut new_contents = String::new();
					file.read_to_string(&mut new_contents)?;

					if !new_contents.is_empty() {
						callback(&new_contents);
						last_read += new_contents.len() as u64;
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