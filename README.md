# pepulo-rps
Rock-paper-scissors game implemented in Rust and leptos. My first project using leptos. Don't use this as a model project.

Web UI is in Finnish

To run and install prerequisites:

- Install Cargo (Go to https://rustup.rs and follow the instructions)
- Install Trunk
~~~
cargo install trunk
~~~
- Add the wasm target
~~~
rustup target add wasm32-unknown-unknown
~~~
- Run the server. The UI will be available at 127.0.0.1:8000.
- --open flag is optional. With the flag Trunk will open the UI in your default browser
~~~
trunk serve --open
~~~
When running the project for the first time, you will have to wait for compilation.
