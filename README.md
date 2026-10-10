# simple-easing

This package contains a set of simple easing functions, as visualised on
[easings.net](https://easings.net/).

Each function takes the progress of a transition as `t` in the range `0.0..=1.0` and returns
the eased progress, which can be used to interpolate between the start (`0.0`) and the end
(`1.0`) state. The result might leave the `0..=1` range temporarily (e.g. `back_*` and
`elastic_*`).

All functions share the signature `fn(f32) -> f32`, so they can be stored as fn pointers:

```rust
use ::simple_easing::{cubic_in_out, roundtrip};

let easing: fn(f32) -> f32 = cubic_in_out;
let value = easing(0.5);
assert!((value - 0.5).abs() < 0.001);

// `roundtrip` goes from `0.0` to `1.0` and back, and composes with any other easing
let pulse = easing(roundtrip(0.75));
```

## Functions

`linear`, `reverse`, `roundtrip`, and an `*_in`, `*_out` and `*_in_out` variant of each of
`sine`, `quad`, `cubic`, `quart`, `quint`, `expo`, `circ`, `back`, `elastic` and `bounce`.

## License

Licensed under either of MIT or Apache-2.0, at your option.
