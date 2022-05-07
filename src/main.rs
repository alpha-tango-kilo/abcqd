use image::{DynamicImage, ImageBuffer, imageops, Pixel};
use std::error::Error;
use std::ops::Deref;
use std::process;

fn main() {
    if let Err(why) = _main() {
        eprintln!("{why}");
        process::exit(1);
    }
}

fn _main() -> Result<(), Box<dyn Error>> {
    Ok(())
}

trait AutoCrop {
    fn auto_crop(&mut self);
}

impl AutoCrop for DynamicImage {
    fn auto_crop(&mut self) {
        use DynamicImage::*;
        match self {
            ImageLuma8(buf) => buf.auto_crop(),
            ImageLumaA8(buf) => buf.auto_crop(),
            ImageRgb8(buf) => buf.auto_crop(),
            ImageRgba8(buf) => buf.auto_crop(),
            ImageLuma16(buf) => buf.auto_crop(),
            ImageLumaA16(buf) => buf.auto_crop(),
            ImageRgb16(buf) => buf.auto_crop(),
            ImageRgba16(buf) => buf.auto_crop(),
            _ => panic!("unsupported DynamicImage variant"),
        }
    }
}

impl<P, C> AutoCrop for ImageBuffer<P, C>
where
    P: Pixel + Eq,
    C: Deref<Target = [<P as Pixel>::Subpixel]>,
{
    fn auto_crop(&mut self) {
        let top = self.rows().next().expect("empty image");
        if all_same(top) {
            imageops::crop(self, 0, 1, self.width(), self.height() - 1);
        }
        // panic here implies the image has only a single row of pixels
        let bottom = self.rows().next_back().expect("image too small to crop");
    }
}

fn all_same<T: Eq>(iterable: impl IntoIterator<Item = T>) -> bool {
    let mut iter = iterable.into_iter();
    let first = match iter.next() {
        Some(t) => t,
        None => return true,
    };
    iter.all(|t| t == first)
}
