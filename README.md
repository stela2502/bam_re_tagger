
# bam_re_tagger - a simple Rust program to change tag names in a bam file

## Install

You need the Rust compiler: https://www.rust-lang.org/tools/install

```bash
cargo install --git https://github.com/stela2502/bam_re_tagger
```

## Usage

If you have a DB Rustody bam file and want to analyze that using velocyto.py, the bam tag **MA:Z** needs to be converted to **UB:Z**.
That is what this toolk does.

```bash
bam_re_tagger -h
bam_re_tagger 

USAGE:
    bam_re_tagger [OPTIONS] --input <INPUT> --output <OUTPUT>

OPTIONS:
    -f, --from-tag <FROM_TAG>    tag to search for in the BAM file (default: MA:Z:) [default: MA:Z:]
    -h, --help                   Print help information
    -i, --input <INPUT>          Path to the input BAM file
    -o, --output <OUTPUT>        Path to the output BAM file
    -t, --to-tag <TO_TAG>        tag to replace the found tag with (default: UB:Z:) [default: UB:Z:]
```
