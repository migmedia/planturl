// Copyright 2021 - 2022  Micha Glave

use anyhow::Result;
use clap::Parser;
use planturl::*;
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;
use std::str::FromStr;

/// compress with deflate.
pub fn deflate(data: &str) -> Vec<u8> {
    deflate::deflate_bytes_conf(data.as_bytes(), deflate::Compression::Best)
}

#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone)]
enum Compression {
    deflate,
    hex,
    best,
}

impl FromStr for Compression {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "deflate" | "gz" => Ok(Compression::deflate),
            "hex" => Ok(Compression::hex),
            "best" => Ok(Compression::best),
            _ => Err(format!("Unknown compress-format: {s}")),
        }
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone)]
enum ImageType {
    ascii,
    png,
    svg,
}

impl Display for ImageType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ImageType::ascii => "txt",
                ImageType::svg => "svg",
                ImageType::png => "png",
            }
        )
    }
}

impl FromStr for ImageType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "ascii" | "txt" => Ok(ImageType::ascii),
            "svg" => Ok(ImageType::svg),
            "png" => Ok(ImageType::png),
            _ => Err(format!("Unknown image-type {s}")),
        }
    }
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Options {
    /// Input file, stdin if not present
    #[arg(short, long)]
    source: Option<PathBuf>,

    /// appends the encoded-string onto this URL
    #[arg(short = 'u', long, default_value = "http://www.plantuml.com/plantuml")]
    base_url: String,

    /// embeds the url into an HTML-IMG-Tag.
    #[arg(short, long)]
    img: bool,

    /// downloads an image from a plantuml-server.
    #[arg(short, long)]
    download: bool,

    /// compression to use [hex, deflate, best]
    #[arg(short, long, default_value = "deflate")]
    compression: Compression,

    /// imagetype [ascii, png, svg]
    #[arg(short = 't', long = "type", default_value = "svg")]
    image_type: ImageType,

    /// saves the result in the given file or stdout if not present
    #[arg(short, long)]
    file: Option<PathBuf>,
}

fn main() -> anyhow::Result<()> {
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
    let Options {
        img,
        download,
        base_url: url,
        image_type,
        compression,
        ..
    } = &opt;
    let encoded_diagram = match compression {
        Compression::hex => encode_hex(&input),
        Compression::deflate => encode(&deflate(&input)),
        Compression::best => {
            let deflated = encode(&deflate(&input));
            if deflated.len() >= input.len() * 2 + 2 {
                encode_hex(&input)
            } else {
                deflated
            }
        }
    };
    let mut sink = if let Some(filename) = &opt.file {
        Box::new(
            File::create(filename)
                .unwrap_or_else(|_| panic!("target file {} not found!", filename.display())),
        ) as Box<dyn Write>
    } else {
        Box::new(std::io::stdout())
    };

    let url = format!("{url}/{image_type}/{encoded_diagram}");
    if *img {
        sink.write_all(format!("<img src=\"{url}\">").as_bytes())?;
    } else if *download {
        let img = reqwest::blocking::get(&url)?.bytes()?;
        sink.write_all(&img)?;
    } else {
        sink.write_all(encoded_diagram.as_bytes())?;
    }
    Ok(())
}
