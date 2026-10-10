# simple-easing

This package contains a set of simple easing functions, as visualised on
[easings.net](https://easings.net/).

Each function takes the progress of a transition as `t` in the range `0.0..=1.0` and returns
the eased progress, which can be used to interpolate between the start (`0.0`) and the end
(`1.0`) state. The result might leave the `0..=1` range temporarily (e.g. `back_*` and
`elastic_*`).

All functions share the signature `fn(f32) -> f32`, so they can be stored as fn pointers of
type `EasingFn`:

```rust
use ::simple_easing::{EasingFn, cubic_in_out, roundtrip};

let easing: EasingFn = cubic_in_out;
let value = easing(0.5);
assert!((value - 0.5).abs() < 0.001);

// `roundtrip` goes from `0.0` to `1.0` and back, and composes with any other easing
let pulse = easing(roundtrip(0.75));
```

Custom easings are just closures. As long as they don't capture variables, they are
`EasingFn`s too:

```rust
use ::simple_easing::{EasingFn, back_out, cubic_in, roundtrip};

let pulse: EasingFn = |t| cubic_in(roundtrip(t));
let half_back: EasingFn = |t| back_out(t) * 0.5;
```

To also accept closures that capture variables, use the `AnyEasingFn` trait:

```rust
use ::simple_easing::{AnyEasingFn, cubic_in};

let power = 2.5;
let easings: Vec<Box<dyn AnyEasingFn>> = vec![
	Box::new(cubic_in),
	Box::new(move |t: f32| t.powf(power)),
];

fn halfway(easing: impl AnyEasingFn) -> f32 {
	easing(0.5)
}
let a = halfway(cubic_in);
let b = halfway(&easings[1]);
```

## Functions

`linear`, `reverse`, `roundtrip`, and an `*_in`, `*_out` and `*_in_out` variant of each of
`sine`, `quad`, `cubic`, `quart`, `quint`, `expo`, `circ`, `back`, `elastic` and `bounce`.

## License

Licensed under either of MIT or Apache-2.0, at your option.
