use image::GenericImageView;

fn get_str_ascii(intensity: u8) -> &'static str {
    let index = intensity / 32;
    let ascii = [" ", ".", ",", "-", "~", "+", "=", "@"];
    return ascii[index as usize];
}

fn get_image(dir: &str, scale: u32) {
    let img = image::open(dir).unwrap();
    let (width, height) = img.dimensions();
    for y in 0..height {
        for x in 0..width {
            if y % (scale * 2) == 0 && x % scale == 0 {
                let pixel = img.get_pixel(x, y);
                let mut intensity = pixel[0] / 3 + pixel[1] / 3 + pixel[2] / 3;
                if pixel[3] == 0 {
                    intensity = 0;
                }
                print!("{}", get_str_ascii(intensity));
            }
        }
        if y % (scale * 2) == 0 {
            print!("");
        }
    }
}

fn main() {
    get_image("example.jpg", 2);
}
