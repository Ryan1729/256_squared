I was stuck by the following question:

Given you interpreted two bytes as (x, y) coordinates within a 256 by 256 grid, what would it look like if you iterated over all the possible coordinates and bitwse-OR-ed in `0b1111` into each byte and marked cells in the grid based on the number of coordinates that landed in a particular grid cell.

 I couldn't easily do the calculations to visualize it in my head so I decided to make a computer do that. Probably I will add a way to select different operations as well.

[live version](https://ryan1729.github.io/256_squared/)

### Building (using Rust's native WebAssembly backend)

1. Install newest nightly Rust:

       $ curl https://sh.rustup.rs -sSf | sh

2. Install WebAssembly target:

       $ rustup target add wasm32-unknown-unknown

3. Install [cargo-web]:

       $ cargo install -f cargo-web

4. Build it:

       $ cargo web start --target=wasm32-unknown-unknown --release

5. Visit `http://localhost:8000` with your browser.

[cargo-web]: https://github.com/koute/cargo-web

### Building for other backends

Replace `--target=wasm32-unknown-unknown` with `--target=wasm32-unknown-emscripten` or `--target=asmjs-unknown-emscripten`
if you want to build it using another backend. You will also have to install the
corresponding targets with `rustup` - `wasm32-unknown-emscripten` and `asmjs-unknown-emscripten`
respectively.
