extern crate tempfile;
extern crate error_chain;
extern crate reqwest;
extern crate tokio;

error_chain::error_chain! {
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let temporary_directory = tempfile::Builder::new()
        .prefix("temp-dir")
        .tempdir()?;

    let target_image_url = "https://www.rust-lang.org/logos/rust-logo-512x512.png";
    let response = reqwest::get(target_image_url).await?;
    let mut output_file = {
        let filename = response
            .url()
            .path_segments()
            .and_then(|segments| segments.last())
            .and_then(|name| if name.is_empty() {None} else { Some(name) })
            .unwrap_or("tmp.bin");

        println!("We are going to download: \"{}\"", filename);

        let file_path = temporary_directory.path().join(filename);

        println!("This is going to be the file location: \"{:?}\"", file_path);

        std::fs::File::create(file_path)?
    };

    let content = response.text().await?;

    std::io::copy(&mut content.as_bytes(), &mut output_file).unwrap();

    Ok(())
}
