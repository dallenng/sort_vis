// Taken and adapted from https://github.com/bevyengine/bevy/blob/b416d181a7e489df9a2b35f98e9e8038699586f5/crates/bevy_time/src/fixed_timestep.rs

use std::error::Error;
use std::fmt;
use std::num::NonZeroU8;
use std::time::Duration;

#[cfg(test)]
mod tests;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct FixedTime {
    accumulated: Duration,
    /// The amount of time spanned by each fixed update.
    /// Defaults to 1/60th of a second.
    ///
    /// To configure this value, simply mutate or overwrite this field.
    pub period: Duration,
}

impl FixedTime {
    /// Creates a new [`FixedTime`] struct with a specified period.
    pub fn new(period: Duration) -> Self {
        Self { accumulated: Duration::ZERO, period }
    }

    /// Adds to this instance's accumulated time. `delta_time` should be the
    /// amount of in-game time that has passed since `tick` was last called.
    pub fn tick(&mut self, delta_time: Duration) {
        self.accumulated += delta_time;
    }

    /// Attempts to advance by a single period. The period is modified according
    /// to the `speed`. The greater the speed, the lesser the period. This can
    /// be used to make time faster. This will return [`FixedUpdateError`]
    /// if there is not enough accumulated time -- in other words, if
    /// advancing time would put the fixed update schedule ahead of the main
    /// schedule.
    pub fn expend(&mut self, speed: NonZeroU8) -> Result<(), FixedUpdateError> {
        if let Some(new_value) = self.accumulated.checked_sub(self.period / u32::from(speed.get()))
        {
            self.accumulated = new_value;
            Ok(())
        } else {
            Err(FixedUpdateError::NotEnoughTime {
                accumulated: self.accumulated,
                period: self.period,
            })
        }
    }
}

impl Default for FixedTime {
    fn default() -> Self {
        FixedTime { accumulated: Duration::ZERO, period: Duration::from_secs_f32(1. / 3.) }
    }
}

/// An error returned when working with [`FixedTime`].
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum FixedUpdateError {
    /// There is not enough accumulated time to advance the fixed update
    /// schedule.
    NotEnoughTime {
        /// The amount of time available to advance the fixed update schedule.
        accumulated: Duration,
        /// The length of one fixed update.
        period: Duration,
    },
}

impl fmt::Display for FixedUpdateError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            FixedUpdateError::NotEnoughTime { .. } => {
                "At least one period worth of time must be accumulated."
            }
        })
    }
}

impl Error for FixedUpdateError {}
