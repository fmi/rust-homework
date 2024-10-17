use solution::*;

#[test]
fn test_basic() {
    let color1 = Color::new_rgb(0, 0, 0);
    assert_eq!(color1.unwrap_rgb().0, 0);
    assert_eq!(&color1.to_string()[0..1], "#");

    let color2 = Color::new_hsv(0, 0, 0);
    assert_eq!(color2.unwrap_hsv().0, 0);

    assert_eq!(color1.invert().unwrap_rgb().0, 255);
}
