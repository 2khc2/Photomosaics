use color_eyre::Result;
use image::{
     io::Reader as ImageReader, DynamicImage,  GenericImage,
    GenericImageView,  RgbImage,
};

const TEST_PATH: &str = "img/test.png";

struct PixelBlock {
    r: u8,
    g: u8,
    b: u8,
    width: u32,
    height: u32,
    x: u32,
    y: u32,
}

struct DynamicImageCrop {
    img: DynamicImage,
    x: u32,
    y: u32,
}

fn avg_color(img: &mut DynamicImage, size: u32) -> Vec<PixelBlock> {
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
        // println!("{} {} {}", r / num_pixels, g / num_pixels, b / num_pixels);
        avg_pixels.push(PixelBlock {
            r: (r / num_pixels) as u8,
            g: (g / num_pixels) as u8,
            b: (b / num_pixels) as u8,
            width: size,
            height: size,
            x: s.x,
            y: s.y,
        })
    }
    avg_pixels
}

fn pixellate(blocks: Vec<PixelBlock>, w:u32, h:u32) -> Result<()> {
    // let block_h = blocks[0].height;
    // let block_w = blocks[0].width;
    let mut rbg_img = RgbImage::new(w, h);

    for block in blocks {
        // rbg_img.copy_from(other, x, y)
        let mut rgb_block = RgbImage::new(block.width,block.height);
        for pixel in rgb_block.pixels_mut() {
            pixel[0] = block.r;
            pixel[1] = block.g;
            pixel[2] = block.b;
        }
        rbg_img.copy_from(&rgb_block, block.x, block.y)?;
    }
    Ok(rbg_img.save(TEST_PATH)?)
}

fn main() -> Result<()> {
    let size = 200;

    let mut img = ImageReader::open("img/fox.jpg")?.decode()?;
    let blocks = avg_color(&mut img, size);
    pixellate(blocks,img.width(), img.height())?;


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
