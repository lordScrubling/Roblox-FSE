#![windows_subsystem = "windows"]

use std::{
	fs,
	io::ErrorKind,
};
use home::home_dir;

// maybe one day i'll figure out how to hook it to roblox updating w/ shim
fn main() -> std::io::Result<()>{
	let mut path = home_dir().unwrap();
	path.push("AppData\\Local\\Roblox\\Versions");
	let mut dirs = fs::read_dir(path)?
		.flatten()
		.filter(|dir| dir.file_type().unwrap().is_dir());

	let mut last = dirs.next().expect("no dirs");
	for dir in dirs {
		if dir.metadata()?.modified()? > last.metadata()?.modified()? {
			last = dir;
		}
	}

	let mut path = last.path().clone();
	path.push("ClientSettings");
	if let Some(err) = fs::create_dir(&path).err() {
		match err.kind() {
			ErrorKind::AlreadyExists => return Ok(()),
			_ => return Err(err),
		}
	}
	path.push("ClientAppSettings.json");
	fs::write(path, "{\"FFlagHandleAltEnterFullscreenManually\":\"False\"}")?;

	Ok(())
}