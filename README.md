# planturl

A [plantuml](https://plantuml.com/) file to server-url-encoder. For use with [Gitea](https://gitea.io) or any
documentation tool.
Encodes plantuml-code-snippets as URL usable for the [PlantUML-Service](http://www.plantuml.com/plantuml/) or a
self-hosted clone as described in [PlantUML Text Encoding](https://plantuml.com/pte).

## Install

check out this project.

```
cargo install --features=build-binary --path=<path to this project> 
```

## Usage

```usage
A plantuml-file to server-url-encoder and downloader.


Usage: planturl [OPTIONS]

Options:
  -s, --source <SOURCE>            Input file, stdin if not present
  -u, --base-url <BASE_URL>        appends the encoded-string onto this URL [default: http://www.plantuml.com/plantuml]
  -i, --img                        embeds the url into an HTML-IMG-Tag
  -d, --download                   downloads an image from a plantuml-server
  -c, --compression <COMPRESSION>  compression to use [hex, deflate, best] [default: deflate]
  -t, --type <IMAGE_TYPE>          imagetype [ascii, png, svg] [default: svg]
  -f, --file <FILE>                saves the result in the given file or stdout if not present
  -h, --help                       Print help information
  -V, --version                    Print version information
```

## Gitea integration

add a plantuml-markup-section in the `app.ini`-file:

```toml
[markup.plantuml]
ENABLED = true
# List of file extensions that should be rendered by an external command
FILE_EXTENSIONS = .puml,.uml,.plantuml
# External command to render all matching extensions
RENDER_COMMAND = "/path/to/planturl --base-url https://www.plantuml.com/plantuml --img"
# Don't pass the file on STDIN, pass the filename as argument instead.
IS_INPUT_FILE = false

```

