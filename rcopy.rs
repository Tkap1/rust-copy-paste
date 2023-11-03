
use std::env;
use std::process;
use std::fs;

fn main()
{
	let args: Vec<String> = env::args().collect();

	if args.len() <= 1 {
		print_usage_and_exit();
	}

	if args[1] == "copy" {

		if args.len() < 3 {
			print_usage_and_exit();
		}

		clear_temp_directory();
		let dst = str_to_file_name(&args[2]);

		if let Err(foo) = fs::copy(&args[2], format!("temp/{}", dst)) {
			println!("Error: {}", foo);
			process::exit(1);
		}
		else {
			println!("Copied {}", args[2]);
		}
	}

	else if args[1] == "cut" {

		if args.len() < 3 {
			print_usage_and_exit();
		}

		clear_temp_directory();

		let dst = str_to_file_name(&args[2]);

		if let Err(foo) = fs::rename(&args[2], format!("temp/{}", dst)) {
			println!("Error: {}", foo);
			process::exit(1);
		}
		else {
			println!("Cut {}", args[2]);
		}
	}

	else if args[1] == "paste" {
		let exe_folder = get_exe_folder();
		let mut paths = fs::read_dir(format!("{}/temp", exe_folder.display())).unwrap();
		let file_path = paths.next();
		if file_path.is_none() {
			println!("Error: There is nothing to paste");
			process::exit(1);
		}
		let file_name_ = file_path.unwrap().unwrap().file_name();
		let file_name = file_name_.to_string_lossy();
		let mut target = file_name.clone();
		if args.len() >= 3 {
			target = args[2].clone().into();
		}
		let source = format!("{}/temp/{}", exe_folder.display(), &file_name);

		if let Err(foo) = fs::copy(source, &*target) {
			println!("Error: {}", foo);
			process::exit(1);
		}
		println!("Pasted {}", target);
	}

	else {
		print_usage_and_exit();
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

fn get_exe_folder() -> std::path::PathBuf
{
	let exe_path = std::env::current_exe().unwrap();
	let exe_folder = exe_path.parent().unwrap();
	return exe_folder.to_owned();
}

fn clear_temp_directory()
{
	let exe_folder = get_exe_folder();
	let path = format!("{}/temp", exe_folder.display());
	fs::remove_dir_all(&path).unwrap();
	fs::create_dir(path).unwrap();
}

fn str_to_file_name(str: &String) -> String
{
	let path = std::path::PathBuf::from(&str);
	let dst_ = path.file_name().unwrap();
	let dst = dst_.to_string_lossy();
	return dst.to_string();
}