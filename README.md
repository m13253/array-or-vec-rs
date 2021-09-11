# array-or-vec-rs

Rust trait for generic functions that need either a compile-time array or a dynamic vector

## Usage

Put this to your `Cargo.toml`:
```toml
[dependencies]
array-or-vec = { "git" = "https://github.com/m13253/array-or-vec-rs" }
```

Write your code that uses the trait:
```rust
use array_or_vec::ArrayOrVec;
use num_traits::NumRef;  // From crate "num_traits"

fn sum<T, A>(a: &A) -> T
where
    T: NumRef,
    A: ArrayOrVec<T>,
{
    a.iter().fold(T::zero(), |acc, x| acc + x)
}
```

## FAQ

1. Why don't you publish this package to crates.io?

   Because I can't find a good name for this package.
   Crates.io doesn't allow me to change the name after publishing.
