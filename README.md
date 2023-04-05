# Github Actions Autodocs (GCA)

Generate a basic readme for the given github action

## Installation

Install via cargo

```sh
  cargo install github-actions-autodocs
```

## Usage

```sh
Usage: github-actions-autodocs [OPTIONS]

Options:
  -f, --file <FILE>      Source path [default: action.yml]
      --dry              Dry run
  -o, --output <OUTPUT>  Output filepath
  -h, --help             Print help
  -V, --version          Print version
```

## Github Action

```yaml
name: Generate Readme

on:
  push:

jobs:
  generate:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: rawnly/github-actions-autodocs@main
        with:
          action-file: action.yml
      - shell: bash
        run: |
          git add -A .
          git commit -m 'docs: README'
          git push -u origin ${{ github.ref }}
```

> TIP: Alias `github-actions-autodocs` to `gca` 😉

### LICENSE

The MIT License
