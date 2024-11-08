# Log Read Test Project

This project is designed to test the functionality of reading from a log file and dynamically updating based on file changes. The main purpose is to monitor a given log file (`test.txt`), read its initial contents, print them to the console, and continue to watch for any changes to the file, printing new contents as they are appended.

## Description

The project demonstrates a practical implementation of file I/O and event watching in Rust. It leverages the `notify` crate to monitor file changes and the `log` crate for logging purposes. The expected behavior is:
- Initially read the contents of `test.txt` and print them.
- Continuously watch for any updates to `test.txt` and print new contents as they are appended to the file.
- Log the operations and any errors encountered during the execution.

## Build Steps

To build and run this project, follow these steps:

1. **Install Rust**: Ensure that you have Rust installed. You can install it from [rust-lang.org](https://www.rust-lang.org/).

2. **Clone the Repository**: Clone this repository to your local machine.

    ```sh
    git clone https://github.com/Drew-Chase/log-read-test
    cd log-read-test
    ```

3. **Prepare `test.txt` File**: Create and populate a `test.txt` file in the root of the project directory with some initial content. This is the file that the program will monitor.

4. **Build the Project**: Use Cargo, Rust's package manager, to build the project.

    ```sh
    cargo build
    ```

5. **Run the Project**: Execute the project to start monitoring `test.txt`.

    ```sh
    cargo run
    ```

## Usage

Once the program is running, it will:
- Print the initial contents of `test.txt` to the console.
- Continue to watch for any changes to the file and print additional contents as they are added to `test.txt`.

This project serves as a basis for understanding file I/O operations, asynchronous updates, and implementing an event-watching mechanism in Rust.

