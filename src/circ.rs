/// <https://easings.net/#easeInCirc>
#[inline]
#[must_use]
pub fn circ_in(t: f32) -> f32 {
	1.0 - t.mul_add(-t, 1.0).sqrt()
}

/// <https://easings.net/#easeOutCirc>
#[inline]
#[must_use]
pub fn circ_out(t: f32) -> f32 {
	(t - 1.0).mul_add(-(t - 1.0), 1.0).sqrt()
}

/// <https://easings.net/#easeInOutCirc>
#[inline]
#[must_use]
pub fn circ_in_out(t: f32) -> f32 {
	if t < 0.5 {
		(1.0 - (2.0 * t).mul_add(-(2.0 * t), 1.0).sqrt()) / 2.0
	} else {
		f32::midpoint(
			(-2.0f32)
				.mul_add(t, 2.0)
				.mul_add(-(-2.0f32).mul_add(t, 2.0), 1.0)
				.sqrt(),
			1.0,
		)
	}
}
