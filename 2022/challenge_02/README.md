# Color parsing

На предното домашно печатахме цветове, сега ще вървим в обратната посока -- парсене. Имате низ, искате да извадите цвят. Тъй като това е предизвикателство, няма да ви дадем *цялата* информация, която ви трябва за да се справите, но може да намерите помощни средства в стандартната документация, примерно:

- <https://doc.rust-lang.org/stable/std/primitive.u8.html>
- <https://doc.rust-lang.org/stable/std/string/struct.String.html>
- <https://doc.rust-lang.org/stable/std/str/trait.FromStr.html>

```rust
pub enum Color {
    RGB {
        red: u8,
        green: u8,
        blue: u8
    },
    HSV {
        hue: u16,
        saturation: u8,
        value: u8,
    }
}

impl Color {
    pub fn new_rgb(red: u8, green: u8, blue: u8) -> Color {
        Self::RGB { red, green, blue }
    }

    pub fn new_hsv(hue: u16, saturation: u8, value: u8) -> Color {
        if hue > 360 || saturation > 100 || value > 100 {
            panic!("Invalid input");
        }

        Self::HSV { hue, saturation, value }
    }

    pub fn from_str(input: &str) -> Self {
        todo!()
    }
}
```

Очакваме `Color::from_str("#ff00ff")` да се изпарси до `Color::RGB { red: 255, green: 0, blue: 255 }` примерно. Очакваме `Color::from_str("hsv(360,100%,50%)")` да извади `Color::HSV { hue: 360, saturation: 100, value: 50 }`. Може да приемете, че винаги ще ви даваме валиден вход (т.е. няма нужда да мислите за error handling за момента, `panic!`-вайте ако се налага), няма да има интервали между запетайките, винаги шестнадесетичната репрезентация на RGB ще бъде с малки букви.
