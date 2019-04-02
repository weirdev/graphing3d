use std::io;
use std::path::Path;
use image::{ImageBuffer, Luma};
use ndarray::{Array, Array1, Array2, Zip, aview_mut1};

mod drawing;

pub use self::drawing::{Shape2D, Line2D, Ellipse2D, Shape3D};
pub use self::drawing::{draw_line2d_antialiased, draw_ellipse};

pub enum Scene<'a> {
    Scene2D(Vec<Shape2D>),
    Scene3D(Vec<Shape3D<'a>>)
}

pub trait Canvas {
    fn draw(&mut self, x: f64, y: f64, color: u8);
    fn draw_line2d(&mut self, line: &Line2D);
    fn draw_ellipse2d(&mut self, ellipse: &Ellipse2D);
    fn draw_points3d(&mut self, points: &mut Array2<f64>);
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
        draw_line2d_antialiased(line, |x, y, c| self.draw(x, y, c), dims);
    }
    
    fn draw_ellipse2d(&mut self, ellipse: &Ellipse2D) {
        let dims = self.img.dimensions();
        draw_ellipse(ellipse, |x, y, c| self.draw(x, y, c), dims);
    }

    fn draw_points3d(&mut self, points: &mut Array2<f64>) {
        let cameraloc = array![0.5, 0.5, -2.0];

        // Tait–Bryan angle vector in radians
        let camdir: Array1<f64> = array![0.0, 0.0, 0.0];

        // Relative to cameraloc
        let display_loc = array![0.0, 0.0, 1.0];

        *points -= &cameraloc;

        let mut transform_matrix = array![
            [1.0, 0.0, 0.0],
            [0.0, camdir[0].cos(), camdir[0].sin()],
            [0.0, -camdir[0].sin(), camdir[0].cos()]];
        
        transform_matrix = transform_matrix.dot(&array![
            [camdir[1].cos(), 0.0, -camdir[1].sin()],
            [0.0, 1.0, 0.0],
            [camdir[1].sin(), 0.0, camdir[1].cos()]]);

        transform_matrix = transform_matrix.dot(&array![
            [camdir[2].cos(), camdir[2].sin(), 0.0],
            [-camdir[2].sin(), camdir[2].cos(), 0.0],
            [0.0, 0.0, 1.0]]);
        
        let transf_points = points.dot(&transform_matrix.t());

        let mut proj_points = Array::zeros((points.dim().0, 2));

        Zip::from(proj_points.genrows_mut())
            .and(transf_points.genrows())
            .apply(|mut proj, pt3d| 
                    proj.assign(&aview_mut1(&mut 
                        [(display_loc[2] / pt3d[2]) * pt3d[0] + display_loc[0],
                        (display_loc[2] / pt3d[2]) * pt3d[1] + display_loc[1]])));

        let fov_side = 0.5;
        proj_points /= fov_side;
        proj_points += 0.5;
        
        for point in proj_points.genrows() {
            self.draw_ellipse2d(&Ellipse2D {
                x0: point[0] - 0.01,
                x1: point[0] + 0.01,
                y0: point[1] - 0.01,
                y1: point[1] + 0.01
            });
        }
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
            Scene::Scene3D(mut scene) => {
                    for shape in scene.iter_mut() {
                        match shape {
                            Shape3D::Points3D(ref mut points) => self.draw_points3d(*points)
                        }
                    }
                }
        }
    }
}
