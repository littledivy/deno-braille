use drawille::Canvas;
use image::DynamicImage;
use image::imageops::resize;

fn main() {
  let im = image::open("/home/divy/Downloads/logo.jpg").unwrap().to_rgba();
  let gray = DynamicImage::ImageRgba8(im).into_luma8();
 
  let (_tw, _th) = term_size::dimensions().unwrap();
  let tw = (_tw * 1) as u32;
  let th = (_th * 3) as u32;
  
  let mut w = gray.width();
  let mut h = gray.height();
  
  let mut img = gray.clone();
  if tw < w {
        let ratio = tw / w;
        w = tw;
        h = h * ratio;
        img = resize(&gray, tw, th, image::imageops::FilterType::Lanczos3);
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
