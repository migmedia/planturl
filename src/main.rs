//! Copyright 2021 - Micha Glave

use std::fs::File;
use std::io;
use std::io::{Read, Write};
use std::path::PathBuf;
use structopt::*;
use strum::EnumString;

/// map the 6bit to a char the plantuml-way.
fn get_char_for_index(index: u8) -> Option<char> {
    let plantuml64 = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz-_".as_bytes();
    plantuml64.get(index as usize).map(|b: &u8| *b as char)
}

/// Encodes the 8-bit array in a 6-bit-based String analog to `base64`.
/// Described in [Plantuml-Documentation](https://plantuml.com/en/text-encoding).
fn encode(data: &[u8]) -> String {
    data.chunks(3)
        .map(split)
        .flat_map(encode_chunk)
        .collect::<String>()
}

/// Splits the 8-bit array-chunk into a 6-bit.
/// Based on <https://tiemenwaterreus.com/posts/implementing-base64-in-rust/>.
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

/// trim the input-data by removing the `@startuml`-prelude and `@enduml`-suffix.
fn cleanup_input(data: &str) -> String {
    let pattern: &[_] = &['\n', ' ', '\t'];
    data.replace("\r\n", "\n")
        .trim_matches(pattern)
        .replace("@startuml\n", "")
        .replace("\n@enduml", "")
}

/// compress with deflate.
fn deflate(data: &str) -> Vec<u8> {
    deflate::deflate_bytes_conf(data.as_bytes(), deflate::Compression::Best)
}

/// Encodes in HEX as described in
/// [PlantUML Text Encoding | Simple HEX format](https://plantuml.com/en/text-encoding#32ec0710e82adf79).
fn encode_hex(input: &str) -> String {
    String::from("~h")
        + &*input
            .as_bytes()
            .iter()
            .map(|c| format!("{:02x}", c))
            .collect::<String>()
}

#[derive(Debug, EnumString)]
enum Compression {
    Deflate,
    Hex,
    Best,
}

#[derive(StructOpt, Debug)]
#[structopt(name = "planturl", about = "A plantuml-url generator.")]
struct Options {
    /// Input file, stdin if not present
    #[structopt(short, long, parse(from_os_str))]
    source: Option<PathBuf>,

    /// appends the encoded-string onto this URL
    #[structopt(short, long, default_value = "http://www.plantuml.com/plantuml/png/")]
    url: String,

    /// embeds the url into an HTML-IMG-Tag.
    #[structopt(short, long)]
    img: bool,

    /// compression to use [Hex, Deflate, Best]
    #[structopt(short, long, default_value = "Deflate")]
    compression: Compression,
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
    let input = cleanup_input(&*input);
    use Compression::*;
    let url = match opt.compression {
        Hex => encode_hex(&input),
        Deflate => encode(&*deflate(&*input)),
        Best => {
            let deflated = encode(&*deflate(&*input));
            if deflated.len() >= input.len() * 2 + 2 {
                encode_hex(&input)
            } else {
                deflated
            }
        }
    };
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
    use crate::{cleanup_input, deflate, encode, encode_hex};

    #[test]
    fn compress_example() {
        let input = "Bob -> Alice : hello";
        assert_eq!("SyfFKj2rKt3CoKnELR1Io4ZDoSa70000", encode(&*deflate(input)));
    }

    #[test]
    fn ignore_trailing_whitespace() {
        let input = "  \n@startuml\nBob -> Alice : hello\n@enduml\n\n";
        assert_eq!("Bob -> Alice : hello", cleanup_input(input));
    }

    #[test]
    fn ignore_crlf() {
        let input = "@startuml\r\nBob -> Alice : hello\r\n@enduml\r\n";
        assert_eq!("Bob -> Alice : hello", cleanup_input(input));
    }

    #[test]
    fn encode_in_hex() {
        let input = "@startuml\nAlice->Bob : I am using hex\n@enduml";
        let hex = encode_hex(input);
        assert_eq!("~h407374617274756d6c0a416c6963652d3e426f62203a204920616d207573696e67206865780a40656e64756d6c", hex)
    }
}
