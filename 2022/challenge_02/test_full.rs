use solution::*;

#[test]
fn test_parsing_hsv() {
    let parsed = Color::from_str("hsv(300,50%,70%)");

    if let Color::HSV { hue, saturation, value } = parsed {
        assert_eq!(hue, 300);
        assert_eq!(saturation, 50);
        assert_eq!(value, 70);
    } else {
        panic!("Not the expected result");
    }
}

#[test]
fn test_parsing_hex_rgb() {
    let parsed = Color::from_str("#abcdef");

    if let Color::RGB { red, green, blue } = parsed {
        assert_eq!(red, 0xAB);
        assert_eq!(green, 0xCD);
        assert_eq!(blue, 0xEF);
    } else {
        panic!("Not the expected result");
    }
}
