use std::path::PathBuf;

use clap::Parser;

use mojiharau::{fix_mojibake, Config};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct CLIConfig {
    /// Mojibake'd file to fix
    #[clap(parse(try_from_str=check_and_canonicalize))]
    source: PathBuf,

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

fn check_and_canonicalize(s: &str) -> std::io::Result<PathBuf> {
    let actual_path = PathBuf::from(s);
    actual_path.canonicalize()
}

fn main() -> Result<(), mojiharau::ZipError> {
    let user_config = CLIConfig::parse();

    println!("Chosen file: {}", user_config.source.display());

    let target_file_name = match user_config.output.as_ref() {
        Some(path) => path.clone(),
        None => {
            let archive_name = user_config.source.file_stem().unwrap();
            let fixed_name = format!("{}-fixed", archive_name.to_str().unwrap());
            let mut final_path = PathBuf::from(user_config.source.parent().unwrap());
            final_path.push(fixed_name);
            if let Some(ext) = user_config.source.extension() {
                final_path.set_extension(ext);
            }
            final_path
        }
    };

    if user_config.verbose {
        println!("Output file path: {}", target_file_name.display());
    }

    let fix_config = Config {
        input: user_config.source,
        output: target_file_name,
        unpack: user_config.unpack,
    };

    fix_mojibake(&fix_config, user_config.verbose)
}
