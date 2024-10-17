use solution::*;

use std::panic::catch_unwind;

#[test]
fn test_rgb_display() {
    assert_eq!(Color::new_rgb(0, 0, 0).to_string(), String::from("#000000"));
    assert_eq!(Color::new_rgb(1, 20, 123).to_string(), String::from("#01147b"));
    assert_eq!(Color::new_rgb(255, 255, 255).to_string(), String::from("#ffffff"));
}

#[test]
fn test_hsv_display() {
    assert_eq!(Color::new_hsv(0, 0, 0).to_string(), String::from("hsv(0,0%,0%)"));
    assert_eq!(Color::new_hsv(90, 3, 99).to_string(), String::from("hsv(90,3%,99%)"));
    assert_eq!(Color::new_hsv(360, 100, 100).to_string(), String::from("hsv(360,100%,100%)"));
}

#[test]
fn test_new_hsv() {
    assert!(catch_unwind(|| Color::new_hsv(361, 0, 0)).is_err());
    assert!(catch_unwind(|| Color::new_hsv(100, 200, 0)).is_err());
    assert!(catch_unwind(|| Color::new_hsv(200, 100, 255)).is_err());
}

#[test]
fn test_invert_rgb() {
    let black = Color::new_rgb(0, 0, 0);
    let white = Color::new_rgb(255, 255, 255);

    assert_eq!(black.invert().unwrap_rgb(), white.unwrap_rgb());
    assert_eq!(white.invert().unwrap_rgb(), black.unwrap_rgb());

    let color1 = Color::new_rgb(120, 90, 135);
    let color2 = Color::new_rgb(135, 165, 120);
    assert_eq!(color1.invert().unwrap_rgb(), color2.unwrap_rgb());
    assert_eq!(color2.invert().unwrap_rgb(), color1.unwrap_rgb());

    assert_eq!(color1.invert().invert().unwrap_rgb(), color1.unwrap_rgb());
    assert_eq!(color2.invert().invert().unwrap_rgb(), color2.unwrap_rgb());
}

#[test]
fn test_invert_hsv() {
    let zero = Color::new_hsv(0, 0, 0);
    let full = Color::new_hsv(360, 100, 100);

    assert_eq!(zero.invert().unwrap_hsv(), full.unwrap_hsv());
    assert_eq!(full.invert().unwrap_hsv(), zero.unwrap_hsv());

    let color1 = Color::new_hsv(120, 90, 35);
    let color2 = Color::new_hsv(240, 10, 65);
    assert_eq!(color1.invert().unwrap_hsv(), color2.unwrap_hsv());
    assert_eq!(color2.invert().unwrap_hsv(), color1.unwrap_hsv());

    assert_eq!(color1.invert().invert().unwrap_hsv(), color1.unwrap_hsv());
    assert_eq!(color2.invert().invert().unwrap_hsv(), color2.unwrap_hsv());
}
