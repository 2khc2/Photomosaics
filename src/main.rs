use color_eyre::Result;
use image::{io::Reader as ImageReader, DynamicImage, GenericImage, GenericImageView, RgbImage};
use std::{cmp::min, fs};

const TEST_PATH: &str = "img/test.png";

struct PixelBlock {
    r: u8,
    g: u8,
    b: u8,
    width: u32,
    height: u32,
    x: Option<u32>,
    y: Option<u32>,
}

struct DynamicImageCrop {
    img: DynamicImage,
    x: u32,
    y: u32,
}

fn divide_to_squares(img: &mut DynamicImage, size: u32) -> Vec<DynamicImageCrop> {
    let mut squares: Vec<DynamicImageCrop> = vec![];
    let steps = size as usize;
    for i in (size..img.width()).step_by(steps) {
        for j in (size..img.height()).step_by(steps) {
            squares.push(DynamicImageCrop {
                img: img.crop(i - size, j - size, size, size),
                x: i - size,
                y: j - size,
            });
        }
    }
    squares
}

fn avg_color(img: &mut DynamicImage, size: u32) -> PixelBlock {
    let _avg_pixels: Vec<PixelBlock> = vec![];
    let num_pixels = size * size;
    let mut r = 0;
    let mut g = 0;
    let mut b = 0;
    for p in img.pixels() {
        r += p.2[0] as u32;
        g += p.2[1] as u32;
        b += p.2[2] as u32;
    }
    PixelBlock {
        r: (r / num_pixels) as u8,
        g: (g / num_pixels) as u8,
        b: (b / num_pixels) as u8,
        width: size,
        height: size,
        x: None,
        y: None,
    }
}

fn div_avg_color(img: &mut DynamicImage, size: u32) -> Vec<PixelBlock> {
    let squares = divide_to_squares(img, size);

    let mut avg_pixels: Vec<PixelBlock> = vec![];
    let num_pixels = size * size;
    for s in squares {
        let mut r = 0;
        let mut g = 0;
        let mut b = 0;
        for p in s.img.pixels() {
            r += p.2[0] as u32;
            g += p.2[1] as u32;
            b += p.2[2] as u32;
        }
        avg_pixels.push(PixelBlock {
            r: (r / num_pixels) as u8,
            g: (g / num_pixels) as u8,
            b: (b / num_pixels) as u8,
            width: size,
            height: size,
            x: Some(s.x),
            y: Some(s.y),
        })
    }
    avg_pixels
}

fn pixellate(blocks: Vec<PixelBlock>, w: u32, h: u32) -> Result<()> {
    let mut rbg_img = RgbImage::new(w, h);

    for block in blocks {
        let mut rgb_block = RgbImage::new(block.width, block.height);
        for pixel in rgb_block.pixels_mut() {
            pixel[0] = block.r;
            pixel[1] = block.g;
            pixel[2] = block.b;
        }
        rbg_img.copy_from(&rgb_block, block.x.unwrap(), block.y.unwrap())?;
    }
    Ok(rbg_img.save(TEST_PATH)?)
}

fn square_crop(mut img: DynamicImage) -> DynamicImage {
    let size = min(img.width(), img.height());
    img.crop(0, 0, size, size)
}

fn main() -> Result<()> {
    let size = 80;

    let mut img = ImageReader::open("img/fox.jpg")?.decode()?;
    let blocks = div_avg_color(&mut img, size);
    // pixellate(blocks,img.width(), img.height())?;

    let source_fp = fs::read_dir("./img/source/").unwrap();

    let mut src_imgs: Vec<RgbImage> = vec![];
    let mut avg_src_color: Vec<PixelBlock> = vec![];
    for path in source_fp {

        // println!("Name: {}", path.unwrap().path().display());

        let img = ImageReader::open(path.unwrap().path())?.decode()?;
        let mut cropped = square_crop(img);
        let cropped_rgb = cropped.to_rgb8();
        src_imgs.push(cropped_rgb.clone());
        avg_src_color.push(avg_color(&mut cropped, cropped_rgb.height()));
    }

    let mut photomosaic = RgbImage::new(img.width(), img.height());

    for orig_block in blocks {
        let mut diff = u32::MAX;
        let mut idx = 0;
        for (i, src_pb) in avg_src_color.iter().enumerate() {
            let distance = f32::sqrt(
                (
                    (src_pb.r as f32 - orig_block.r  as f32).powi(2 )
                    + (src_pb.g as f32 - orig_block.g as f32).powi( 2)
                     + (src_pb.b as f32 - orig_block.b as f32).powi(2)
                    ).sqrt()
            ) as u32;
            if distance < diff {
                diff = distance;
                idx = i;
            }
        }
        photomosaic.copy_from(&src_imgs[idx], orig_block.x.unwrap(), orig_block.y.unwrap())?;
    }
    photomosaic.save(TEST_PATH)?;

    // img.crop(x, y, width, height)

    // image::imageops::rotate180(&img).save("img/test.png")?;

    // image::imageops::resize(&img, img.width() / 2, img.height()/2, imageops::Triangle).save(TEST_PATH)?;

    // imageops::grayscale(&img).save(TEST_PATH)?;
    // // let a = img.as_bytes().to_vec();

    // let  mut rgbimg = RgbImage::new(img.width() * 2, img.height());
    // rgbimg.copy_from(&img.to_rgb8(), 0, 0)?;
    // rgbimg.copy_from(&imageops::rotate180(&img.to_rgb8()), img.width(), 0)?;
    // rgbimg.save(TEST_PATH)?;
    Ok(())
}
