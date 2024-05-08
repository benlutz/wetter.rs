# wetter.rs

This is a small Rust project called `wetter.rs`.  
I used this project to gain my first experience with Rust.

This CLI allows users to input a location and retrieve the current weather information, including temperature, humidity, and weather conditions. It provides a convenient way to quickly check the weather without leaving the command line interface.

The project is heavily inspired by [wttr.in](https://github.com/chubin/wttr.in)

## Installation

To run this project, make sure you have Rust installed. You can install Rust by following the instructions at [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install).

## Usage

To use this project, follow these steps:

1. Clone the repository: `git clone https://github.com/benlutz/wetter.rs.git`
2. Navigate to the project directory: `cd wetter.rs`
3. Retrieve your own [Open Weather Map Api Key](https://openweathermap.org/)
4. Rename `env.sample` to `.env` and add your api key
5. Build the project: `cargo build`
6. Run the project: `cargo run` to get the current weather in Vienna
7. Use `cargo run -- London` to get the current weather in London

## License

This project is licensed under the [MIT License](LICENSE).
