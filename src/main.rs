extern crate image;

use image::{ImageBuffer, Luma};

mod canvas;

use canvas::{Canvas, ImageCanvas};

struct Line {
    x0: f64,
    y0: f64,
    x1: f64,
    y1: f64
}

enum Shape {
    Line(Line)
}

fn main() {
    let mut img = ImageCanvas::blank(512, 512);
    let mut scene: Vec<Shape> = Vec::new();
    let line1 = Line {
        x0: 0.1,
        y0: 0.2,
        x1: 0.9,
        y1: 0.7
    };
    let line2 = Line {
        x0: 0.5,
        y0: 0.1,
        x1: 0.5,
        y1: 0.9
    };
    scene.push(Shape::Line(line1));
    scene.push(Shape::Line(line2));
    render(scene, &mut img);
    img.save("line.png").unwrap();
}

fn render(scene: Vec<Shape>, img: &mut impl Canvas) {
    for shape in scene.iter() {
        match shape {
            Shape::Line(line) => draw_line(line, img)
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
        img.draw(ypxl1 as f64 / dimensions.0 as f64, xpxl1 as f64 / dimensions.1 as f64, Luma([scale_to_byte(rfpart(yend) * xgap)]));
        img.put_pixel(ypxl1 + 1, xpxl1, Luma([scale_to_byte(yend.fract() * xgap)]));
    } else {
        img.put_pixel(xpxl1, ypxl1, Luma([scale_to_byte(rfpart(yend) * xgap)]));
        img.put_pixel(xpxl1, ypxl1 + 1, Luma([scale_to_byte(yend.fract() * xgap)]));
    }

    let mut intery = yend + gradient;

    xend = x1.floor();
    yend = y1 + gradient * (xend - x1);
    xgap = (x1 + 0.5).fract();
    let xpxl2 = xend as u32;
    let ypxl2 = yend.floor() as u32;
    if steep {
        img.put_pixel(ypxl2, xpxl2, Luma([scale_to_byte(rfpart(yend) * xgap)]));
        img.put_pixel(ypxl2 + 1, xpxl2, Luma([scale_to_byte(yend.fract() * xgap)]));
    } else {
        img.put_pixel(xpxl2, ypxl2, Luma([scale_to_byte(rfpart(yend) * xgap)]));
        img.put_pixel(xpxl2, ypxl2 + 1, Luma([scale_to_byte(yend.fract() * xgap)]));
    }

    if steep {
        for x in (xpxl1+1)..(xpxl2-1) {
            img.put_pixel(intery.floor() as u32, x, Luma([scale_to_byte(rfpart(intery))]));
            img.put_pixel(intery.floor() as u32 + 1, x, Luma([scale_to_byte(intery.fract())]));
            intery += gradient;
        }
    } else {
        for x in (xpxl1+1)..(xpxl2-1) {
            img.put_pixel(x, intery.floor() as u32, Luma([scale_to_byte(rfpart(intery))]));
            img.put_pixel(x, intery.floor() as u32 + 1, Luma([scale_to_byte(intery.fract())]));
            intery += gradient;
        }
    }
}