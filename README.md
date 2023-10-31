# Download Image in Rust 🦀

This Rust project demonstrates how to download an image from a URL and save it to your local file system. It utilizes various Rust crates for HTTP requests and file handling.

## Crates Used

- [tempfile](https://crates.io/crates/tempfile): A crate for working with temporary files and directories.
- [error-chain](https://crates.io/crates/error-chain): A crate for creating custom error types and handling errors gracefully.
- [reqwest](https://crates.io/crates/reqwest): An HTTP client for making web requests.
- [tokio](https://crates.io/crates/tokio): A runtime for writing asynchronous code in Rust.

## Importance

This project showcases how to download and save images from the internet in a Rust application. It's valuable for developers looking to automate image retrieval or implement web scraping with Rust.

## Setting Up and Running Locally

1. Clone this repository:
   ```bash
   git clone https://github.com/codescalper/download-image-rust.git
   cd download-image-rust
   ```
2. Install Rust and Cargo if you haven't already. You can download Rust from [Rust's official website](https://www.rust-lang.org/learn/get-started).

3. Build and run the project using Cargo:

   ```bash
   cargo run
   ```

4. The image will be downloaded and saved to your specified directory (change the `download_directory` variable in the code).

## Where It Can Be Used

1.  **Image Scraping**: Use this project as a starting point to collect and store images from various websites.
2.  **Web Automation**: Incorporate image downloading in web automation tasks, such as automated testing or content harvesting.

---

#### _Output_

![Rust Logo](https://cdn.discordapp.com/attachments/1150040438904979557/1168964244163792916/image.png?ex=6553adca&is=654138ca&hm=05e46d5b456a7bf930941bfa6b23306e67a834d3e2d862b7de47ac1ab28cb2ff)

---

If you have any questions or suggestions, feel free to reach out on Twitter: [@mayanks_tw](https://twitter.com/mayanks_tw).
