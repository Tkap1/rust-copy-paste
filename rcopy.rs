
use std::env;
use std::io;
use std::process;
use std::fs;
use std::path::PathBuf;

fn main()
{
	let args: Vec<String> = env::args().collect();

	if args.len() <= 1 {
		print_usage_and_exit();
	}

	match args[1].as_str() {
		"copy" => {
			if args.len() < 3 {
				print_usage_and_exit();
			}

			clear_temp_directory();
			let dst = file_name_from_path(&args[2]);

			if let Err(foo) = fs::copy(&args[2], format!("temp/{}", dst)) {
				println!("Error: {}", foo);
				process::exit(1);
			}
			else {
				println!("Copied {}", args[2]);
			}
		},
		"cut" => {
			if args.len() < 3 {
				print_usage_and_exit();
			}

			clear_temp_directory();

			let dst = file_name_from_path(&args[2]);

			if let Err(foo) = fs::rename(&args[2], format!("temp/{}", dst)) {
				println!("Error: {}", foo);
				process::exit(1);
			}
			else {
				println!("Cut {}", args[2]);
			}
		},
		"paste" => {
			let exe_folder = get_exe_folder();
			let mut paths = fs::read_dir(format!("{}/temp", exe_folder.display())).unwrap();
			let Some(path) = paths.next() else {
				println!("Error: There is nothing to paste");
				process::exit(1);
			};
			let file_name = path.unwrap().file_name();
			let mut target = file_name.to_string_lossy();
			if args.len() >= 3 {
				target = args[2].clone().into();
			}
			let source = format!("{}/temp/{}", exe_folder.display(), &target);

			if let Err(foo) = fs::copy(source, &*target) {
				println!("Error: {}", foo);
				process::exit(1);
			}
			println!("Pasted {}", target);
		},
		_ => { print_usage_and_exit(); }
	}
}

fn print_usage()
{
	println!("Usage:");
	println!("\trcopy copy filepath");
	println!("\trcopy cut filepath");
	println!("\trcopy paste [filepath]");
}

fn print_usage_and_exit()
{
	print_usage();
	process::exit(1);
}

fn get_exe_folder() -> PathBuf
{
	let exe_path = env::current_exe().unwrap();
	let exe_folder = exe_path.parent().unwrap();
	return exe_folder.to_owned();
}

fn clear_temp_directory()
{
	let exe_folder = get_exe_folder();
	let path = format!("{}/temp", exe_folder.display());
	if let Err(err) = fs::remove_dir_all(&path) {
		if err.kind() != io::ErrorKind::NotFound {
			println!("Error clearing temp directory: {}", err);
			process::exit(1);
		}
	}
	fs::create_dir(path).unwrap();
}

fn file_name_from_path(path: &str) -> String
{
	let path = PathBuf::from(path);
	return path.file_name().unwrap().to_string_lossy().to_string();
}
