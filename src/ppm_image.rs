use crate::colour::Colour;

pub fn write_ppm_image<I>(width: usize, height: usize, max_colour_value: u8, image: I) -> String
where
    I: Iterator<Item = Colour>,
{
    let mut ppm_image = String::with_capacity(16 + width * height * 12);

    let ppm_image_header = format!("P3 {} {} {}\n", width, height, max_colour_value);
    ppm_image.push_str(&ppm_image_header);

    for pixel in image {
        let r = pixel.r * max_colour_value as f32;
        let g = pixel.g * max_colour_value as f32;
        let b = pixel.b * max_colour_value as f32;
        let pixel_as_string = format!("{} {} {}\n", r as i8, g as i8, b as i8);
        ppm_image.push_str(&pixel_as_string);
    }

    ppm_image
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::colour::Colour;

    #[test]
    pub fn write_ppm_image_test() {
        let image = vec![
            Colour {
                r: 0.1,
                g: 0.2,
                b: 0.3,
                a: 0.5,
            },
            Colour {
                r: 0.1,
                g: 0.2,
                b: 0.3,
                a: 0.5,
            },
            Colour {
                r: 0.1,
                g: 0.2,
                b: 0.3,
                a: 0.5,
            },
            Colour {
                r: 0.1,
                g: 0.2,
                b: 0.3,
                a: 0.5,
            },
        ];
        let ppm_image = write_ppm_image(2, 2, 255, image.into_iter());

        println!("{}", ppm_image);
    }
}
