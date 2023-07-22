# cli-fs

cli-fs is a simple CLI app for common file operations written in Rust. It uses the [**fli**](https://github.com/codad5/fli) crate, which provides a simple and intuitive way to define command-line interfaces for your Rust applications.

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
  ./cli-fs write filename.txt -a "Hello, world!"
  ```

- Write a word to a file (overwrite mode, force has no effect):
  ```bash
  ./cli-fs write filename.txt -f "Hello, world!"
  ```

## Credits

cli-fs is built by [codad5](https://github.com/codad5) and uses the [fli](https://github.com/codad5/fli) crate for command-line interface functionality.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
```