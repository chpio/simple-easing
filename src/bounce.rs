/// <https://easings.net/#easeInBounce>
#[inline]
#[must_use]
pub fn bounce_in(t: f32) -> f32 {
	1.0 - bounce_out(1.0 - t)
}

/// <https://easings.net/#easeOutBounce>
#[inline]
#[must_use]
pub fn bounce_out(t: f32) -> f32 {
	const N1: f32 = 7.5625;
	const D1: f32 = 2.75;
	if t < 1.0 / D1 {
		N1 * t * t
	} else if t < 2.0 / D1 {
		N1 * (t - 1.5 / D1).powi(2) + 0.75
	} else if t < 2.5 / D1 {
		N1 * (t - 2.25 / D1).powi(2) + 0.937_5
	} else {
		N1 * (t - 2.625 / D1).powi(2) + 0.984_375
	}
}

/// <https://easings.net/#easeInOutBounce>
#[inline]
#[must_use]
pub fn bounce_in_out(t: f32) -> f32 {
	if t < 0.5 {
		(1.0 - bounce_out(1.0 - 2.0 * t)) / 2.0
	} else {
		f32::midpoint(1.0, bounce_out(2.0 * t - 1.0))
	}
}
