use image::{ImageBuffer, Luma};
use std::io;
use std::path::Path;

pub struct Line {
    pub x0: f64,
    pub y0: f64,
    pub x1: f64,
    pub y1: f64
}

pub enum Shape {
    Line(Line)
}

pub trait Canvas {
    fn draw(&mut self, x: f64, y: f64, color: u8);
    fn draw_line(&mut self, line: &Line);
    fn save<Q>(&self, path: Q) -> io::Result<()> 
            where Q: AsRef<Path>;
    fn render(&mut self, scene: Vec<Shape>);
}

pub struct ImageCanvas {
    img: ImageBuffer<Luma<u8>, Vec<u8>>
}

impl ImageCanvas {
    pub fn blank(x: u32, y: u32) -> ImageCanvas {
        ImageCanvas {
            img: <ImageBuffer<Luma<u8>, Vec<u8>>>::new(x, y)
        }
    }
}

impl Canvas for ImageCanvas {
    fn draw(&mut self, x: f64, y: f64, color: u8) {
        let xcoord = (x * self.img.dimensions().0 as f64).round() as u32;
        let ycoord = (y * self.img.dimensions().1 as f64).round() as u32;
        self.img.put_pixel(xcoord, ycoord, Luma([color]));
    }

    fn save<Q>(&self, path: Q) -> io::Result<()> 
            where Q: AsRef<Path> {
        self.img.save(path)
    }

    fn draw_line(&mut self, line: &Line) {
        let dims = self.img.dimensions();
        draw_line(line, self, dims);
    }
    

    fn render(&mut self, scene: Vec<Shape>) {
        for shape in scene.iter() {
            match shape {
                Shape::Line(line) => self.draw_line(line)
            }
        }
    }
}

fn rfpart(x: f64) -> f64 {
    1.0 - x.fract()
}

fn scale_to_byte(x: f64) -> u8 {
    (x * 255.0).round() as u8
}

fn draw_line(line: &Line, img: &mut impl Canvas, dimensions: (u32, u32)) {
    let mut x0 = line.x0 * dimensions.0 as f64;
    let mut y0 = line.y0 * dimensions.1 as f64;
    let mut x1 = line.x1 * dimensions.0 as f64;
    let mut y1 = line.y1 * dimensions.1 as f64;

    let steep = (y1 - y0).abs() > (x1 - x0).abs();

    if steep {
        std::mem::swap(&mut x0, &mut y0);
        std::mem::swap(&mut x1, &mut y1);
    }
    if x0 > x1 {
        std::mem::swap(&mut x0, &mut x1);
        std::mem::swap(&mut y0, &mut y1);
    }
    
    let dx = x1 - x0;
    let dy = y1 - y0;
    let mut gradient = dy / dx;
    if dx == 0.0 {
        gradient = 1.0;
    }

    let mut xend = x0.floor();
    let mut yend = y0 + gradient * (xend - x0);
    let mut xgap = rfpart(x0 + 0.5);
    let xpxl1 = xend as u32;
    let ypxl1 = yend.floor() as u32;

    if steep {
        img.draw(ypxl1 as f64 / dimensions.0 as f64, xpxl1 as f64 / dimensions.1 as f64, scale_to_byte(rfpart(yend) * xgap));
        img.draw((ypxl1 + 1) as f64 / dimensions.0 as f64, xpxl1 as f64 / dimensions.1 as f64, scale_to_byte(yend.fract() * xgap));
    } else {
        img.draw(xpxl1 as f64 / dimensions.0 as f64, ypxl1 as f64 / dimensions.1 as f64, scale_to_byte(rfpart(yend) * xgap));
        img.draw(xpxl1 as f64 / dimensions.0 as f64, (ypxl1 + 1) as f64 / dimensions.1 as f64, scale_to_byte(yend.fract() * xgap));
    }

    let mut intery = yend + gradient;

    xend = x1.floor();
    yend = y1 + gradient * (xend - x1);
    xgap = (x1 + 0.5).fract();
    let xpxl2 = xend as u32;
    let ypxl2 = yend.floor() as u32;
    if steep {
        img.draw(ypxl2 as f64 / dimensions.0 as f64, xpxl2 as f64 / dimensions.1 as f64, scale_to_byte(rfpart(yend) * xgap));
        img.draw((ypxl2 + 1) as f64 / dimensions.0 as f64, xpxl2 as f64 / dimensions.1 as f64, scale_to_byte(yend.fract() * xgap));
    } else {
        img.draw(xpxl2 as f64 / dimensions.0 as f64, ypxl2 as f64 / dimensions.1 as f64, scale_to_byte(rfpart(yend) * xgap));
        img.draw(xpxl2 as f64 / dimensions.0 as f64, (ypxl2 + 1) as f64 / dimensions.1 as f64, scale_to_byte(yend.fract() * xgap));
    }

    if steep {
        for x in (xpxl1+1)..(xpxl2-1) {
            img.draw(intery.floor() / dimensions.0 as f64, 
                    x as f64 / dimensions.1 as f64, 
                    scale_to_byte(rfpart(intery)));
            img.draw((intery.floor() + 1.0) / dimensions.0 as f64, 
                    x as f64 / dimensions.1 as f64, 
                    scale_to_byte(intery.fract()));
            intery += gradient;
        }
    } else {
        for x in (xpxl1+1)..(xpxl2-1) {
            img.draw(x as f64 / dimensions.0 as f64, 
                    intery.floor() / dimensions.1 as f64, 
                    scale_to_byte(rfpart(intery)));
            img.draw(x as f64 / dimensions.0 as f64, 
                    (intery.floor() + 1.0) / dimensions.1 as f64, 
                    scale_to_byte(intery.fract()));
            intery += gradient;
        }
    }
}
