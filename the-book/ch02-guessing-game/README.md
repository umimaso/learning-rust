# Chapter 2 Notes

## Code
- Rust by default automatically imports a list of things into every Rust program. This is known as
  [the prelude](https://doc.rust-lang.org/stable/std/prelude/index.html)
- `use` statement allows usage of types not in the prelude
- Variables, and references in Rust are immutable by default
- Variable names can be reused for setting new values, and this is known as *shadowing*
- `let` statement is used for creating variables
- `mut` allows for making variables, and references mutable, e.g. `let mut guess`, `&mut guess`
- `:` after a variable name can be used for annotating a variable's type, e.g. `let guess: u32`
- `{}` is used in for marking placeholders in strings for formatting
- `&` indicates that an argument is a reference (allow access without needing to copy the data into memory multiple times)
- `match` expression is made up of *arms*, an arm consisting of a *pattern* to match against, and the code to run should the value given to match fit the arm pattern
- `loop` keyword creates an infinite loop. `continue` for next iteration, `break` to leave
- `type::func` indicates that func is an associated function of type, e.g. `String::new()`
- `Result` type has methods like any other. The `expect` method can be used here to check if the instance of `io::Result` is a `Err` value, and in turn crash the program to stop further use
- The `match` expression can be used instead of `expect` to move from crashing on error to handling it. To match on all error types the catchall value `_` can be used e.g. `Err(_)`

## Crates
- Rust doesn't include random number functionality in its standard library yet. So a crate that provides this functionality has to be used
- Crate dependencies are set under the `[dependencies]` section of the `Cargo.toml` file. These can be pinned against versions, and Cargo understands semantic versioning
- Cargo fetches the latest versions of everything that a dependency needs from the registry (which is a copy of data from [Crates.io](https://crates.io))
- With the crates downloaded, Rust will compile them for use. This complication won't need to reoccur on further builds as Cargo will keep track
- Documentation of all the dependencies can be built, and displayed locally with `cargo doc --open`

## Cargo.lock
- Cargo.lock allows for reproducible builds by keeping track of the dependency versions used when a project is built for the first time
- To update a crate `cargo update` can be used, and this will ignore the Cargo.lock file, and figure out all the latest versions again, which Cargo will write to Cargo.lock
- If changing a crate's version in Cargo.toml then on next build, Cargo will update the registry of crates, and reevaluate the requirements according to the new version given
