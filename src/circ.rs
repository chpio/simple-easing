/// <https://easings.net/#easeInCirc>
#[inline]
pub fn circ_in(t: f32) -> f32 {
	1.0 - (1.0 - t.powi(2)).sqrt()
}

/// <https://easings.net/#easeOutCirc>
#[inline]
pub fn circ_out(t: f32) -> f32 {
	(1.0 - (t - 1.0).powi(2)).sqrt()
}

/// <https://easings.net/#easeInOutCirc>
#[inline]
pub fn circ_in_out(t: f32) -> f32 {
	if t < 0.5 {
		(1.0 - (1.0 - (2.0 * t).powi(2)).sqrt()) / 2.0
	} else {
		((1.0 - (-2.0 * t + 2.0).powi(2)).sqrt() + 1.0) / 2.0
	}
}
