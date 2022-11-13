# gpl2rxpl

This command line tool converts GIMP Palette (.gpl files) to [rx](https://github.com/cloudhead/rx/)(a minimalist pixel editor) script(.palette files).

## usage

```
gpl2rxpl <file [--silent] [--output path]| --help> 
    file: one .gpl file path as input
    --output: specify a file path for output
    --help: print this message
```

after the a palette file is created at, say, `~/Folder/file.palette`

do a `:source ~/Folder/file.palette` in rx to use that palette

## installation

To install this binary using cargo, do
```
cargo install --git https://github.com/edwar4rd/gpl2rxpl/
```
