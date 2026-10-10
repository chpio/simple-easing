# Changelog

## Unreleased

* add `EasingFn` type alias for `fn(f32) -> f32`
* add `AnyEasingFn` trait, implemented by every `Fn(f32) -> f32` (incl. capturing closures)

## 1.0.3

* repair `quint_in` function
* declare minimum supported rust version (1.85)

## 1.0.2

* use rust edition 2024
* make all functions inlinable
* add `#[must_use]` to all functions
* enable clippy lints and fix

## 1.0.1

* repair `elastic_out` function

## 1.0.0

* First release
