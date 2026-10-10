//! This package contains a set of simple easing functions, as visualised on
//! [easings.net](https://easings.net/). Function names follow easings.net, e.g. `easeInOutCubic`
//! is [`cubic_in_out`].
//!
//! Each function takes the progress of a transition as `t` in the range `0.0..=1.0` and returns
//! the eased progress, which can be used to interpolate between the start (`0.0`) and the end
//! (`1.0`) state. The result might leave the `0..=1` range temporarily (e.g. `back_*` and
//! `elastic_*`).
//!
//! All functions share the signature `fn(f32) -> f32`, so they can be stored as fn pointers of
//! type [`EasingFn`]:
//!
//! ```
//! use ::simple_easing::{EasingFn, cubic_in_out, roundtrip};
//!
//! let easing: EasingFn = cubic_in_out;
//! let value = easing(0.5);
//! assert!((value - 0.5).abs() < 0.001);
//!
//! // `roundtrip` goes from `0.0` to `1.0` and back, and composes with any other easing
//! let pulse = easing(roundtrip(0.75));
//! ```
//!
//! Custom easings are just closures. As long as they don't capture variables, they are
//! [`EasingFn`]s too:
//!
//! ```
//! use ::simple_easing::{EasingFn, back_out, cubic_in, roundtrip};
//!
//! let pulse: EasingFn = |t| cubic_in(roundtrip(t));
//! let half_back: EasingFn = |t| back_out(t) * 0.5;
//! ```
//!
//! To also accept closures that capture variables, use the [`AnyEasingFn`] trait:
//!
//! ```
//! use ::simple_easing::{AnyEasingFn, cubic_in};
//!
//! let power = 2.5;
//! let easings: Vec<Box<dyn AnyEasingFn>> = vec![
//!     Box::new(cubic_in),
//!     Box::new(move |t: f32| t.powf(power)),
//! ];
//!
//! fn halfway(easing: impl AnyEasingFn) -> f32 {
//!     easing(0.5)
//! }
//! let a = halfway(cubic_in);
//! let b = halfway(&easings[1]);
//! ```

#![warn(clippy::pedantic, clippy::nursery)]
#![allow(clippy::missing_const_for_fn)]

mod back;
mod bounce;
mod circ;
mod cubic;
mod elastic;
mod expo;
mod quad;
mod quart;
mod quint;
mod sine;

pub use back::*;
pub use bounce::*;
pub use circ::*;
pub use cubic::*;
pub use elastic::*;
pub use expo::*;
pub use quad::*;
pub use quart::*;
pub use quint::*;
pub use sine::*;

/// The signature shared by all easing functions of this crate.
pub type EasingFn = fn(f32) -> f32;

/// Any easing: the functions of this crate, [`EasingFn`]s and closures, including ones that
/// capture variables.
///
/// ```
/// use ::simple_easing::{AnyEasingFn, back_in, cubic_in};
///
/// fn sample(easing: impl AnyEasingFn) -> f32 {
///     easing(0.5)
/// }
///
/// let strength = 0.5;
/// let easings: Vec<Box<dyn AnyEasingFn>> = vec![
///     Box::new(cubic_in),
///     Box::new(move |t| back_in(t) * strength),
/// ];
/// assert_eq!(sample(&easings[0]), 0.125);
/// ```
pub trait AnyEasingFn: Fn(f32) -> f32 {}

impl<F: Fn(f32) -> f32> AnyEasingFn for F {}

#[inline]
#[must_use]
pub fn linear(t: f32) -> f32 {
	t
}

/// A linear easing that goes from `1.0` to `0.0`.
#[inline]
#[must_use]
pub fn reverse(t: f32) -> f32 {
	1.0 - t
}

/// A linear easing that goes from `0.0` to `1.0` and back to `0.0`. That might be used in
/// combination with other easing functions.
///
/// ## Example
/// ```
/// use ::simple_easing::{cubic_in, roundtrip};
/// let ascending = cubic_in(roundtrip(0.25));
/// let descending = cubic_in(roundtrip(0.75));
/// assert!((ascending - descending).abs() < 0.001);
/// ```
#[inline]
#[must_use]
pub fn roundtrip(t: f32) -> f32 {
	if t < 0.5 { t * 2.0 } else { (1.0 - t) * 2.0 }
}
