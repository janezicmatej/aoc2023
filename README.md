![ci](https://github.com/janezicmatej/aoc2023/actions/workflows/ci.yml/badge.svg)
# Advent-of-Code 2023
*This is a dumbed down version of [fspoettel/advent-of-code-rust](https://github.com/fspoettel/advent-of-code-rust) with some extra features*

## Project overview

### Project structure
- `data/` :
    - `examples/`: example files go here; you can push this as test are run in ci
    - `inputs/`: this directory is gitignored, input files go here
- `src/` :
    - `bin/`:
        - `<day>.rs`: solution files 
    - `lib.rs`: library entrypoint, reusable code goes here
    - `template.rs`: contains template code
- `utils/`: binary packages with convenience scripts structured using cargo workspaces
- `.env.example`: example dotenv file

### Cli
- `cargo scaffold <day>`: prepare solution files for `day`
- `cargo download <day>`: download input file for `day`
- `cargo solve <day>`: run solution against input for `day`

*Run `cargo build --workspace --release` once so scaffold and download packages get compiled, otherwise they will have to be compiled on first run.*

### dotenv

set `YEAR` to whichever year you are solving for and `TOKEN` to AoC session Cookie

### FAQ

#### How are your commits numbered in ascending order?
[https://westling.dev/b/extremely-linear-git](https://westling.dev/b/extremely-linear-git)
