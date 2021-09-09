# planturl
A [plantuml](https://plantuml.com/) file to server-url-encoder. For use with [Gitea](https://gitea.io) or any
documentation tool.
Encodes plantuml-code-snippets as URL usable for the [PlantUML-Service](http://www.plantuml.com/plantuml/) or a
self-hosted clone as described in [PlantUML Text Encoding](https://plantuml.com/pte).

## Install

check out this project.

```
cargo install --path=<path to this project>
```

## Usage

```usage
planturl 0.3.0
A plantuml-url generator.

USAGE:
    planturl [FLAGS] [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -i, --img        embeds the url into an HTML-IMG-Tag
    -V, --version    Prints version information

OPTIONS:
    -c, --compression <compression>    compression to use [Hex, Deflate, Best, Classic] [default: Classic]
    -s, --source <source>              Input file, stdin if not present
    -u, --url <url>                    appends the encoded-string onto this URL [default:
                                       http://www.plantuml.com/plantuml/png/]

```

## Gitea integration

add a plantuml-markup-section in the `app.ini`-file:

```toml
[markup.plantuml]
ENABLED = true
# List of file extensions that should be rendered by an external command
FILE_EXTENSIONS = .puml,.uml,.plantuml
# External command to render all matching extensions
RENDER_COMMAND = "/usr/bin/planturl --url http://www.plantuml.com/plantuml/svg/ --img"
# Don't pass the file on STDIN, pass the filename as argument instead.
IS_INPUT_FILE = false

```

