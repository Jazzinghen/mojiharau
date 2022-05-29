use std::fs::File;
use std::io::{BufReader, Result};
use std::path::PathBuf;

use clap::Parser;

use zip::read::ZipArchive;
use zip::result::{ZipError, ZipResult};
use zip::write::ZipWriter;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Config {
    /// Mojibake'd file to fix
    #[clap(parse(try_from_str=check_and_canonicalize))]
    name: PathBuf,

    /// Output filename. If not set the output will be the source filename +
    /// "-fixed"
    #[clap(short, long, parse(try_from_str=check_and_canonicalize), value_name = "FILE")]
    output: Option<PathBuf>,

    /// Directly unpack file instead of converting
    #[clap(short, long)]
    unpack: bool,

    /// Output a ton of data
    #[clap(short, long)]
    verbose: bool,
}

fn check_and_canonicalize(s: &str) -> Result<PathBuf> {
    let actual_path = PathBuf::from(s);
    actual_path.canonicalize()
}

fn main() {
    let user_config = Config::parse();

    println!("Chosen file: {}", user_config.name.display());
    if let Some(output_path) = user_config.output.as_ref() {
        println!("Provided output: {:?}", output_path.display());
    }
    println!("Unpack archive as well? {}", user_config.unpack);

    real_main(&user_config).unwrap();
}

fn real_main(config: &Config) -> ZipResult<()> {
    let file = File::open(config.name.as_path()).unwrap();
    let reader = BufReader::new(file);

    let mut archive = ZipArchive::new(reader).unwrap();
    let encoding = encoding_rs::SHIFT_JIS;

    let target_file_name = match config.output.as_ref() {
        Some(path) => path.clone(),
        None => {
            let archive_name = config.name.file_stem().unwrap();
            let fixed_name = format!("{}-fixed", archive_name.to_str().unwrap());
            let mut final_path = PathBuf::from(config.name.parent().unwrap());
            final_path.push(fixed_name);
            if let Some(ext) = config.name.extension() {
                final_path.set_extension(ext);
            }
            final_path
        }
    };

    println!("Fixed file path: {}", target_file_name.display());

    let fixed_file = File::create(target_file_name.as_path())?;
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

        if config.verbose {
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
