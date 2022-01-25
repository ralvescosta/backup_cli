use clap::Parser;
use std::fs;
use std::process::Command;
use std::str::FromStr;

const DEFAULT_INPUT_DIRECTORY: &str = "/home/rafael/Desktop/AA/";
const DEFAULT_OUTPUT_DIRECTORY: &str = "/home/rafael/Desktop/BB/";

fn main() {
    let params = cli();

    application(params);
}

struct Params {
    pub use_default: String,
    pub input: String,
    pub output: String,
}

fn cli() -> Params {
    #[derive(Parser, Debug)]
    #[clap(author, version, about, long_about = None)]
    struct Args {
        #[clap(short, long, default_value = "y")]
        use_default: String,

        #[clap(short, long, default_value = "")]
        input: String,

        #[clap(short, long, default_value = "")]
        output: String,
    }

    let args = Args::parse();

    println!("{:?}", args);

    Params {
        use_default: args.use_default,
        input: args.input,
        output: args.output,
    }
}

fn application(params: Params) {
    let (input, output) = match params.use_default.as_str() {
        "y" | "Y" | "yes" | "Yes" | "YES" => (DEFAULT_INPUT_DIRECTORY, DEFAULT_OUTPUT_DIRECTORY),
        _ => (params.input.as_str(), params.output.as_str()),
    };

    let input_dirs = fs::read_dir(input).unwrap();

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
                let mut expected_zipped_path = String::from_str(output).unwrap();
                expected_zipped_path.push_str(expected_zip_folder);

                let remove_command_spawn = Command::new("rm")
                    .arg("-r")
                    .arg(expected_zipped_path)
                    .spawn();
                match remove_command_spawn.unwrap().wait() {
                    Ok(_) => {
                        Command::new("mv")
                            .arg(zip_path)
                            .arg(output)
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
