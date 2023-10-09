# nanotsc a small alternative to [tsc](https://www.typescriptlang.org/docs/handbook/compiler-options.html).

If you wanted to transpile TypeScript to JS without having to install tsc.

Just run *nanotsc* followed the names of the files you want to compile

*nanotsc can be as small as 3mb when compiled with ```cargo build --release```*

# To-Do:
* add a flag to enable/disable error checking (currently doesn't check for errors (uses unwrap_unchecked))
* add WebAssembly support