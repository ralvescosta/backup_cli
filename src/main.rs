use std::fs;
use std::process::Command;
use std::str::FromStr;

const INPUT_DIRECTORY: &str = "/input/";
const OUTPUT_DIRECTORY: &str = "/output/";

fn main() {
    let input_dirs = fs::read_dir(INPUT_DIRECTORY).unwrap();

    for dir in input_dirs {
        let current_path = dir.unwrap().path().display().to_string();
        let mut zip_path = current_path.clone();
        zip_path.push_str(".zip");

        let zip_command_spawn = Command::new("zip")
            .arg("-r")
            .arg(zip_path.clone())
            .arg(current_path)
            .spawn();
        match zip_command_spawn.unwrap().wait() {
            Ok(_) => {
                let expected_zip_folder = zip_path.split("/").last().unwrap();
                let mut expected_zipped_path = String::from_str(OUTPUT_DIRECTORY).unwrap();
                expected_zipped_path.push_str(expected_zip_folder);

                let remove_command_spawn = Command::new("rm")
                    .arg("-r")
                    .arg(expected_zipped_path)
                    .spawn();
                match remove_command_spawn.unwrap().wait() {
                    Ok(_) => {
                        Command::new("mv")
                            .arg(zip_path)
                            .arg(OUTPUT_DIRECTORY)
                            .spawn()
                            .expect("[Err] - some error occur while move the zip");
                    }
                    Err(e) => println!("[Err] - error while try to perform rm command - {:?}", e),
                }
            }
            Err(e) => println!("[Err] - error while zip folder - {:?}", e),
        }
    }
}
