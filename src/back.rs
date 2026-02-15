const C1: f32 = 1.70158;
const C2: f32 = C1 * 1.525;
const C3: f32 = C1 + 1.0;

/// <https://easings.net/#easeInBack>
#[inline]
#[must_use]
pub fn back_in(t: f32) -> f32 {
	C3 * t * t * t - C1 * t * t
}

/// <https://easings.net/#easeOutBack>
#[inline]
#[must_use]
pub fn back_out(t: f32) -> f32 {
	1.0 + C3 * (t - 1.0).powi(3) + C1 * (t - 1.0).powi(2)
}

/// <https://easings.net/#easeInOutBack>
#[inline]
#[must_use]
pub fn back_in_out(t: f32) -> f32 {
	if t < 0.5 {
		((2.0 * t).powi(2) * ((C2 + 1.0) * 2.0 * t - C2)) / 2.0
	} else {
		((2.0 * t - 2.0).powi(2) * ((C2 + 1.0) * (t * 2.0 - 2.0) + C2) + 2.0) / 2.0
	}
}
