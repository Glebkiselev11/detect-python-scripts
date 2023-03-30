use core::time;
use std::path::Path;
use std::{fs, thread};
use sysinfo::{ProcessExt, System, SystemExt};
use walkdir::WalkDir;

fn main() {
    let system = System::new_all();

    let mut found_running_scripts: Vec<String> = vec![];

    for (_pid, process) in system.processes() {
        let name = process.name();
        if name == "Python" || name == "python" || name == "python.exe" {
            println!("\n\n\n");

            let cmdline = process.cmd();
            println!("Command line: {:?}", cmdline);

            let script_name = cmdline.iter().find(|arg| arg.ends_with(".py"));
            match script_name {
                Some(script) => {
                    let script = script.replace(".\\", "");
                    println!("Script name: {}", script);
                    found_running_scripts.push(script.clone());

                    let path = format!("{:?}", process.cwd())
                        .replace('"', "")
                        .replace("\\\\", "/");

                    println!("Path to script: {path}",);

                    let script_path = find_script(&path, &script);

                    match script_path {
                        Some(path) => match fs::read_to_string(&path) {
                            Ok(content) => {
                                println!(
                                    "\n\n--------------------------------Script-Content-------------------------------------------\n\n{}",
                                    content
                                );
                            }
                            Err(err) => {
                                println!("Failed to read script: {}", err);
                            }
                        },
                        None => println!("Script not found in the specified search directory"),
                    }
                }
                None => println!("No script found in command line"),
            }
        }
    }

    println!(
        "\n\n--------------------------------Statistic-----------------------------------------------"
    );

    println!("Found script files: ");

    found_running_scripts.iter().for_each(|x| println!("{x}"));

    // It is here for testing on windows - remove it
    thread::sleep(time::Duration::from_secs(100));
}

fn find_script(search_dir: &str, script_name: &str) -> Option<String> {
    let search_dir = Path::new(search_dir);
    for entry in WalkDir::new(search_dir) {
        let entry = match entry {
            Ok(entry) => entry,
            Err(_) => continue,
        };
        if entry.file_name().to_string_lossy() == script_name {
            return Some(entry.path().to_string_lossy().to_string());
        }
    }
    None
}
