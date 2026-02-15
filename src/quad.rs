/// <https://easings.net/#easeInQuad>
#[inline]
#[must_use]
pub fn quad_in(t: f32) -> f32 {
	t * t
}

/// <https://easings.net/#easeOutQuad>
#[inline]
#[must_use]
pub fn quad_out(t: f32) -> f32 {
	(1.0 - t).mul_add(-(1.0 - t), 1.0)
}

/// <https://easings.net/#easeInOutQuad>
#[inline]
#[must_use]
pub fn quad_in_out(t: f32) -> f32 {
	if t < 0.5 {
		2.0 * t * t
	} else {
		1.0 - (-2.0f32).mul_add(t, 2.0).powi(2) / 2.0
	}
}
