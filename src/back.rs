const C1: f32 = 1.70158;
const C2: f32 = C1 * 1.525;
const C3: f32 = C1 + 1.0;

/// <https://easings.net/#easeInBack>
#[inline]
#[must_use]
pub fn back_in(t: f32) -> f32 {
	(C3 * t * t).mul_add(t, -(C1 * t * t))
}

/// <https://easings.net/#easeOutBack>
#[inline]
#[must_use]
pub fn back_out(t: f32) -> f32 {
	C1.mul_add((t - 1.0).powi(2), C3.mul_add((t - 1.0).powi(3), 1.0))
}

/// <https://easings.net/#easeInOutBack>
#[inline]
#[must_use]
pub fn back_in_out(t: f32) -> f32 {
	if t < 0.5 {
		((2.0 * t).powi(2) * ((C2 + 1.0) * 2.0).mul_add(t, -C2)) / 2.0
	} else {
		f32::midpoint(
			2.0f32.mul_add(t, -2.0).powi(2) * (C2 + 1.0).mul_add(t.mul_add(2.0, -2.0), C2),
			2.0,
		)
	}
}
