# Advert of Code (AoC) Solutions

This repository contains my solutions to some of [Advert of Code](https://adventofcode.com/)'s problems.

The solutions are obviously contained in the `solutions` folder. To see a day's solution, visit the `solutions/src/_{year}/{day}/mod.rs` file.

To run one of them, run the `cargo run` command on your terminal. A nightly compiler might be required.

To benchmark solutions, go to `runner/benches/main.rs` to add the interested days. Then run `cargo bench` and observe the results.

## A Message to the Windows Users

Most of my solutions expect `input.txt` to have a line feed (LF) end of line (EOL). Read the answers to this [SO question](https://stackoverflow.com/questions/27810758/how-to-replace-crlf-with-lf-in-a-single-file/27814403) to convert the `input.txt` files from CRLF to LF. Note that this might not be necessary; I have defined a `.gitattributes` file that instructs Git to do just that.

## Proposing better ideas

I'm open to suggestions on how to make my current solutions more time efficient. Head over to "Issues" and create a new issue!

## Forking the project

I have structured the code in a way that allows me to add new days as easily as possible.
If you want, you can fork the project to enjoy the same benefits. After all, the code is MIT licensed.
