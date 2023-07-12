# Vector2d Library
The following library is created for handling the `vector` for the animation library where we put our `engine`.
The libray has several modules and support

## Changelog
- `2023-07-12 22:52`:
    - Now the implementation work for both `.add()` method and the `operators`
    for all the given operations (addition, subtractions, multiplication,
    division)

- `2023-07-12 17:22`:
    - Finished adding the `verboseVectors` which will let us know when the
    vector gets dropped from the memeory (useful for debugging purposes).
    - Implementing the test for all assocaited methods.
- `2023-07-10 15:36:30`:
    - Create the struct for the `vector2dlib`
    - Support testing for unit test and library documentation test.
    - Imported to the `Engine` crate for our animation platform.

## Notes
- `2023-07-12 17:24`:
    - For the `verboseVectors` I have used the composition of and built on top
    of the vector2d. This was necessary since I cannot add a nother field
    called `verbose` for the struct of `Vector2d` (stand `Add`, `Sub`, `Div`
    and `Mul` needs only two fields).
    - For implementing the `std::fmt::Display` for the `verboseVectors`, I
    found a creative way to extract first the `Some(vector)` since `Option` is
    not implementing for the `Display` trait, check the `std::fmt::Display` trait at the end of the `verbose_vector2d_module.rs` module.

- `2023-07-10 22:49:26`:
    - Up to this point the `Vector2d` supports
        - (+,-,\*,/) operators for the vector
            - Input can be vector or scalar
        - `.add()`, `.div()`, `.mul()` and `.sub()` accepts only `Vector2d` cannot accept `scalar`
            - Meaning that `let s = v.div(8.0)` will not work.

## Features
- [x] Generic format for the vector
- [x] Logging and error handling, check`vectorResult` for example.
- [x] Support traits for display in debugging mode.
- [x] Operators overloading for (+)/(-)/(/)/(\*) for our vector.
- [ ] Adding support for other vector methods - in progress
    - [ ] `lerp` function.
    - [ ] `distance` between two vectors.
    - [ ] `cross` product similar to the dot product.
    - [x] `negate`
    - [x] `zero` vector initalizer.
    - [x] `ones` vector initalizer.

## Welcome message
- You can run the command, for lunching the `src/main.rs` which is not
necessary, but I created for demostration purposes.

```sh
cargo run --quiet
# or
cargo run -j 10 --quiet
```
## Testing Module
- Using the testing command below will run the `test/tests.rs` script along side
with the documentation scripts for each metho in the
`vector2dlib::math::vector_module::Vector2d` of `Vector2d` strcut methods.

```sh
cargo test
```
- Sometimes we want to see the results output of some selected variables by the
specified `test`. To see the printed values during cargo test, you can use the
`--nocapture` flag. This flag prevents the test framework from capturing the
output, allowing the printed values to be displayed in the console.

```sh
cargo test -- --nocapture
# Or
cargo test -j 8 -- --nocapture
```

