//! Copyright 2021 - Micha Glave

use deflate::{deflate_bytes_conf, Compression};
use std::fs::File;
use std::io;
use std::io::{Read, Write};
use std::path::PathBuf;
use structopt::*;

/// encoding differs from `base64` as described in [Plantuml-Documentation](https://plantuml.com/en/text-encoding).
fn get_char_for_index(index: u8) -> Option<char> {
    let plantuml64 = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz-_".as_bytes();
    plantuml64.get(index as usize).map(|b: &u8| *b as char)
}

fn encode(data: &[u8]) -> String {
    data.chunks(3)
        .map(split)
        .flat_map(encode_chunk)
        .collect::<String>()
}

/// base64 encoding based on https://tiemenwaterreus.com/posts/implementing-base64-in-rust/.
fn split(chunk: &[u8]) -> Vec<u8> {
    match chunk.len() {
        1 => vec![&chunk[0] >> 2, (&chunk[0] & 0b00000011) << 4],
        2 => vec![
            &chunk[0] >> 2,
            (&chunk[0] & 0b00000011) << 4 | &chunk[1] >> 4,
            (&chunk[1] & 0b00001111) << 2,
        ],
        3 => vec![
            &chunk[0] >> 2,
            (&chunk[0] & 0b00000011) << 4 | &chunk[1] >> 4,
            (&chunk[1] & 0b00001111) << 2 | &chunk[2] >> 6,
            &chunk[2] & 0b00111111,
        ],
        _ => unreachable!(),
    }
}

fn encode_chunk(chunk: Vec<u8>) -> Vec<char> {
    let mut out = vec!['0'; 4];
    for i in 0..chunk.len() {
        if let Some(chr) = get_char_for_index(chunk[i]) {
            out[i] = chr;
        }
    }
    out
}

fn cleanup_input(data: &str) -> String {
    let pattern: &[_] = &['\n', ' ', '\t'];
    data.replace("\r\n", "\n")
        .trim_matches(pattern)
        .replace("@startuml\n", "")
        .replace("\n@enduml", "")
}

pub fn encode_deflate(data: &str) -> String {
    let compressed = deflate_bytes_conf(cleanup_input(data).as_bytes(), Compression::Best);
    encode(compressed.as_slice())
}

#[derive(StructOpt, Debug)]
#[structopt(name = "planturl", about = "A plantuml-url generator.")]
struct Options {
    /// Input file, stdin if not present
    #[structopt(short, long, parse(from_os_str))]
    source: Option<PathBuf>,

    /// appends the generated url onto this url
    #[structopt(short, long, default_value = "http://www.plantuml.com/plantuml/png/")]
    url: String,

    /// embeds the url into an HTML-IMG-Tag.
    #[structopt(short, long)]
    img: bool,
}

fn main() {
    let opt = Options::from_args();
    let mut input = String::new();
    if let Some(filename) = &opt.source {
        let mut file =
            File::open(filename).expect(&*format!("source file {} not found!", filename.display()));
        file.read_to_string(&mut input)
            .expect("error reading source file");
    } else {
        io::stdin()
            .read_to_string(&mut input)
            .expect("error reading stdin");
    }
    let url = encode_deflate(&*input);
    let encoded_diagram = if opt.img {
        format!("<img src=\"{}{}\" >", &opt.url, url)
    } else {
        url
    };

    io::stdout()
        .write_all(encoded_diagram.as_bytes())
        .expect("error writing stdout");
}

#[cfg(test)]
mod should {
    use crate::encode_deflate;

    #[test]
    fn compress_example() {
        let input = "Bob -> Alice : hello";
        assert_eq!("SyfFKj2rKt3CoKnELR1Io4ZDoSa70000", encode_deflate(input));
    }

    #[test]
    fn ignore_trailing_whitespace() {
        let input = "  \n@startuml\nBob -> Alice : hello\n@enduml\n\n";
        assert_eq!("SyfFKj2rKt3CoKnELR1Io4ZDoSa70000", encode_deflate(input));
    }

    #[test]
    fn ignore_crlf() {
        let input = "@startuml\r\nBob -> Alice : hello\r\n@enduml\r\n";
        assert_eq!("SyfFKj2rKt3CoKnELR1Io4ZDoSa70000", encode_deflate(input));
    }
}
