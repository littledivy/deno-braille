use drawille::Canvas;
use image::{
    imageops::{resize, FilterType::Lanczos3},
    open, DynamicImage,
};
use terminal_size::{terminal_size, Height, Width};

fn main() {
    let im = open("logo.jpg").unwrap().to_rgba8();
    let gray = DynamicImage::ImageRgba8(im).into_luma8();

    let (Width(_tw), Height(_th)) = terminal_size().unwrap();
    let tw = _tw as u32;
    let _th = _th as u32;

    let mut w = gray.width();
    let mut h = gray.height();

    let mut img = gray.clone();
    if tw < w {
        let ratio: f64 = tw as f64 / w as f64;
        h = (h as f64 * ratio) as u32;
        w = tw;
        img = resize(&gray, w, h, Lanczos3);
    }

    let samples = img.as_raw().to_vec();

    let mut x = 0;
    let mut y = 0;
    let mut canvas = Canvas::new(w, h);
    for pix in samples {
        if pix > 128 {
            canvas.set(x, y);
        }
        x += 1;
        if x >= w {
            y += 1;
            x = 0;
        }
    }

    println!("{}", canvas.frame());
}
