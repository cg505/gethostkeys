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

	let mut file = match OpenOptions::new()
		.read(true)
		.open(ssh_dir.join("hkconfig"))
	{
		Ok(file) => file,
		Err(_) => {
			println!("wtf");
			return;
		},
	};

	let mut reader = BufReader::new(file);

	'file_scan: loop {
		println!("top loop");
		let mut line = String::new();
		// search forward to Host line
		loop {
			if reader.read_line(&mut line).unwrap() == 0 {
				break 'file_scan;
			}
			if line.starts_with("Host ") {
				// next line should contain dest url
				break;
			}
		}
		line = String::new();
		if reader.read_line(&mut line).unwrap() == 0 {
			break 'file_scan;
		}
		let mut parts = line.splitn(2, "#");
		if match parts.next() {
			Some(space) => space.trim().len() != 0,
			None => true,
		} {
			// todo: recover if this line is a Host line
			println!("unable to parse hkconfig file: no comment after Host line");
			break 'file_scan;
		}
		// lmao refactor plzzz
		let url = match parts.next() {
			Some(part) => part.trim(),
			None => {
				println!("where is the url dummy");
				break 'file_scan;
			},
		};
		let mut dest = String::new();
		loop {
			let mut line = String::new();
			if reader.read_line(&mut line).unwrap() == 0 {
				println!("ended looking for dest");
				break 'file_scan;
			}
			println!("now parsing config line: {}", line);
			match line.chars().next() {
				None => continue,
				Some(car) => if !car.is_whitespace() {
					println!("where is the config! dummy");
					break 'file_scan;
				} else {
					// todo: quotes? idk
					let mut parts = line.split_whitespace();
					match parts.next() {
						None => continue,
						Some(first) => if first != "UserKnownHostsFile" {
							println!("wrong config value");
							continue;
						},
					};
					println!("found right line");
					match parts.next() {
						None => continue, // wtf
						Some(second) => {
							println!("it's {}!", second);
							dest = second.to_string();
							break;
						},
					};
				}
			};
		}
		if !dest.starts_with("hkdb/") {
			println!("heads up! stuff not going in hkdb");
			// todo confirm mb
		}

		println!("fetching from {} to {}", url, dest);

		let mut file = OpenOptions::new()
			.read(true)
			.write(true)
			.create(true)
			.truncate(true)
			.open(ssh_dir.join(dest)).unwrap();


		let mut easy = Easy::new();
		easy.url(url).unwrap();
		let mut transfer = easy.transfer();
		transfer.write_function(|data| {
			Ok(file.write(data).unwrap())
		}).unwrap();
		transfer.perform().unwrap();
	}
	println!("top loop done");
}
