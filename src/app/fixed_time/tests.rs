use super::*;

#[test]
fn fixed_time_starts_at_zero() {
    let new_time = FixedTime::new(Duration::from_secs(42));
    assert_eq!(new_time.accumulated, Duration::ZERO);

    let default_time = FixedTime::default();
    assert_eq!(default_time.accumulated, Duration::ZERO);
}

#[test]
fn fixed_time_ticks_up() {
    let mut fixed_time = FixedTime::default();
    fixed_time.tick(Duration::from_secs(1));
    assert_eq!(fixed_time.accumulated, Duration::from_secs(1));
}

#[test]
fn enough_accumulated_time_is_required() {
    let mut fixed_time = FixedTime::new(Duration::from_secs(2));
    fixed_time.tick(Duration::from_secs(1));
    assert!(fixed_time.expend(NonZeroU8::MIN).is_err());
    assert_eq!(fixed_time.accumulated, Duration::from_secs(1));

    fixed_time.tick(Duration::from_secs(1));
    assert!(fixed_time.expend(NonZeroU8::MIN).is_ok());
    assert_eq!(fixed_time.accumulated, Duration::ZERO);
}

#[test]
fn repeatedly_expending_time() {
    let mut fixed_time = FixedTime::new(Duration::from_secs(1));
    fixed_time.tick(Duration::from_secs_f32(3.2));
    assert!(fixed_time.expend(NonZeroU8::MIN).is_ok());
    assert!(fixed_time.expend(NonZeroU8::MIN).is_ok());
    assert!(fixed_time.expend(NonZeroU8::MIN).is_ok());
    assert!(fixed_time.expend(NonZeroU8::MIN).is_err());
}
