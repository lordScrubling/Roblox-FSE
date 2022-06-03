use std::process::Command;

// maybe one day i'll figure out how to hook it to roblox updating
fn main() {
	let mut ls = Command::new("C:\\msys64\\usr\\bin\\sh.exe")
		.args(["-c", "ls C:/Users/Leo/AppData/Local/Roblox/Versions/* -dt"])
		.output()
		.unwrap();

	let mut i = 0;
	while i < ls.stdout.len() && ls.stdout[i] != b'\n' {
		i += 1;
	}
	ls.stdout.resize(i, 0);

	Command::new("cp")
		.args(["C:\\Users\\Leo\\Documents\\ClientSettings", unsafe { std::str::from_utf8_unchecked(&ls.stdout) }, "-r"])
		.status()
		.unwrap();
}
