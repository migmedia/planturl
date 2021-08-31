# planturl 
A [plantuml](https://plantuml.com/) file to server-url-encoder. For use with [Gitea](https://gitea.io) or any documentation tool.

## Install

```
cargo install --path=<path to this project>
```

## Usage

```
planturl 0.1.0
A plantuml-url generator.

USAGE:
    planturl [FLAGS] [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -i, --img        embeds the url into an HTML-IMG-Tag
    -V, --version    Prints version information

OPTIONS:
    -s, --source <source>    Input file, stdin if not present
    -u, --url <url>          appends the generated url onto this url [default: http://www.plantuml.com/plantuml/png/]

```

