# Advent of Code 2023

This repo contains solutions implemented by Sylvester Hesp (@oisyn) in Rust for all days of [Advent of Code 2023](https://adventofcode.com/2023).

## Usage

The implementations are arranged as stand alone crates for each day. They use the following command line parameters:

| argument | description |
|---|---|
|&lt;nothing>|Runs with the standard puzzle input `<day>/data/input.txt`
|-e[postfix]|Runs with example input `<day>/data/example[postfix].txt`. So `-e` uses `example.txt` `-e2` `example2.txt`, etc.
|-i &lt;path>|Runs with an explicitly specified input relative to the current folder|

Examples and (my personal) puzzle inputs are located in `<day>/data`.

`/extra` is ignored by git, it is intended as a folder for alternative inputs.
