use std::fs::create_dir_all;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::Write;
use std::path::Path;

use curl::easy::Easy;

use dirs::home_dir;

fn check_ssh_config(ssh_dir: &Path) -> bool {
	let mut file = match OpenOptions::new()
		.read(true)
		.open(ssh_dir.join("config"))
	{
		Ok(file) => file,
		Err(_) => return false,
	};

	let to_match = "Include hkconfig";
	let mut reader = BufReader::new(file);
    let lines = reader.lines();
	for line_result in lines {
		let line = match line_result {
			Ok(line) => line,
			Err(_) => return false,
		};

		if line == to_match {
			return true;
		}
	}
	return false;
}

fn main() {
	let ssh_dir = home_dir().unwrap().join(".ssh");

	if !check_ssh_config(&ssh_dir) {
		println!("heads up: you're missing the line \"Include hkconfig\" from ~/.ssh/config");
	}

	create_dir_all(ssh_dir.join("hkdb")).unwrap();

	let mut file = OpenOptions::new()
		.read(true)
		.write(true)
		.create(true)
		.truncate(true)
		.open(ssh_dir.join("hkdb").join("ocf.berkeley.edu")).unwrap();


	let mut easy = Easy::new();
	easy.url("https://failure.ocf.berkeley.edu/ssh_known_hosts").unwrap();
	let mut transfer = easy.transfer();
	transfer.write_function(|data| {
		Ok(file.write(data).unwrap())
	}).unwrap();
	transfer.perform().unwrap();
}
