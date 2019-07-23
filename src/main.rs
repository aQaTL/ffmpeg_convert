#![feature(bind_by_move_pattern_guards)]

use rayon::prelude::*;
use std::{fs, io, process::Command};

const USAGE_STR: &'static str = r#"USAGE: 
	ffmpeg_convert [output_format]"#;

fn main() -> io::Result<()> {
	let args = std::env::args().skip(1).collect::<Vec<_>>();
	if args.len() == 0 {
		eprintln!("{}", USAGE_STR);
		return Ok(());
	}

	let dir = fs::read_dir(std::env::current_dir().unwrap())?.filter_map(|e| e.ok()).collect::<Vec<_>>();
	dir.par_iter().for_each(|entry| {
		match entry.metadata() {
			Ok(m) if m.is_file() => (),
			_ => return, // Skip if it's not a file
		}

		let file_path = entry.path();

		Command::new("ffmpeg")
			.arg("-i")
			.arg(file_path
						 .file_name()
						 .map_or(Some(""), |f| f.to_str())
						 .unwrap_or_default())
			.arg(file_path
						 .with_extension(&args[0])
						 .file_name()
						 .map_or(Some(""), |f| f.to_str())
						 .unwrap_or_default())
			.spawn()
			.expect("Failed to execute command")
			.wait()
			.expect("Failed to wait for ffmpeg");

		fs::remove_file(file_path).expect("Failed to remove old file");
	});

	Ok(())
}
