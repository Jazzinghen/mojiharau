use zip::read::ZipArchive;
pub use zip::result::{ZipError, ZipResult};
use zip::write::ZipWriter;

use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

pub struct Config {
    pub input: PathBuf,
    pub output: PathBuf,
    pub unpack: bool,
}

pub fn fix_mojibake(config: &Config, verbose: bool) -> ZipResult<()> {
    let file = File::open(config.input.as_path()).unwrap();
    let reader = BufReader::new(file);

    let mut archive = ZipArchive::new(reader).unwrap();
    let encoding = encoding_rs::SHIFT_JIS;

    let fixed_file = File::create(config.output.as_path())?;
    let mut fixed_zip = ZipWriter::new(fixed_file);

    for compressed_id in 0..archive.len() {
        let compressed_file = archive.by_index(compressed_id)?;
        let outpath = match compressed_file.enclosed_name() {
            Some(path) => path,
            None => {
                println!("Entry number {} has a suspicious path", compressed_id);
                continue;
            }
        };

        let (decoded_string, encoding_used, malformed) =
            encoding.decode(compressed_file.name_raw());
        if malformed {
            return ZipResult::Err(ZipError::UnsupportedArchive(
                "The provided archive had file names that could't be converted to UTF-8",
            ));
        }

        let final_filename = String::from(decoded_string);

        if verbose {
            if compressed_file.is_dir() {
                println!(
                    "Found directory with name \"{}\"; Encoding: {}",
                    outpath.display(),
                    encoding_used.name()
                );
                println!("\tDecoded name: \"{}\"", final_filename);
            } else {
                println!(
                    "Found file with name \"{}\" ({} bytes); Encoding: {}",
                    outpath.display(),
                    compressed_file.size(),
                    encoding_used.name()
                );
                println!("\tDecoded name: \"{}\"", final_filename);
            }

            let comment = compressed_file.comment();
            if !comment.is_empty() {
                println!("\tCurrent entry' comment: {}", comment);
            }
            println!()
        };

        fixed_zip.raw_copy_file_rename(compressed_file, final_filename)?;
    }

    fixed_zip.finish()?;
    Ok(())
}
