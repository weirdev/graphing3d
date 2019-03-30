use image::{ImageBuffer, Luma};
use std::io;
use std::path::Path;

mod scene2d;
mod scene3d;

pub use self::scene2d::{Shape2D, Line2D, Ellipse2D};
pub use self::scene3d::{Shape3D, Vector3D};

pub enum Scene {
    Scene2D(Vec<Shape2D>),
    Scene3D(Vec<Shape3D>)
}

pub trait Canvas {
    fn draw(&mut self, x: f64, y: f64, color: u8);
    fn draw_line2d(&mut self, line: &Line2D);
    fn draw_ellipse2d(&mut self, ellipse: &Ellipse2D);
    fn draw_point3d(&mut self, point: &Vector3D);
    fn save<Q>(&self, path: Q) -> io::Result<()> 
            where Q: AsRef<Path>;
    fn render(&mut self, scene: Scene);
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

    fn draw_line2d(&mut self, line: &Line2D) {
        let dims = self.img.dimensions();
        draw_line2d_antialiased(line, self, dims);
    }
    
    fn draw_ellipse2d(&mut self, ellipse: &Ellipse2D) {
        let dims = self.img.dimensions();
        draw_ellipse(ellipse, self, dims);
    }

    fn draw_point3d(&mut self, point: &Vector3D) {
        println!("1");
        let cameraloc = Vector3D {
            x: 0.5,
            y: 0.5,
            z: -2.0
        };
        let camdir = Vector3D {
            x: 0.0,
            y: 0.0,
            z: 1.0
        };

        let camtopoint = point - &cameraloc;

        let camxrad = camdir.x.acos();
        let camyrad = camdir.y.acos();

        println!("2");

        let ptxrad = (camtopoint.x / camtopoint.norm()).acos();
        let ptyrad = (camtopoint.y / camtopoint.norm()).acos();

        let relptxrad = ptxrad - camxrad;
        let relptyrad = ptyrad - camyrad;

        let totalviewrad= ((0.5 / 2.0) as f64).atan();

        let ptplotxpos = relptxrad / totalviewrad + 0.5;
        let ptplotypos = relptyrad / totalviewrad + 0.5;

        println!("{} {}", ptplotxpos, ptplotypos);

        self.draw_ellipse2d(&Ellipse2D {
            x0: ptplotxpos - 0.01,
            x1: ptplotxpos + 0.01,
            y0: ptplotypos - 0.01,
            y1: ptplotypos + 0.01
        })
    }

    fn render(&mut self, scene: Scene) {
        match scene {
            Scene::Scene2D(scene) => {
                    for shape in scene.iter() {
                        match shape {
                            Shape2D::Line2D(line) => self.draw_line2d(line),
                            Shape2D::Ellipse2D(ellipse) => self.draw_ellipse2d(ellipse)
                        }
                    }
                }
            Scene::Scene3D(scene) => {
                    for shape in scene.iter() {
                        match shape {
                            Shape3D::Point3D(point) => self.draw_point3d(point)
                        }
                    }
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

fn draw_line2d_antialiased(line: &Line2D, img: &mut impl Canvas, dimensions: (u32, u32)) {
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
fn draw_ellipse(ellipse: &Ellipse2D, img: &mut impl Canvas, dimensions: (u32, u32)) {
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

    
    while (y0 as i32 - y1 as i32) < b as i32 {
        img.draw((x0-1) as f64 / dimensions.0 as f64, y0 as f64 / dimensions.1 as f64, 255);
        img.draw((x1+1) as f64 / dimensions.0 as f64, y0 as f64 / dimensions.1 as f64, 255);
        y0 += 1;
        img.draw((x0-1) as f64 / dimensions.0 as f64, y1 as f64 / dimensions.1 as f64, 255);
        img.draw((x1+1) as f64 / dimensions.0 as f64, y1 as f64 / dimensions.1 as f64, 255);
        y1 -= 1;
    }
}