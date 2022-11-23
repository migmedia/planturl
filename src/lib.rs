// Copyright 2021 - 2022  Micha Glave
//! # planturl
//!
//! Helper-crate to convert plantuml-sourcecode to a url-string accepted by the
//! [PlantUML-Demo-Server](https://www.plantuml.com/plantuml).
//!
//! ```rust
//! use planturl::{encode,cleanup_input};
//! let input = "Bob -> Alice : hello";
//! assert_eq!("SyfFKj2rKt3CoKnELR1Io4ZDoSa70000",
//!    encode(&deflate::deflate_bytes_conf(
//!          cleanup_input(input).as_bytes(),
//!          deflate::Compression::Best
//!         ))
//! );
//! ```

/// Encodes the 8-bit array in a 6-bit-based String analog to `base64`.
/// Described in [Plantuml-Documentation](https://plantuml.com/en/text-encoding).
pub fn encode(data: &[u8]) -> String {
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

/// Maps the 6bit to a char the plantuml-way.
fn get_char_for_index(index: u8) -> Option<char> {
    let plantuml64 = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz-_".as_bytes();
    plantuml64.get(index as usize).map(|b: &u8| *b as char)
}

/// Trims the input-data by removing the `@startuml`-prelude and `@enduml`-suffix.
/// Needed to reduce size of URL.
pub fn cleanup_input(data: &str) -> String {
    let pattern: &[_] = &['\n', ' ', '\t'];
    data.replace("\r\n", "\n")
        .trim_matches(pattern)
        .replace("@startuml\n", "")
        .replace("\n@enduml", "")
}

/// Encodes in HEX as described in
/// [PlantUML Text Encoding | Simple HEX format](https://plantuml.com/en/text-encoding#32ec0710e82adf79).
pub fn encode_hex(input: &str) -> String {
    String::from("~h")
        + &*input
            .as_bytes()
            .iter()
            .map(|c| format!("{:02x}", c))
            .collect::<String>()
}

#[cfg(test)]
mod should {
    use crate::{cleanup_input, encode, encode_hex};

    #[test]
    fn compress_example() {
        let input = "Bob -> Alice : hello";
        assert_eq!(
            "SyfFKj2rKt3CoKnELR1Io4ZDoSa70000",
            encode(&deflate::deflate_bytes_conf(
                input.as_bytes(),
                deflate::Compression::Best
            ))
        );
    }

    fn encode_deflate() ->
        let input = "@startuml\nAlice -> Bob: Authentication Request\nBob --> Alice: \
        Authentication Response\n@enduml";
        assert_eq!(
            "Syp9J4vLqBLJSCfFib9mB2t9ICqhoKnEBCdCprC8IYqiJIqkuGBAAUW2rO0LOr5LN92VLvpA1G00",
            encode(&deflate::deflate_bytes_conf(
                input.as_bytes(),
                deflate::Compression::Best
            ))
        );
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

    /// Test based on Demo: https://plantuml.com/en/text-encoding#32ec0710e82adf79
    #[test]
    fn encode_in_hex() {
        let input = "@startuml\nAlice->Bob : I am using hex\n@enduml";
        let hex = encode_hex(input);
        assert_eq!("~h407374617274756d6c0a416c6963652d3e426f62203a204920616d207573696e67206865780a40656e64756d6c", hex)
    }
}
