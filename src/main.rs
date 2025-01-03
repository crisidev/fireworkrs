use std::{env, fs, path::Path, thread, time::Duration};

use include_dir::{include_dir, Dir};

const FIREWORKS_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/fireworks");
const FIREPLACE_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/fireplace");
const RICKROLL_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/rick_ascii");

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.contains(&String::from("--help")) {
        println!();
        println!("Play text art animations in the terminal\n");
        println!("Usage: fireworkrs [folder] [loops]");
        println!("\t[folder]\tFolder containing text art frames: fireworks | fireplace | rick_ascii (default: fireworks)");
        println!("\t[loops]\t\tNumber of times to loop the animation or use -1 to loop until the user terminates the program (default: 20)");
        println!();
        return;
    }
    let folder_name = args
        .get(1)
        .map_or("fireworks".to_string(), |s| s.to_string());
    let loops: i32 = args.get(2).and_then(|s| s.parse().ok()).unwrap_or(20);

    let mut frames = vec![];
    let mut file_found = true;
    let mut file_number = 0;
    let folder_path = Path::new(&folder_name);
    if folder_path.exists() {
        println!("using folder from path {folder_name}");
        while file_found {
            let f = folder_path.join(format!("{file_number}.txt"));
            if f.exists() {
                let content = fs::read_to_string(f).unwrap_or_else(|_| panic!("unable to read file {file_number}.txt from folder {folder_name} into utf8 string"));
                frames.push(content);
                file_number += 1;
            } else {
                file_found = false;
            }
        }
    } else {
        println!("using embedded folder {folder_name}");
        let folder = match folder_name.as_str() {
            "fireworks" => FIREWORKS_DIR,
            "fireplace" => FIREPLACE_DIR,
            "rick_ascii" => RICKROLL_DIR,
            _ => panic!("folder {folder_name} not available"),
        };

        while file_found {
            if let Some(f) = folder.get_file(format!("{file_number}.txt")) {
                let content = f.contents_utf8().unwrap_or_else(|| panic!("unable to read file {file_number}.txt from folder {folder_name} into utf8 string")).to_string();
                frames.push(content);
                file_number += 1;
            } else {
                file_found = false;
            }
        }
    }

    let mut i = 0;
    let mut first = true;
    let num_lines = frames[0].lines().count();
    let backspace_adjust = "\x1b[A".repeat(num_lines + 1);

    while i < loops || loops == -1 {
        for frame in frames.iter() {
            if !first {
                println!("{backspace_adjust}");
            }
            println!("{frame}");
            first = false;
            thread::sleep(Duration::from_millis(50));
        }
        i += 1;
    }
}
