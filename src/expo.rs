/// <https://easings.net/#easeInExpo>
#[inline]
#[must_use]
pub fn expo_in(t: f32) -> f32 {
	if t <= 0.0 {
		0.0
	} else {
		10.0f32.mul_add(t, -10.0).exp2()
	}
}

/// <https://easings.net/#easeOutExpo>
#[inline]
#[must_use]
pub fn expo_out(t: f32) -> f32 {
	if 1.0 <= t {
		1.0
	} else {
		1.0 - (-10.0 * t).exp2()
	}
}

/// <https://easings.net/#easeInOutExpo>
#[inline]
#[must_use]
pub fn expo_in_out(t: f32) -> f32 {
	if t <= 0.0 {
		0.0
	} else if 1.0 <= t {
		1.0
	} else if t < 0.5 {
		20.0f32.mul_add(t, -10.0).exp2() / 2.0
	} else {
		(2.0 - (-20.0f32).mul_add(t, 10.0).exp2()) / 2.0
	}
}
