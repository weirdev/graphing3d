use image::{ImageBuffer, Luma};
use std::io;
use std::path::Path;

pub struct Line {
    pub x0: f64,
    pub y0: f64,
    pub x1: f64,
    pub y1: f64
}

pub struct Ellipse {
    pub x0: f64,
    pub y0: f64,
    pub x1: f64,
    pub y1: f64
}

pub enum Shape {
    Line(Line),
    Ellipse(Ellipse)
}

pub trait Canvas {
    fn draw(&mut self, x: f64, y: f64, color: u8);
    fn draw_line(&mut self, line: &Line);
    fn draw_ellipse(&mut self, ellipse: &Ellipse);
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
        // image y axis 0 at top, but for Canvas we use bottom as 0
        let ycoord = ((1.0 - y) * self.img.dimensions().1 as f64).round() as u32;
        self.img.put_pixel(xcoord, ycoord, Luma([color]));
    }

    fn save<Q>(&self, path: Q) -> io::Result<()> 
            where Q: AsRef<Path> {
        self.img.save(path)
    }

    fn draw_line(&mut self, line: &Line) {
        let dims = self.img.dimensions();
        draw_line_antialiased(line, self, dims);
    }
    
    fn draw_ellipse(&mut self, ellipse: &Ellipse) {
        let dims = self.img.dimensions();
        draw_ellipse(ellipse, self, dims);
    }

    fn render(&mut self, scene: Vec<Shape>) {
        for shape in scene.iter() {
            match shape {
                Shape::Line(line) => self.draw_line(line),
                Shape::Ellipse(ellipse) => self.draw_ellipse(ellipse)
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

fn draw_line_antialiased(line: &Line, img: &mut impl Canvas, dimensions: (u32, u32)) {
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

//http://members.chello.at/~easyfilter/bresenham.html
fn draw_ellipse(ellipse: &Ellipse, img: &mut impl Canvas, dimensions: (u32, u32)) {
    let mut x0 = (ellipse.x0 * dimensions.0 as f64).round() as u32;
    let mut y0 = (ellipse.y0 * dimensions.1 as f64).round() as u32;
    let mut x1 = (ellipse.x1 * dimensions.0 as f64).round() as u32;
    let mut y1 = (ellipse.y1 * dimensions.1 as f64).round() as u32;

    let mut a = (x1 as i32 - x0 as i32).abs() as u32;
    let b = (y1 as i32 - y0 as i32).abs() as u32;
    let mut b1 = b & 1;

    let mut dx = 4*(1 - a as i32) * b as i32 * b as i32;
    let mut dy = 4*(b1 as i32 + 1) * a as i32 * a as i32;
    let mut err = dx + dy + (b1 as i32 * a as i32 * a as i32);

    if x0 > x1 {
        x0 = x1;
        x1 += a;
    }
    if y0 > y1 {
        y0 = y1;
    }
    y0 += (b+1) / 2;
    y1 = y0 - b1;

    a *= 8 * a;
    b1 = 8 * b * b;

    println!("{} {} {} {}", x0, x1, y0, y1);

    loop {
        img.draw(x1 as f64 / dimensions.0 as f64, y0 as f64 / dimensions.1 as f64, 255);
        img.draw(x0 as f64 / dimensions.0 as f64, y0 as f64 / dimensions.1 as f64, 255);
        img.draw(x0 as f64 / dimensions.0 as f64, y1 as f64 / dimensions.1 as f64, 255);
        img.draw(x1 as f64 / dimensions.0 as f64, y1 as f64 / dimensions.1 as f64, 255);
        let e2 = 2 * err;
        if e2 <= dy {
            y0 += 1;
            y1 -= 1;
            dy += a as i32;
            err += dy;
        }
        if e2 >= dx || 2*err > dy {
            x0 += 1;
            x1 -= 1;
            dx += b1 as i32;
            err += dx;
        }

        if x0 > x1 {
            break;
        }
    }

    /*
    while (y0 as i32 - y1 as i32) < b as i32 {
        img.draw((x0-1) as f64 / dimensions.0 as f64, y0 as f64 / dimensions.1 as f64, 255);
        img.draw((x1+1) as f64 / dimensions.0 as f64, y0 as f64 / dimensions.1 as f64, 255);
        y0 += 1;
        img.draw((x0-1) as f64 / dimensions.0 as f64, y1 as f64 / dimensions.1 as f64, 255);
        img.draw((x1+1) as f64 / dimensions.0 as f64, y1 as f64 / dimensions.1 as f64, 255);
        y1 -= 1;
    }*/
}