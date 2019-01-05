use std::fs::OpenOptions;
use std::io::Write;

use curl::easy::Easy;

fn main() {
	// let path = Path::new("/home/cooperc/.ssh/ghdb");
	// let display = path.display();
	let mut file = OpenOptions::new()
		.read(true)
		.write(true)
		.create(true)
		.truncate(true)
		.open("/home/cooperc/.ssh/hkdb/ocf.berkeley.edu").unwrap();


	let mut easy = Easy::new();
	easy.url("https://failure.ocf.berkeley.edu/ssh_known_hosts").unwrap();
	let mut transfer = easy.transfer();
	transfer.write_function(|data| {
		Ok(file.write(data).unwrap())
	}).unwrap();
	transfer.perform().unwrap();
}
