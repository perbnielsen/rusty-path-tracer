use crate::colour::Colour;

pub fn write_ppm_image<I>(width: usize, height: usize, image: I) -> String
where
    I: IntoIterator<Item = Colour>,
{
    const MAX_COLOUR_VALUE: f32 = 255.0;
    let mut ppm_image = String::with_capacity(16 + width * height * 12);

    let ppm_image_header = format!("P3 {} {} {}\n", width, height, MAX_COLOUR_VALUE);
    ppm_image.push_str(&ppm_image_header);

    for pixel in image {
        let r = pixel.r * MAX_COLOUR_VALUE;
        let g = pixel.g * MAX_COLOUR_VALUE;
        let b = pixel.b * MAX_COLOUR_VALUE;
        let pixel_as_string = format!("{} {} {}\n", r as u8, g as u8, b as u8);
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
                g: 0.5,
                b: 1.0,
                a: 0.5,
            },
        ];
        let ppm_image = write_ppm_image(2, 2, image);

        println!("{}", ppm_image);
    }
}
