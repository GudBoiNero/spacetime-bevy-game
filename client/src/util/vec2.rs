use bevy::a11y::accesskit::Vec2;

pub fn normalized(vec: Vec2) -> Vec2 {
    let magnitude = f64::sqrt(vec.x.powi(2) + vec.y.powi(2));
    let x = vec.x / magnitude;
    let y = vec.y / magnitude;
    Vec2 { x: if x.is_nan() {0.0} else {x}, y: if y.is_nan() {0.0} else {y} }
}