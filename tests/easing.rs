use ::simple_easing::*;
use ::std::f64::consts::PI;

type Reference = fn(f64) -> f64;

/// Every easing function next to an `f64` reference implementation, transcribed from the
/// formulas on <https://easings.net/>.
const REFERENCES: &[(&str, EasingFn, Reference)] = &[
	("linear", linear, |x| x),
	("reverse", reverse, |x| 1.0 - x),
	("roundtrip", roundtrip, |x| {
		if x < 0.5 { 2.0 * x } else { 2.0 - 2.0 * x }
	}),
	("sine_in", sine_in, |x| 1.0 - (x * PI / 2.0).cos()),
	("sine_out", sine_out, |x| (x * PI / 2.0).sin()),
	("sine_in_out", sine_in_out, |x| {
		-((PI * x).cos() - 1.0) / 2.0
	}),
	("quad_in", quad_in, |x| x * x),
	("quad_out", quad_out, |x| 1.0 - (1.0 - x) * (1.0 - x)),
	("quad_in_out", quad_in_out, |x| {
		if x < 0.5 {
			2.0 * x * x
		} else {
			1.0 - (-2.0 * x + 2.0).powi(2) / 2.0
		}
	}),
	("cubic_in", cubic_in, |x| x * x * x),
	("cubic_out", cubic_out, |x| 1.0 - (1.0 - x).powi(3)),
	("cubic_in_out", cubic_in_out, |x| {
		if x < 0.5 {
			4.0 * x * x * x
		} else {
			1.0 - (-2.0 * x + 2.0).powi(3) / 2.0
		}
	}),
	("quart_in", quart_in, |x| x * x * x * x),
	("quart_out", quart_out, |x| 1.0 - (1.0 - x).powi(4)),
	("quart_in_out", quart_in_out, |x| {
		if x < 0.5 {
			8.0 * x * x * x * x
		} else {
			1.0 - (-2.0 * x + 2.0).powi(4) / 2.0
		}
	}),
	("quint_in", quint_in, |x| x * x * x * x * x),
	("quint_out", quint_out, |x| 1.0 - (1.0 - x).powi(5)),
	("quint_in_out", quint_in_out, |x| {
		if x < 0.5 {
			16.0 * x * x * x * x * x
		} else {
			1.0 - (-2.0 * x + 2.0).powi(5) / 2.0
		}
	}),
	("expo_in", expo_in, |x| {
		if x == 0.0 {
			0.0
		} else {
			2f64.powf(10.0 * x - 10.0)
		}
	}),
	("expo_out", expo_out, |x| {
		if x == 1.0 {
			1.0
		} else {
			1.0 - 2f64.powf(-10.0 * x)
		}
	}),
	("expo_in_out", expo_in_out, |x| {
		if x == 0.0 {
			0.0
		} else if x == 1.0 {
			1.0
		} else if x < 0.5 {
			2f64.powf(20.0 * x - 10.0) / 2.0
		} else {
			(2.0 - 2f64.powf(-20.0 * x + 10.0)) / 2.0
		}
	}),
	("circ_in", circ_in, |x| 1.0 - (1.0 - x * x).sqrt()),
	("circ_out", circ_out, |x| (1.0 - (x - 1.0).powi(2)).sqrt()),
	("circ_in_out", circ_in_out, |x| {
		if x < 0.5 {
			(1.0 - (1.0 - (2.0 * x).powi(2)).sqrt()) / 2.0
		} else {
			((1.0 - (-2.0 * x + 2.0).powi(2)).sqrt() + 1.0) / 2.0
		}
	}),
	("back_in", back_in, |x| {
		BACK_C3 * x * x * x - BACK_C1 * x * x
	}),
	("back_out", back_out, |x| {
		1.0 + BACK_C3 * (x - 1.0).powi(3) + BACK_C1 * (x - 1.0).powi(2)
	}),
	("back_in_out", back_in_out, |x| {
		if x < 0.5 {
			((2.0 * x).powi(2) * ((BACK_C2 + 1.0) * 2.0 * x - BACK_C2)) / 2.0
		} else {
			((2.0 * x - 2.0).powi(2) * ((BACK_C2 + 1.0) * (x * 2.0 - 2.0) + BACK_C2) + 2.0) / 2.0
		}
	}),
	("elastic_in", elastic_in, |x| {
		if x == 0.0 {
			0.0
		} else if x == 1.0 {
			1.0
		} else {
			-2f64.powf(10.0 * x - 10.0) * ((x * 10.0 - 10.75) * ELASTIC_C4).sin()
		}
	}),
	("elastic_out", elastic_out, |x| {
		if x == 0.0 {
			0.0
		} else if x == 1.0 {
			1.0
		} else {
			2f64.powf(-10.0 * x) * ((x * 10.0 - 0.75) * ELASTIC_C4).sin() + 1.0
		}
	}),
	("elastic_in_out", elastic_in_out, |x| {
		if x == 0.0 {
			0.0
		} else if x == 1.0 {
			1.0
		} else if x < 0.5 {
			-(2f64.powf(20.0 * x - 10.0) * ((20.0 * x - 11.125) * ELASTIC_C5).sin()) / 2.0
		} else {
			(2f64.powf(-20.0 * x + 10.0) * ((20.0 * x - 11.125) * ELASTIC_C5).sin()) / 2.0 + 1.0
		}
	}),
	("bounce_in", bounce_in, |x| {
		1.0 - reference_bounce_out(1.0 - x)
	}),
	("bounce_out", bounce_out, reference_bounce_out),
	("bounce_in_out", bounce_in_out, |x| {
		if x < 0.5 {
			(1.0 - reference_bounce_out(1.0 - 2.0 * x)) / 2.0
		} else {
			(1.0 + reference_bounce_out(2.0 * x - 1.0)) / 2.0
		}
	}),
];

