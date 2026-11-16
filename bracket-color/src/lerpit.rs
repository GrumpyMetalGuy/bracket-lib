use crate::prelude::{HSV, RGB, RGBA};
use core::iter::{ExactSizeIterator, Iterator};
use std::convert::TryInto;

/// Implements an RGB Lerp as an iterator.
///
/// `steps` is the number of intervals between `start` and `end`; the
/// iterator yields `steps + 1` points, inclusive of both endpoints.
pub struct RgbLerp {
    /// Starting color
    start: RGB,
    /// Ending color
    end: RGB,
    /// Number of lerp intervals between start and end
    n_steps: usize,
    /// Current step (modified by the iterator)
    step: usize,
}

impl RgbLerp {
    /// Creates a new RGB lerp iterator. The iterator smoothly transitions between two colors,
    /// yielding `steps + 1` points including both endpoints.
    ///
    /// # Arguments
    ///
    /// * `start` - the color to start from.
    /// * `end` - the color to end at on the final step.
    /// * `steps` - number of intervals between the start and end colors.
    ///
    /// # Example
    ///
    /// ```rust
    /// use bracket_color::prelude::*;
    /// for color in RgbLerp::new(RGB::named(RED), RGB::named(YELLOW), 20) {
    ///     println!("{:?}", color); // In-between color
    /// }
    /// ```
    #[inline]
    pub fn new<T>(start: RGB, end: RGB, steps: T) -> Self
    where
        T: TryInto<usize>,
    {
        Self {
            start,
            end,
            n_steps: steps
                .try_into()
                .ok()
                .expect("Not a usize-convertible integer"),
            step: 0,
        }
    }

    /// Number of intervals between the start and end colors.
    #[inline]
    #[must_use]
    pub fn intervals(&self) -> usize {
        self.n_steps
    }
}

impl Iterator for RgbLerp {
    type Item = RGB;

    /// Returns the next step in the iterator.
    #[inline]
    #[allow(clippy::cast_precision_loss)]
    fn next(&mut self) -> Option<RGB> {
        if self.n_steps == 0 {
            if self.step == 0 {
                self.step += 1;
                Some(self.start)
            } else {
                None
            }
        } else if self.step <= self.n_steps {
            let percent = self.step as f32 / self.n_steps as f32;
            self.step += 1;
            Some(self.start.lerp(self.end, percent))
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.len();
        (remaining, Some(remaining))
    }
}

impl ExactSizeIterator for RgbLerp {
    #[inline]
    fn len(&self) -> usize {
        if self.n_steps == 0 {
            1usize.saturating_sub(self.step)
        } else {
            (self.n_steps + 1).saturating_sub(self.step)
        }
    }
}

/// An HSV Lerp - transition from one HSV color to another in a set number of steps.
pub struct HsvLerp {
    /// The starting color
    start: HSV,
    /// The ending color
    end: HSV,
    /// Number of lerp intervals between start and end
    n_steps: usize,
    /// The current step (modified by the iterator)
    step: usize,
}

impl HsvLerp {
    /// Creates a new `HsvLerp` iterator.
    #[inline]
    pub fn new<T>(start: HSV, end: HSV, steps: T) -> Self
    where
        T: TryInto<usize>,
    {
        Self {
            start,
            end,
            n_steps: steps.try_into().ok().expect("Not an integer"),
            step: 0,
        }
    }
}

impl Iterator for HsvLerp {
    type Item = HSV;

    #[inline]
    #[allow(clippy::cast_precision_loss)]
    fn next(&mut self) -> Option<HSV> {
        if self.n_steps == 0 {
            if self.step == 0 {
                self.step += 1;
                Some(self.start)
            } else {
                None
            }
        } else if self.step <= self.n_steps {
            let percent = self.step as f32 / self.n_steps as f32;
            self.step += 1;
            Some(self.start.lerp(self.end, percent))
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.len();
        (remaining, Some(remaining))
    }
}

impl ExactSizeIterator for HsvLerp {
    #[inline]
    fn len(&self) -> usize {
        if self.n_steps == 0 {
            1usize.saturating_sub(self.step)
        } else {
            (self.n_steps + 1).saturating_sub(self.step)
        }
    }
}

/// Implements an RGBA Lerp as an iterator.
pub struct RgbaLerp {
    /// Starting color
    start: RGBA,
    /// Ending color
    end: RGBA,
    /// Number of lerp intervals between start and end
    n_steps: usize,
    /// Current step (modified by the iterator)
    step: usize,
}

impl RgbaLerp {
    /// Creates a new RGBA lerp iterator.
    #[inline]
    pub fn new<T>(start: RGBA, end: RGBA, steps: T) -> Self
    where
        T: TryInto<usize>,
    {
        Self {
            start,
            end,
            n_steps: steps
                .try_into()
                .ok()
                .expect("Not a usize-convertible integer"),
            step: 0,
        }
    }
}

impl Iterator for RgbaLerp {
    type Item = RGBA;

