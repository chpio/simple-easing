/// <https://easings.net/#easeInCubic>
#[inline]
pub fn cubic_in(t: f32) -> f32 {
	t * t * t
}

/// <https://easings.net/#easeOutCubic>
#[inline]
pub fn cubic_out(t: f32) -> f32 {
	1.0 - (1.0 - t).powi(3)
}

/// <https://easings.net/#easeInOutCubic>
#[inline]
pub fn cubic_in_out(t: f32) -> f32 {
	if t < 0.5 {
		4.0 * t * t * t
	} else {
		1.0 - (-2.0 * t + 2.0).powi(3) / 2.0
	}
}
