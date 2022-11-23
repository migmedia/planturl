// Copyright 2021 - 2022  Micha Glave

use clap::Parser;
use planturl::*;
use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;
use strum::EnumString;

/// compress with deflate.
pub fn deflate(data: &str) -> Vec<u8> {
    deflate::deflate_bytes_conf(data.as_bytes(), deflate::Compression::Best)
}

#[derive(Debug, EnumString, Copy, Clone)]
enum Compression {
    Deflate,
    Hex,
    Best,
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Options {
    /// Input file, stdin if not present
    #[arg(short, long)]
    source: Option<PathBuf>,

    /// appends the encoded-string onto this URL
    #[arg(short, long, default_value = "http://www.plantuml.com/plantuml/png/")]
    url: String,

    /// embeds the url into an HTML-IMG-Tag.
    #[arg(short, long)]
    img: bool,

    /// compression to use [Hex, Deflate, Best]
    #[arg(short, long, default_value = "Deflate")]
    compression: Compression,
}

fn main() {
    let opt = Options::parse();
    let mut input = String::new();
    if let Some(filename) = &opt.source {
        let mut file = File::open(filename)
            .unwrap_or_else(|_| panic!("source file {} not found!", filename.display()));
        file.read_to_string(&mut input)
            .expect("error reading source file");
    } else {
        std::io::stdin()
            .read_to_string(&mut input)
            .expect("error reading stdin");
    }
    let input = cleanup_input(&input);
    use Compression::*;
    let url = match opt.compression {
        Hex => encode_hex(&input),
        Deflate => encode(&deflate(&input)),
        Best => {
            let deflated = encode(&deflate(&input));
            if deflated.len() >= input.len() * 2 + 2 {
                encode_hex(&input)
            } else {
                deflated
            }
        }
    };
    let encoded_diagram = if opt.img {
        format!("<img src=\"{}{url}\" >", &opt.url)
    } else {
        url
    };

    std::io::stdout()
        .write_all(encoded_diagram.as_bytes())
        .expect("error writing stdout");
}
