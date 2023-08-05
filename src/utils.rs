// utils.rs
use std::io::prelude::*;
use std::io::{Seek, Write};
use std::iter::Iterator;
use zip::result::ZipError;
use zip::write::FileOptions;

use std::fs::File;
use std::path::Path;
use walkdir::{DirEntry, WalkDir};

use dirs;

// post zipped file to server
pub async fn post_shbin() {
    let path = dirs::home_dir().unwrap().join("packet-sender.zip").into_os_string().into_string().unwrap();

    // read the whole file
    let contents = tokio::fs::read(path).await.unwrap();
    
    // create a part
    let part = reqwest::multipart::Part::bytes(contents)
        .file_name("packet-sender.zip")
        .mime_str("application/octet-stream").unwrap();

    // create multipart form
    let form = reqwest::multipart::Form::new()
        .part("file", part);
        
    let client = reqwest::Client::new();
    let _ = client.post("http://localhost:8080/push")
        .multipart(form)
        .send()
        .await
        .unwrap();
}

pub async fn rm_zip() {
    let path = dirs::home_dir().unwrap().join("packet-sender.zip").into_os_string().into_string().unwrap();
    tokio::fs::remove_file(path).await.unwrap();
}


// zip ~/.shbin --> ~/packet-sender.zip
pub fn zip_shbin() -> i32 {
    let shbin_path = dirs::home_dir().unwrap().join(".shbin").into_os_string().into_string().unwrap();
    let output_path = dirs::home_dir().unwrap().join("packet-sender.zip").into_os_string().into_string().unwrap();
    
    // use deflate compressino method
    const METHOD_DEFLATED: zip::CompressionMethod = zip::CompressionMethod::Deflated;

    match doit(&shbin_path, &output_path, METHOD_DEFLATED) {
        Ok(_) => println!("done: {} written to {}", shbin_path, output_path),
        Err(_) => panic!("Failed to convert path to string"),
    }

    0
}
// from zip-rs (https://github.com/zip-rs/zip/blob/master/examples/write_dir.rs)
fn zip_dir<T>(
    it: &mut dyn Iterator<Item = DirEntry>,
    prefix: &str,
    writer: T,
    method: zip::CompressionMethod,
) -> zip::result::ZipResult<()>
where
    T: Write + Seek,
{
    let mut zip = zip::ZipWriter::new(writer);
    let options = FileOptions::default()
        .compression_method(method)
        .unix_permissions(0o755);

    let mut buffer = Vec::new();
    for entry in it {
        let path = entry.path();
        let name = path.strip_prefix(Path::new(prefix)).unwrap();

        // Write file or directory explicitly
        // Some unzip tools unzip files with directory paths correctly, some do not!
        if path.is_file() {
            //println!("adding file {path:?} as {name:?} ...");
            #[allow(deprecated)]
            zip.start_file_from_path(name, options)?;
            let mut f = File::open(path)?;

            f.read_to_end(&mut buffer)?;
            zip.write_all(&buffer)?;
            buffer.clear();
        } else if !name.as_os_str().is_empty() {
            // Only if not root! Avoids path spec / warning
            // and mapname conversion failed error on unzip
            //println!("adding dir {path:?} as {name:?} ...");
            #[allow(deprecated)]
            zip.add_directory_from_path(name, options)?;
        }
    }
    zip.finish()?;
    Result::Ok(())
}

// from zip-rs (https://github.com/zip-rs/zip/blob/master/examples/write_dir.rs)
fn doit(
    src_dir: &str,
    dst_file: &str,
    method: zip::CompressionMethod,
) -> zip::result::ZipResult<()> {
    if !Path::new(src_dir).is_dir() {
        return Err(ZipError::FileNotFound);
    }

    let path = Path::new(dst_file);
    let file = File::create(path).unwrap();

    let walkdir = WalkDir::new(src_dir);
    let it = walkdir.into_iter();

    zip_dir(&mut it.filter_map(|e| e.ok()), src_dir, file, method)?;

    Ok(())
}