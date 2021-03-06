# USAGE

## find_apt 0.1.0

```
USAGE:
    find_apt.exe <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    help         Prints this message or the help of the given subcommand(s)
    search       Search one address against a list of starting point
    summarize    Run the search command for every address in a file
```

## find_apt.exe-search 0.1.0

```
Search one address against a list of starting point

USAGE:
    find_apt.exe search [OPTIONS] <address>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -s, --starting-points <starting-points>    The starting points that will be tested. Formatted as a comma separated
                                               list of place_id:nickname elements [default: ChIJz85LumxtkFQRhW-
                                               lYWwmRpM:Microsoft,ChIJRe_JoxEBkFQRbaakkmkDFk0:Boeing]

ARGS:
    <address>    The address to test every starting point against
```

## find_apt.exe-summarize 0.1.0
```
Run the search command for every address in a file

USAGE:
    find_apt.exe summarize [OPTIONS] <file>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -s, --starting-points <starting-points>    The starting points that will be tested against every address in the
                                               input file. Formatted as a comma separated list of place_id:nickname
                                               elements [default: ChIJz85LumxtkFQRhW-
                                               lYWwmRpM:Microsoft,ChIJRe_JoxEBkFQRbaakkmkDFk0:Boeing]

ARGS:
    <file>    The file which should contain one address per line
```
