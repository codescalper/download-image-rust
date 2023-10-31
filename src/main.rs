use error_chain::error_chain;
use std::fs::File;
use std::io::copy;
use std::path::PathBuf;
use tempfile::Builder;

error_chain! {
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let tmp_dir = Builder::new().prefix("example").tempdir()?;
    let target = "https://www.rust-lang.org/logos/rust-logo-512x512.png";
    let response = reqwest::get(target).await?;
    let download_directory = "C:\\Users\\mayank singh\\Downloads"; // Replace with your desired directory path

    let mut dest = {
        let fname = response
            .url()
            .path_segments()
            .and_then(|segments| segments.last())
            .and_then(|name| if name.is_empty() { None } else { Some(name) })
            .unwrap_or("tmp.bin");

        println!("file to download: '{}'", fname);

        let file_path: PathBuf = [download_directory, fname].iter().collect();
        println!("will be located under: '{:?}'", file_path);

        File::create(file_path)?
    };

    let response = reqwest::get(target).await?;
    let content = response.text().await?;
    copy(&mut content.as_bytes(), &mut dest)?;

    Ok(())
}
