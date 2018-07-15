# Rust solution

## How to build

The easiest way would be using [rustup](https://rustup.rs/).

I have tested it with the `nightly` version.
`> rustup install nightly`
`> rustup default nightly`

Once the full toolchain is installed you should only need to run

`> cargo build` with the optional flag `--release`.

## How to run

The compiled binaries should be at `target`. Depending on the presence of `--release` it could be different.
Ex:
`> target/debug/waldo-rust img1.jpg img2.jpg`
`> target/release/waldo-rust img1.jpg img2.jpg`

## Why write a naive solution?

The OpenCV python solution is really simple and just uses implemented libraries to do a task that seems the core of the problem.
I wanted to implement something bigger than programming challenges with Rust, to get a handle of it, and decided that this is a good chance.

## Any tests?

I know that there aren't any tests, just the ones I made manually.
I would like to write automated tests but I believe this is more a task of integration testing than unit testing. For that one would need a large set of images and subimages with the correct solutions.

A middle point would be to have 2 big images and take random crops from both and check that they match the origin and the correct image.

## Is it worth?

I don't think it is worth as more than an intellectual challenge. My light knowledge of Rust just let's me implement it in a really inefficient way compared to the more than likely finetuned OpenCV implementation.

I think this code needs to be profiled and optimized to get to an usable level and by no ways proves or disproves any thing about Rust compiled speed.
