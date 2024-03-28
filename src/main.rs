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

struct TrackConfig {
    track_name: &'static str,
    source_dir: &'static str,
    extensions: &'static [&'static str],
    config_files: &'static [&'static str],
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

    let track_configs: Vec<TrackConfig> = vec![
        TrackConfig {
            track_name: "rust",
            source_dir: "src/",
            extensions: &[".rs"],
            config_files: &["Cargo.toml"],
        },
        TrackConfig {
            track_name: "javascript",
            source_dir: "src/",
            extensions: &[".js", ".ts", ".cjs", ".mjs", ".cts", ".mts"],
            config_files: &["package.json", "deno.json"],
        },
        TrackConfig {
            track_name: "java",
            source_dir: "src/main/java",
            extensions: &[".java"],
            config_files: &["gradlew", "pom.xml"],
        },
        TrackConfig {
            track_name: "gleam",
            source_dir: "src/",
            extensions: &[".gleam"],
            config_files: &["gleam.toml"],
        },
        TrackConfig {
            track_name: "go",
            source_dir: "src/main/",
            extensions: &[".go"],
            config_files: &["go.mod"],
        },
    ];

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

                for config in track_configs {
                    if *track == config.track_name {
                        let source_dir = path.join(config.source_dir);
                        let new_paths = get_exercise_paths(source_dir, config.extensions);
                        exercise_paths.extend(new_paths);
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

                for config in track_configs {
                    if config
                        .config_files
                        .iter()
                        .any(|&file| path.join(file).try_exists().unwrap_or(false))
                    {
                        println!("Detected track: {}", config.track_name);
                        let source_dir = path.join(config.source_dir);
                        let new_paths = get_exercise_paths(source_dir, config.extensions);
                        exercise_paths.extend(new_paths);
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

fn get_exercise_paths(source_dir: PathBuf, extensions: &[&str]) -> Vec<PathBuf> {
    let mut exercise_paths = Vec::new();
    let files = read_dir(source_dir);
    for file in files.unwrap() {
        let file = file.unwrap();
        let file_name = file.file_name();
        let file_name = file_name.to_str().unwrap();
        if extensions.iter().any(|&ext| file_name.ends_with(ext)) {
            exercise_paths.push(file.path());
        }
    }
    exercise_paths
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
