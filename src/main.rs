use std::fs;
use std::process::Command;

const INPUT_DIRECTORY: &str = "/input/";
const OUTPUT_DIRECTORY: &str = "/output/";

fn main() {
    let input_dirs = fs::read_dir(INPUT_DIRECTORY).unwrap();

    for dir in input_dirs {
        let current_path = dir.unwrap().path().display().to_string();

        let mut zip_path = current_path.clone();
        zip_path.push_str(".zip");

        let spawn = Command::new("zip")
            .arg("-r")
            .arg(zip_path.clone())
            .arg(current_path)
            .spawn();

        match spawn.unwrap().wait() {
            Ok(_) => {
                Command::new("mv")
                    .arg(zip_path)
                    .arg(OUTPUT_DIRECTORY)
                    .spawn()
                    .expect("some error occur while move the zip file");
            }
            Err(e) => println!("{:?}", e),
        }
    }
}
