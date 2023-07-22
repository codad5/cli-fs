# cli-fs

cli-fs is a simple CLI app for common file operations written in Rust. It uses the [**fli**](https://github.com/codad5/fli) crate, which provides a simple and intuitive way to define command-line interfaces for your Rust applications.

## About cli-fs

cli-fs was developed as a showcase of some of the use cases of the `fli` crate, which provides an easy-to-use framework for building command-line interfaces in Rust. It demonstrates how to perform common file operations such as listing the contents of a directory, moving files, creating files, speaking words, and writing to files.

## Please Note

This project is primarily intended as a demonstration and learning tool for using the `fli` crate. While it provides basic functionality for file operations, it may not be suitable for production use without further enhancements and testing.
## Usage

To use cli-fs, follow the steps below:

1. Clone the repository:
   ```bash
   git clone https://github.com/codad5/cli-fs.git
   cd cli-fs
   ```

2. Build the application:
   ```bash
   cargo build --release
   ```

3. Run the application:
   ```bash
   cargo run --release
   ```

## Commands

cli-fs provides the following commands:

- `ls`: List the contents of the current directory.
  - Options:
    - `-b, --brief`: List the directory content in brief.

- `move`: Move a file or directory to another location.
  - Options:
    - `-f, --force`: Force move.

- `create`: Create a new file.
  - Options:
    - `-p, --put <content>`: Put content into the file.

- `speak`: Speak a word or the content of a file.
  - Options:
    - `-f, --file <filename>`: Speak the content of a file.

- `write`: Write a word to a file.
  - Options:
    - `-a, --append`: Append the word to the file.
    - `-f, --force`: Force the operation (no effect).

## Examples

- List the contents of the current directory in brief:
  ```bash
  ./cli-fs ls -b
  ```

- Move a file to another location:
  ```bash
  ./cli-fs move old_file.txt new_directory/
  ```

- Create a new file with content:
  ```bash
  ./cli-fs create new_file.txt -p "Hello, world!"
  ```

- Speak a word:
  ```bash
  ./cli-fs speak "Hello, world!"
  ```

- Speak the content of a file:
  ```bash
  ./cli-fs speak -f filename.txt
  ```

- Write a word to a file (append mode):
  ```bash
  ./cli-fs write filename.txt "Hello, world!" -a
  ```

- Write a word to a file (overwrite mode, force has no effect):
  ```bash
  ./cli-fs write filename.txt "Hello, world!" -f
  ```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- The `fli` crate used in this project is developed and maintained by [Chibueze Aniezeofor](https://github.com/codad5).
- Special thanks to [Chibueze Aniezeofor](https://github.com/codad5) for creating cli-fs and showcasing the capabilities of the `fli` crate.

Feel free to explore the code, experiment with the app, and use it as a reference for building your own CLI applications using the `fli` crate. If you encounter any issues or have suggestions for improvements, please don't hesitate to open an issue or pull request on the [GitHub repository](https://github.com/codad5/cli-fs).

Happy coding!
```