/// All `*_in`, `*_out` and `*_in_out` triples.
const FAMILIES: &[(&str, [EasingFn; 3])] = &[
	("sine", [sine_in, sine_out, sine_in_out]),
	("quad", [quad_in, quad_out, quad_in_out]),
	("cubic", [cubic_in, cubic_out, cubic_in_out]),
	("quart", [quart_in, quart_out, quart_in_out]),
	("quint", [quint_in, quint_out, quint_in_out]),
	("expo", [expo_in, expo_out, expo_in_out]),
	("circ", [circ_in, circ_out, circ_in_out]),
	("back", [back_in, back_out, back_in_out]),
	("elastic", [elastic_in, elastic_out, elastic_in_out]),
	("bounce", [bounce_in, bounce_out, bounce_in_out]),
];

const BACK_C1: f64 = 1.70158;
const BACK_C2: f64 = BACK_C1 * 1.525;
const BACK_C3: f64 = BACK_C1 + 1.0;
const ELASTIC_C4: f64 = (2.0 * PI) / 3.0;
const ELASTIC_C5: f64 = (2.0 * PI) / 4.5;

fn reference_bounce_out(x: f64) -> f64 {
	const N1: f64 = 7.5625;
	const D1: f64 = 2.75;
	if x < 1.0 / D1 {
		N1 * x * x
	} else if x < 2.0 / D1 {
		N1 * (x - 1.5 / D1).powi(2) + 0.75
	} else if x < 2.5 / D1 {
		N1 * (x - 2.25 / D1).powi(2) + 0.9375
	} else {
		N1 * (x - 2.625 / D1).powi(2) + 0.984375
	}
}

const SAMPLES: u16 = 1000;

fn samples() -> impl Iterator<Item = f32> {
	(0..=SAMPLES).map(|i| f32::from(i) / f32::from(SAMPLES))
}

fn assert_close(actual: f32, expected: f64, tolerance: f64, context: &str) {
	let diff = (f64::from(actual) - expected).abs();
	assert!(
		diff <= tolerance,
		"{context}: got {actual}, expected {expected} (diff {diff})"
	);
}

#[test]
fn matches_reference() {
	for &(name, easing, reference) in REFERENCES {
		for t in samples() {
			assert_close(
				easing(t),
				reference(f64::from(t)),
				1e-5,
				&format!("{name}({t})"),
			);
		}
	}
}

#[test]
fn starts_at_zero_and_ends_at_one() {
	for &(name, family) in FAMILIES {
		for (easing, suffix) in family.into_iter().zip(["in", "out", "in_out"]) {
			assert_close(easing(0.0), 0.0, 1e-6, &format!("{name}_{suffix}(0.0)"));
			assert_close(easing(1.0), 1.0, 1e-6, &format!("{name}_{suffix}(1.0)"));
		}
	}
}

#[test]
fn out_mirrors_in() {
	for &(name, [ease_in, ease_out, _]) in FAMILIES {
		for t in samples() {
			assert_close(
				ease_out(t),
				f64::from(1.0 - ease_in(1.0 - t)),
				1e-5,
				&format!("{name}_out({t}) vs. 1 - {name}_in(1 - {t})"),
			);
		}
	}
}

#[test]
fn in_out_is_point_symmetric() {
	for &(name, [_, _, ease_in_out]) in FAMILIES {
		for t in samples() {
			assert_close(
				ease_in_out(t),
				f64::from(1.0 - ease_in_out(1.0 - t)),
				1e-5,
				&format!("{name}_in_out({t}) vs. 1 - {name}_in_out(1 - {t})"),
			);
		}
	}
}

#[test]
fn in_out_is_continuous_at_half() {
	let just_below_half = f32::from_bits(0.5f32.to_bits() - 1);
	for &(name, [_, _, ease_in_out]) in FAMILIES {
		// loose tolerance: `circ_in_out` has a vertical tangent at `0.5`
		assert_close(
			ease_in_out(just_below_half),
			f64::from(ease_in_out(0.5)),
			1e-3,
			&format!("{name}_in_out around 0.5"),
		);
	}
}
