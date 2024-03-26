# Exercism CLI Enhancement

This CLI app is designed to enhance the functionality of the Exercism CLI tool. The Exercism CLI tool is used for downloading and submitting exercises for various programming tracks. However, it has some limitations that this CLI app aims to address.

## Features

1. **Automatic Exercise Submission**: The CLI app simplifies the exercise submission process. Instead of manually specifying the file path, the CLI app automatically detects the programming track and finds the exercise file in the source directory. This eliminates the need for manual file path input.

2. **Integration with Visual Studio Code**: After downloading an exercise, the CLI app will open the exercise directory in Visual Studio Code. This allows for a smoother workflow by directly opening the exercise in an IDE.

## Prerequisites

To use this CLI app, you need to have the Exercism CLI tool installed on your system. The CLI app relies on the Exercism CLI tool to perform the exercise download and submission operations.

## Usage

1. Downloading an Exercise:
  - Copy command from exercism `exercism download --track <track> --exercise <exercise>`
  - Combine with our CLI command: `ecd exercism download --track <track> --exercise <exercise>`
  - Description: Downloads the specified exercise for the given programming track and open VSCode with the exercise directory.

2. Submitting an Exercise:
  - Command: `exercism-enhanced submit [--track <track>]`
  - Description: Submits the exercise for the specified programming track. If no track is provided, the CLI app will automatically detect the track based on the current directory.

## Installation

To install the CLI app, follow these steps:

1. Install the Exercism CLI tool by following the instructions provided by Exercism (https://exercism.io/cli).
2. Clone the repository for the CLI app from GitHub.
3. Build the CLI app using the appropriate build command for your system.
4. Add the CLI app to your system's PATH variable for easy access.

## Compatibility

The CLI app is compatible with Windows, macOS, and Linux operating systems.

## License

This CLI app is licensed under the [MIT License](LICENSE).

## Acknowledgements

This CLI app is built on top of the Exercism CLI tool, which is developed and maintained by the Exercism community.