use std::fs::File;
use std::io::Write;

#[derive(Clone, Copy, Debug)]
enum Color {
    WHITE = 0x0,
    BLACK = 0xFFFFFF,
    MAGENTA = 0xFF00FF,
}

#[allow(dead_code)]
fn fill_solid_circle(pixels: &mut [u32], width: usize, height: usize, radius: usize, foreground: Color, background: Color) {
    let cx = width as i32;
    let cy = height as i32;
    let r = radius as i32;

    for y in 0..height {
        for x in 0..width {
            let dx = cx - x as i32 * 2 - 1;
            let dy = cy - y as i32 * 2 - 1;

            pixels[y * width + x] = if dx*dx + dy*dy <= r*r {
                foreground
            } else {
                background
            } as u32;
        }
    }
}

#[allow(dead_code)]
fn fill_checker_pattern(pixels: &mut [u32], width: usize, height: usize, tile_size: usize, foreground: Color, background: Color) {
    for y in 0..height {
        for x in 0..width {
            pixels[y * width + x] = if (x / tile_size + y / tile_size) % 2 == 0 {
                foreground
            } else {
                background
            } as u32;
        }
    }
}

// TODO: add magnifier effect to circle
#[allow(dead_code)]
fn fill_circle_with_checker(pixels: &mut [u32], width: usize, height: usize, radius: usize, tile_size: usize, foreground: Color, background: Color) {
    let cx = width as i32;
    let cy = height as i32;
    let r = radius as i32 * 2;

    for y in 0..height {
        for x in 0..width {
            let dx = cx - x as i32 * 2 - 1;
            let dy = cy - y as i32 * 2 - 1;

            pixels[y * width + x] = if dx*dx + dy*dy <= r*r {
                 if (x / tile_size + y / tile_size) % 2 == 0 {
                    foreground
                 } else {
                    background
                 }
            } else {
                 if (x / tile_size + y / tile_size) % 2 == 0 {
                    background
                 } else {
                    foreground
                 }
            } as u32;
        }
    }
}

fn fill(pixels: &mut [u32], c: Color) {
    pixels.fill(c as u32);
}

fn write_ppm(output_path: &str, pixels: &[u32], width: usize, height: usize) {
    let mut buffer = Vec::<u8>::new();

    for y in 0..height {
        for x in 0..width {
            let pixel = pixels[y * width + x];
            buffer.push(((pixel >> 8 * 2) & 0xFF) as u8);
            buffer.push(((pixel >> 8 * 1) & 0xFF) as u8);
            buffer.push(((pixel >> 8 * 0) & 0xFF) as u8);
        }
    }
    
    let mut file = File::create(output_path).unwrap();
    write!(file, "P6\n{width} {height} 255\n").unwrap();
    file.write(&buffer).unwrap();

    println!("Saved to {}", output_path);
}

fn main() {
    const WIDTH: usize = 1024;
    const HEIGHT: usize = 1024;

    let mut pixels = [0u32; WIDTH * HEIGHT];
    fill(&mut pixels, Color::MAGENTA);
    fill_circle_with_checker(&mut pixels, WIDTH, HEIGHT, WIDTH / 3, 128, Color::BLACK, Color::WHITE);

    write_ppm("output.ppm", &pixels, WIDTH, HEIGHT);
}
