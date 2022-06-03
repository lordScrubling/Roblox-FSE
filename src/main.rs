use std::fs;

// maybe one day i'll figure out how to hook it to roblox updating
fn main() -> std::io::Result<()>{
	let mut dirs = fs::read_dir("C:\\Users\\Leo\\AppData\\Local\\Roblox\\Versions")?
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
	fs::create_dir(&path)?;
	path.push("ClientAppSettings.json");
	fs::copy("C:\\Users\\Leo\\Documents\\ClientAppSettings.json", path)?;

	Ok(())
}