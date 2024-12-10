# Advert of Code (AoC) Solutions

This repository contains my solutions to some of [Advert of Code](https://adventofcode.com/)'s problems.

To run a solution, run the `cargo run <year> <day>` command on your terminal. A nightly compiler is required.

> [!NOTE]
> - Some of my solutions expect from your CPU to support instruction sets like AVX2 and SSE2. I do not verify their availability (or I might do, who knows). At worst you will get undefined behavior. At best you will get meaningless results.
> - The inputs (i.e. `input.txt` files) must have a trailing newline.

## A Message to the Windows Users

Most of my solutions expect `input.txt` to have a line feed (LF) end of line (EOL). Read the answers to this [SO question](https://stackoverflow.com/questions/27810758/how-to-replace-crlf-with-lf-in-a-single-file/27814403) to convert the `input.txt` files from CRLF to LF. Note that this might not be necessary; I have defined a `.gitattributes` file that instructs Git to do just that.

## Proposing better ideas

I'm open to suggestions on how to make my current solutions more time efficient. Head over to "Issues" and create a new issue!

## Forking the project

I have structured the code in a way that allows me to add new days as easily as possible.
If you want, you can fork the project to enjoy the same benefits. After all, the code is MIT licensed.
