# Rust Web Example: *RESTful Axum*
##### Cole Nemec 2024.
>This is repository for my work in Bart Massey's CS410P taught in Spring '24 at PSU.

The present code is based on programatik29's `axum-tutorial`[here](https://github.com/programatik29/axum-tutorial/tree/master?tab=readme-ov-file) which I'm following to familiarize myself with Axum, Tokio, and webdev with Rust. 
Right now, this repository is structured as a Rust workspace. Ultimately, it will be one cohesive project.

## Run the code:
### `hello-world`:
Compile and run `hello-world` from the base of the workspace with `cargo run --bin hello-world`. Then, visit `http://localhost:3000` in a browser.

### `generate-random-number`:
Compile and run `generate-random-number` from the base of the workspace with `cargo run --bin generate-random-number`. Then, visit `http://localhost:3000/?start=<x>&end=<y>` where `x < y` represent the low-point and high-point, respectively, of the range in which to generate a number.
Passing any `x > y` will cause a panic and nothing will display.
 