    #[inline]
    #[allow(clippy::cast_precision_loss)]
    fn next(&mut self) -> Option<RGBA> {
        if self.n_steps == 0 {
            if self.step == 0 {
                self.step += 1;
                Some(self.start)
            } else {
                None
            }
        } else if self.step <= self.n_steps {
            let percent = self.step as f32 / self.n_steps as f32;
            self.step += 1;
            Some(self.start.lerp(self.end, percent))
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.len();
        (remaining, Some(remaining))
    }
}

impl ExactSizeIterator for RgbaLerp {
    #[inline]
    fn len(&self) -> usize {
        if self.n_steps == 0 {
            1usize.saturating_sub(self.step)
        } else {
            (self.n_steps + 1).saturating_sub(self.step)
        }
    }
}

/// Implements an Alpha-Only Lerp as an iterator.
pub struct AlphaLerp {
    /// Starting color
    start: RGBA,
    /// Ending color
    end: RGBA,
    /// Number of lerp intervals between start and end
    n_steps: usize,
    /// Current step (modified by the iterator)
    step: usize,
}

impl AlphaLerp {
    /// Creates a new alpha lerp iterator.
    #[inline]
    pub fn new<T>(start: RGBA, end: RGBA, steps: T) -> Self
    where
        T: TryInto<usize>,
    {
        Self {
            start,
            end,
            n_steps: steps
                .try_into()
                .ok()
                .expect("Not a usize-convertible integer"),
            step: 0,
        }
    }
}

impl Iterator for AlphaLerp {
    type Item = RGBA;

    #[inline]
    #[allow(clippy::cast_precision_loss)]
    fn next(&mut self) -> Option<RGBA> {
        if self.n_steps == 0 {
            if self.step == 0 {
                self.step += 1;
                Some(self.start)
            } else {
                None
            }
        } else if self.step <= self.n_steps {
            let percent = self.step as f32 / self.n_steps as f32;
            self.step += 1;
            Some(self.start.lerp_alpha(self.end, percent))
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.len();
        (remaining, Some(remaining))
    }
}

impl ExactSizeIterator for AlphaLerp {
    #[inline]
    fn len(&self) -> usize {
        if self.n_steps == 0 {
            1usize.saturating_sub(self.step)
        } else {
            (self.n_steps + 1).saturating_sub(self.step)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn rgb_lerp_yields_steps_plus_one_inclusive_endpoints() {
        let colors: Vec<_> = RgbLerp::new(RGB::named(RED), RGB::named(YELLOW), 4).collect();
        assert_eq!(colors.len(), 5);
        assert_eq!(colors[0], RGB::named(RED));
        assert_eq!(colors[4], RGB::named(YELLOW));
    }

    #[test]
    fn rgb_lerp_len_tracks_remaining() {
        let mut lerp = RgbLerp::new(RGB::named(RED), RGB::named(YELLOW), 4);
        assert_eq!(lerp.len(), 5);
        lerp.next();
        assert_eq!(lerp.len(), 4);
        assert_eq!(lerp.size_hint(), (4, Some(4)));
    }

    #[test]
    fn rgb_lerp_zero_steps_yields_start_once() {
        let colors: Vec<_> = RgbLerp::new(RGB::named(RED), RGB::named(YELLOW), 0).collect();
        assert_eq!(colors.len(), 1);
        assert_eq!(colors[0], RGB::named(RED));
    }

    #[test]
    fn hsv_and_rgba_and_alpha_lerp_counts() {
        assert_eq!(HsvLerp::new(RGB::named(RED).into(), RGB::named(YELLOW).into(), 3).count(), 4);
        assert_eq!(
            RgbaLerp::new(RGBA::named(RED), RGBA::named(YELLOW), 3).count(),
            4
        );
        assert_eq!(
            AlphaLerp::new(RGBA::named(RED), RGBA::named(YELLOW), 3).count(),
            4
        );
    }
}
