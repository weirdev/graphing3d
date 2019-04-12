mod scene2d;
mod scene3d;

use image::Rgba;

pub use self::scene2d::{Shape2D, Line2D, Ellipse2D};
pub use self::scene3d::Shape3D;

fn rfpart(x: f64) -> f64 {
    1.0 - x.fract()
}

fn scale_to_byte(x: f64) -> u8 {
    (x * 255 as f64).round() as u8
}

// https://en.wikipedia.org/wiki/Xiaolin_Wu%27s_line_algorithm
pub fn draw_line2d_antialiased<F>(line: &Line2D, mut draw: F, dimensions: (u32, u32), color: Rgba<u8>)
        where F: FnMut(f64, f64, Rgba<u8>)
{
    // TODO: along with scale_to_byte, needs to be modified to account for background color with antialiasing
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
        let mut newpx = color.clone();
        newpx.data[3] = scale_to_byte(rfpart(yend) * xgap);
        draw(ypxl1 as f64 / dimensions.0 as f64, xpxl1 as f64 / dimensions.1 as f64, newpx);
        newpx = color.clone();
        newpx.data[3] = scale_to_byte(yend.fract() * xgap);
        draw((ypxl1 + 1) as f64 / dimensions.0 as f64, xpxl1 as f64 / dimensions.1 as f64, newpx);
    } else {
        let mut newpx = color.clone();
        newpx.data[3] = scale_to_byte(rfpart(yend) * xgap);
        draw(xpxl1 as f64 / dimensions.0 as f64, ypxl1 as f64 / dimensions.1 as f64, newpx);
        newpx = color.clone();
        newpx.data[3] = scale_to_byte(yend.fract() * xgap);
        draw(xpxl1 as f64 / dimensions.0 as f64, (ypxl1 + 1) as f64 / dimensions.1 as f64, newpx);
    }

    let mut intery = yend + gradient;

    xend = x1.floor();
    yend = y1 + gradient * (xend - x1);
    xgap = (x1 + 0.5).fract();
    let xpxl2 = xend as u32;
    let ypxl2 = yend.floor() as u32;
    if steep {
        let mut newpx = color.clone();
        newpx.data[3] = scale_to_byte(rfpart(yend) * xgap);
        draw(ypxl2 as f64 / dimensions.0 as f64, xpxl2 as f64 / dimensions.1 as f64, newpx);
        newpx = color.clone();
        newpx.data[3] = scale_to_byte(yend.fract() * xgap);
        draw((ypxl2 + 1) as f64 / dimensions.0 as f64, xpxl2 as f64 / dimensions.1 as f64, newpx);
    } else {
        let mut newpx = color.clone();
        newpx.data[3] = scale_to_byte(rfpart(yend) * xgap);
        draw(xpxl2 as f64 / dimensions.0 as f64, ypxl2 as f64 / dimensions.1 as f64, newpx);
        newpx = color.clone();
        newpx.data[3] = scale_to_byte(yend.fract() * xgap);
        draw(xpxl2 as f64 / dimensions.0 as f64, (ypxl2 + 1) as f64 / dimensions.1 as f64, newpx);
    }

    if steep {
        for x in (xpxl1+1)..(xpxl2-1) {
            let mut newpx = color.clone();
            newpx.data[3] = scale_to_byte(rfpart(intery));
            draw(intery.floor() / dimensions.0 as f64, 
                    x as f64 / dimensions.1 as f64, 
                    newpx);
            newpx = color.clone();
            newpx.data[3] = scale_to_byte(intery.fract());
            draw((intery.floor() + 1.0) / dimensions.0 as f64, 
                    x as f64 / dimensions.1 as f64, 
                    newpx);
            intery += gradient;
        }
    } else {
        for x in (xpxl1+1)..(xpxl2-1) {
            let mut newpx = color.clone();
            newpx.data[3] = scale_to_byte(rfpart(intery));
            draw(x as f64 / dimensions.0 as f64, 
                    intery.floor() / dimensions.1 as f64, 
                    newpx);
            newpx = color.clone();
            newpx.data[3] = scale_to_byte(intery.fract());
            draw(x as f64 / dimensions.0 as f64, 
                    (intery.floor() + 1.0) / dimensions.1 as f64, 
                    newpx);
            intery += gradient;
        }
    }
}

//http://members.chello.at/~easyfilter/bresenham.html
pub fn draw_ellipse<F>(ellipse: &Ellipse2D, mut draw: F, dimensions: (u32, u32), color: Rgba<u8>) 
        where F: FnMut(f64, f64, Rgba<u8>)
{
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

    loop {
        draw(x1 as f64 / dimensions.0 as f64, y0 as f64 / dimensions.1 as f64, color);
        draw(x0 as f64 / dimensions.0 as f64, y0 as f64 / dimensions.1 as f64, color);
        draw(x0 as f64 / dimensions.0 as f64, y1 as f64 / dimensions.1 as f64, color);
        draw(x1 as f64 / dimensions.0 as f64, y1 as f64 / dimensions.1 as f64, color);
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

    
    while (y0 as i32 - y1 as i32) < b as i32 {
        draw((x0-1) as f64 / dimensions.0 as f64, y0 as f64 / dimensions.1 as f64, color);
        draw((x1+1) as f64 / dimensions.0 as f64, y0 as f64 / dimensions.1 as f64, color);
        y0 += 1;
        draw((x0-1) as f64 / dimensions.0 as f64, y1 as f64 / dimensions.1 as f64, color);
        draw((x1+1) as f64 / dimensions.0 as f64, y1 as f64 / dimensions.1 as f64, color);
        y1 -= 1;
    }
}