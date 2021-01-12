use drawille::Canvas;
use image::{
    imageops::{resize, FilterType::Lanczos3},
    open, DynamicImage,
};
use term_size::dimensions;

fn main() {
    let im = open("/home/divy/Downloads/logo.jpg").unwrap().to_rgba();
    let gray = DynamicImage::ImageRgba8(im).into_luma8();

    let (_tw, _th) = dimensions().unwrap();
    let tw = (_tw * 1) as u32;
    let th = (_th * 3) as u32;

    let mut w = gray.width();
    let mut h = gray.height();

    let mut img = gray.clone();
    if tw < w {
        let ratio = tw / w;
        w = tw;
        h = h * ratio;
        img = resize(&gray, tw, th, Lanczos3);
    }

    let samples = img.as_raw().to_vec();

    let mut x = 0;
    let mut y = 0;
    let mut canvas = Canvas::new(tw, th);
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
