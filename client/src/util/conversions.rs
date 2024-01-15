pub fn f64_to_f32(x: f64) -> f32 {
    let y = x as f32;
    assert_eq!(
        x.is_finite(),
        y.is_finite(),
        "f32 overflow during conversion"
    );
    y
}