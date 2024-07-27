# WASM Password Generator

A WebAssembly (WASM) based password generator implemented in Rust. This password generator allows users to specify various criteria such as length, inclusion/exclusion of numbers, alphabets, special characters, and more.

## Features

- **Length**: Specify the length of the password.
- **Include Numbers**: Option to include numeric characters.
- **Include Uppercase Alphabets**: Option to include upper case alphabetic characters.
- **Include Lowercase Alphabets**: Option to include lower case alphabetic characters.
- **Special Characters**: Specify which special characters to include.
- **Exclude Characters**: Specify characters to exclude from the password.
- **No Similar Characters**: Option to exclude similar characters like `i`,`I`, `l`, `1`, `L`, `o`, `0`, `O`.
- **No Duplicate Characters**: Option to ensure no duplicate characters in the password.
- **No Sequential Characters**: Option to ensure no sequential characters in the password.

## Getting Started

### Prerequisites

- Rust and Cargo: [Install Rust](https://www.rust-lang.org/tools/install)
- `wasm-pack`: Install `wasm-pack` by running:
```sh
  cargo install wasm-pack 
```
- Clone the repository:
```sh
    git clone https://github.com/yourusername/password-generator.git
    cd password-generator
```
- Build the WebAssembly package:
```sh
    wasm-pack build --target web
```
### Integrating with a Web Page
- Copy the generated `pkg` directory to your web server or project directory.
- Create an HTML file with the following content:
```html
    <script type="module">
        import init, { generate_password } from './pkg/wasm_password_generator.js';
        async function run() {
            await init();
            document.getElementById('generate').onclick = () => {
                const length = parseInt(document.getElementById('length').value);
                const include_numbers = document.getElementById('include_numbers').checked;
                const include_alphabets = document.getElementById('include_alphabets').checked;
                const special_chars = document.getElementById('special_chars').value;
                const exclude_chars = document.getElementById('exclude_chars').value;
                const no_similar = document.getElementById('no_similar').checked;
                const no_duplicates = document.getElementById('no_duplicates').checked;
                const no_sequential = document.getElementById('no_sequential').checked;
                const password = generate_password(length, include_numbers, include_alphabets, special_chars, exclude_chars, no_similar, no_duplicates, no_sequential);
                document.getElementById('password').innerText = password;
                document.getElementById('copy').classList.remove("d-none");
            };
        }
        run();
    </script>
```
### Development
- Running Tests
- To run the tests and see detailed output, use the following command:
```sh
    cargo test -- --nocapture
```
- This will execute the test suite and print the generated passwords and other debug information to the console.

### Debugging
- You can add `println!` statements in the Rust code to print debug information during the development and testing phases.

### License
- This project is licensed under the MIT License. See the LICENSE file for details.
