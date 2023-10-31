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
    let target = "https://scontent-bom1-2.cdninstagram.com/v/t51.2885-19/299393232_1466816717167701_8169088999936650689_n.jpg?stp=dst-jpg_s320x320&_nc_ht=scontent-bom1-2.cdninstagram.com&_nc_cat=109&_nc_ohc=vr2WXBRWKjgAX-462Yq&edm=AOQ1c0wBAAAA&ccb=7-5&oh=00_AfAdKOwYUJel0-QSK39DjtIFdKrFrwmOs3ZgkyIZ8Mqh8Q&oe=6545A6E0&_nc_sid=8b3546";
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
    let content = response.bytes().await?; // Changing this to .byutes() from .text() will download the file as binary
                                           // fixed this error from stackoverflow https://stackoverflow.com/questions/63879853/why-is-a-png-image-downloaded-by-reqwest-corrupt
    let mut cursor = std::io::Cursor::new(content);
    std::io::copy(&mut cursor, &mut dest)?;

    Ok(())
}
