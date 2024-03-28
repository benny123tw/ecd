use clap::{Parser, Subcommand};
use std::{
    fs::read_dir,
    path::{Path, PathBuf},
    process::Command,
};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    commands: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Run exercism commands to download exercises
    Exercism {
        /// Sub-Commands
        #[command(subcommand)]
        subcommands: Option<SubCommands>,
    },

    /// Submit the exercise automatically
    Submit {
        /// Specifies the track or it will automatically detect the track
        #[arg(short, long)]
        track: Option<String>,
    },
}

#[derive(Subcommand)]
enum SubCommands {
    /// Downloading the exercise
    Download {
        /// Indicates the language track
        #[arg(short, long)]
        track: String,

        /// Indicates the exercise
        #[arg(short, long)]
        exercise: String,

        /// Force download
        #[arg(short, long)]
        force: bool,
    },
}

fn main() {
    let cli = Cli::parse();

    match &cli.commands {
        Some(Commands::Exercism { subcommands }) => match subcommands {
            Some(SubCommands::Download {
                track,
                exercise,
                force,
            }) => {
                println!(
                    "Downloading the exercise: {} from the track: {}",
                    exercise, track
                );

                let mut command = Command::new("exercism");
                command
                    .arg("download")
                    .arg(format!("--track={}", track))
                    .arg(format!("--exercise={}", exercise));

                if *force {
                    command.arg("--force");
                }

                let output = command.output().expect("Failed to execute command");

                if output.status.success() {
                    let path_str = String::from_utf8_lossy(&output.stdout)
                        .trim_end_matches('\n')
                        .to_string();

                    // Open the exercise in Visual Studio Code
                    open_vscode(&path_str)
                } else {
                    let error = String::from_utf8_lossy(&output.stderr);

                    let exercise_directory = error
                        .lines()
                        .find(|line| line.starts_with("Error: directory"))
                        .and_then(|line| line.split('\'').nth(1))
                        .unwrap_or("");

                    println!("Directory: '{}' already exists", exercise_directory);

                    open_vscode(&exercise_directory);
                }
            }
            None => {
                println!("No subcommand provided");
            }
        },
        Some(Commands::Submit { track }) => match track {
            Some(track) => {
                println!("Submitting the exercise for the track: {}", track);

                let path = Path::new(".");
                let mut exercise_paths: Vec<PathBuf> = Vec::new();

                // Check if the current directory is an exercise directory
                if *track == "rust" {
                    let source_dir = path.join("src/");
                    let files = read_dir(source_dir);
                    for file in files.unwrap() {
                        let file = file.unwrap();
                        let file_name = file.file_name();
                        let file_name = file_name.to_str().unwrap();
                        if file_name.ends_with(".rs") {
                            exercise_paths.push(file.path());
                        }
                    }
                }

                if ["javascript", "typescript", "js", "ts"].contains(&track.as_str()) {
                    // JavaScript track
                    let source_dir = path.join("src/");
                    let files = read_dir(source_dir);
                    for file in files.unwrap() {
                        let file = file.unwrap();
                        let file_name = file.file_name();
                        let file_name = file_name.to_str().unwrap();
                        if file_name.ends_with(".js") || file_name.ends_with(".ts") {
                            exercise_paths.push(file.path());
                        }
                    }
                }

                if *track == "java" {
                    // Java track
                    let source_dir = path.join("src/main/java");
                    let files = read_dir(source_dir);
                    for file in files.unwrap() {
                        let file = file.unwrap();
                        let file_name = file.file_name();
                        let file_name = file_name.to_str().unwrap();
                        if file_name.ends_with(".java") {
                            exercise_paths.push(file.path());
                        }
                    }
                }

                if *track == "gleam" {
                    // Gleam track
                    let source_dir = path.join("src/");
                    let files = read_dir(source_dir);
                    for file in files.unwrap() {
                        let file = file.unwrap();
                        let file_name = file.file_name();
                        let file_name = file_name.to_str().unwrap();
                        if file_name.ends_with(".gleam") {
                            exercise_paths.push(file.path());
                        }
                    }
                }

                if *track == "go" {
                    // Go track
                    let source_dir = path.join("src/main/");
                    let files = read_dir(source_dir);
                    for file in files.unwrap() {
                        let file = file.unwrap();
                        let file_name = file.file_name();
                        let file_name = file_name.to_str().unwrap();
                        if file_name.ends_with(".go") {
                            exercise_paths.push(file.path());
                        }
                    }
                }

                let exercise_paths_str: Vec<_> = exercise_paths
                    .iter()
                    .map(|p| p.to_str().expect("Failed to convert PathBuf to str"))
                    .collect();

                println!("Submitting the exercise: {:?}", exercise_paths_str);
                Command::new("exercism")
                    .arg("submit")
                    .args(&exercise_paths_str)
                    .spawn()
                    .expect("Failed to execute command");
            }
            None => {
                println!("Submitting the exercise for the auto-detected track");

                let path = Path::new(".");
                let mut exercise_paths: Vec<PathBuf> = Vec::new();

                // Check if the current directory is an exercise directory
                if path.join("Cargo.toml").try_exists().unwrap_or(false) {
                    // Rust track
                    let source_dir = path.join("src/");
                    let files = read_dir(source_dir);
                    for file in files.unwrap() {
                        let file = file.unwrap();
                        let file_name = file.file_name();
                        let file_name = file_name.to_str().unwrap();
                        if file_name.ends_with(".rs") {
                            exercise_paths.push(file.path());
                        }
                    }
                }

                if path.join("package.json").try_exists().unwrap_or(false) {
                    // JavaScript track
                    let source_dir = path.join("src/");
                    let files = read_dir(source_dir);
                    for file in files.unwrap() {
                        let file = file.unwrap();
                        let file_name = file.file_name();
                        let file_name = file_name.to_str().unwrap();
                        if file_name.ends_with(".js") || file_name.ends_with(".ts") {
                            exercise_paths.push(file.path());
                        }
                    }
                }

                if path.join("gradlew").try_exists().unwrap_or(false)
                    || path.join("pom.xml").try_exists().unwrap_or(false)
                {
                    // Java track
                    let source_dir = path.join("src/main/java");
                    let files = read_dir(source_dir);
                    for file in files.unwrap() {
                        let file = file.unwrap();
                        let file_name = file.file_name();
                        let file_name = file_name.to_str().unwrap();
                        if file_name.ends_with(".java") {
                            exercise_paths.push(file.path());
                        }
                    }
                }

                if path.join("gleam.toml").try_exists().unwrap_or(false) {
                    // Gleam track
                    let source_dir = path.join("src/");
                    let files = read_dir(source_dir);
                    for file in files.unwrap() {
                        let file = file.unwrap();
                        let file_name = file.file_name();
                        let file_name = file_name.to_str().unwrap();
                        if file_name.ends_with(".gleam") {
                            exercise_paths.push(file.path());
                        }
                    }
                }

                if path.join("go.mod").try_exists().unwrap_or(false) {
                    // Go track
                    let source_dir = path.join("src/main/");
                    let files = read_dir(source_dir);
                    for file in files.unwrap() {
                        let file = file.unwrap();
                        let file_name = file.file_name();
                        let file_name = file_name.to_str().unwrap();
                        if file_name.ends_with(".go") {
                            exercise_paths.push(file.path());
                        }
                    }
                }

                let exercise_paths_str: Vec<_> = exercise_paths
                    .iter()
                    .map(|p| p.to_str().expect("Failed to convert PathBuf to str"))
                    .collect();

                println!("Submitting the exercise: {:?}", exercise_paths_str);
                Command::new("exercism")
                    .arg("submit")
                    .args(&exercise_paths_str)
                    .spawn()
                    .expect("Failed to execute command");
            }
        },
        None => {
            println!("No command provided");
        }
    }
}

#[cfg(target_os = "windows")]
fn open_vscode(path: &str) {
    Command::new("powershell")
        .current_dir(format!("{}", path))
        .args(&["/C", "code", "."])
        .spawn()
        .expect("Failed to execute command");
}

#[cfg(any(target_os = "macos", target_os = "linux"))]
fn open_vscode(path: &str) {
    Command::new("code")
        .current_dir(path)
        .args(&["."])
        .spawn()
        .expect("Failed to execute command");
}
