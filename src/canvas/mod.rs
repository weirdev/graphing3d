use std::io;
use std::path::Path;
use std::f64::consts::PI;
use image::{ImageBuffer, Rgba};
use ndarray::{Array, Array1, Zip, aview_mut1, ArrayViewMut2};

mod drawing;

pub use self::drawing::{Shape2D, Line2D, Ellipse2D, Shape3D};
pub use self::drawing::{draw_line2d_antialiased, draw_ellipse};

pub enum Scene<'a> {
    Scene2D(Vec<(Shape2D, Rgba<u8>)>),
    Scene3D(Vec<(Shape3D<'a>, Rgba<u8>)>)
}

pub trait Canvas {
    fn draw(&mut self, x: f64, y: f64, color: Rgba<u8>);
    fn draw_line2d(&mut self, line: &Line2D, color: Rgba<u8>);
    fn draw_ellipse2d(&mut self, ellipse: &Ellipse2D, color: Rgba<u8>);

    // axes_rotation is Tait–Bryan angle vector in radians (x, y, z)
    fn draw_points3d(&mut self, points: &mut ArrayViewMut2<f64>, color: Rgba<u8>, axes_rotation: &Array1<f64>);

    fn save<Q>(&self, path: Q) -> io::Result<()> 
            where Q: AsRef<Path>;

    // axes_rotation is Tait–Bryan angle vector in radians (x, y, z)
    fn render(&mut self, scene: Scene, axes_rotation: Option<&Array1<f64>>);
}

pub struct ImageCanvas {
    img: ImageBuffer<Rgba<u8>, Vec<u8>>
}

impl ImageCanvas {
    pub fn blank(x: u32, y: u32, background: Rgba<u8>) -> ImageCanvas {
        ImageCanvas {
            img: <ImageBuffer<Rgba<u8>, Vec<u8>>>::from_pixel(x, y, background)
        }
    }
}

impl Canvas for ImageCanvas {
    fn draw(&mut self, x: f64, y: f64, color: Rgba<u8>) {
        let xcoord = (x * self.img.dimensions().0 as f64).round() as u32;
        // image y axis 0 at top, but for Canvas we use bottom as 0
        let ycoord = ((1.0 - y) * self.img.dimensions().1 as f64).round() as u32;
        self.img.put_pixel(xcoord, ycoord, color);
    }

    fn save<Q>(&self, path: Q) -> io::Result<()> 
            where Q: AsRef<Path> {
        self.img.save(path)
    }

    fn draw_line2d(&mut self, line: &Line2D, color: Rgba<u8>) {
        let dims = self.img.dimensions();
        draw_line2d_antialiased(line, |x, y, c| self.draw(x, y, c), dims, color);
    }
    
    fn draw_ellipse2d(&mut self, ellipse: &Ellipse2D, color: Rgba<u8>) {
        let dims = self.img.dimensions();
        draw_ellipse(ellipse, |x, y, c| self.draw(x, y, c), dims, color);
    }

    // https://en.wikipedia.org/wiki/Euler_angles#Rotation_matrix passive rotation
    // TODO: points matrix should be transformed in place
    // axes_rotation is Tait–Bryan angle vector in radians (x, y, z)
    fn draw_points3d(&mut self, points: &mut ArrayViewMut2<f64>, color: Rgba<u8>, axes_rotation: &Array1<f64>) {
        let origin = array![0.5, 0.5, 0.5];

        // Translate points to be relative to 
        *points -= &origin;

        let cosx = axes_rotation[0].cos();
        let cosy = axes_rotation[1].cos();
        let cosz = axes_rotation[2].cos();

        let sinx = axes_rotation[0].sin();
        let siny = axes_rotation[1].sin();
        let sinz = axes_rotation[2].sin();

        // TODO: simplify this
        let transform_matrix = array![
            [cosy*cosz,     cosx*sinz + cosz*sinx*siny, sinx*sinz - cosx*cosz*siny],
            [-cosy*sinz,    cosx*cosz + sinx*siny*sinz, cosz*sinx + cosx*siny*sinz],
            [siny,          -cosy*sinx,                 cosx*cosy]];

        // TODO: Not in-place
        let mut transf_points = points.dot(&transform_matrix.t());
        
        // Relative to origin
        let cameraloc = array![0.0, 0.0, -2.5];
        // Translate points to be relative to the camera location
        transf_points -= &cameraloc;

        // Relative to cameraloc
        let display_loc = array![0.0, 0.0, 1.0];

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
            }, color);
        }
    }

    // axes_rotation is Tait–Bryan angle vector in radians (x, y, z)
    fn render(&mut self, scene: Scene, axes_rotation: Option<&Array1<f64>>) {
        let no_rotation = array![0.0, 0.0, 0.0];
        let set_axes_rotation = match axes_rotation {
            Some(ar) => ar,
            None => &no_rotation
        };
        
        match scene {
            Scene::Scene2D(scene) => {
                    for (shape, color) in scene.iter() {
                        match shape {
                            Shape2D::Line2D(line) => self.draw_line2d(line, *color),
                            Shape2D::Ellipse2D(ellipse) => self.draw_ellipse2d(ellipse, *color)
                        }
                    }
                }
            Scene::Scene3D(mut scene) => {
                    for (shape, color) in scene.iter_mut() {
                        match shape {
                            Shape3D::Points3D(ref mut points) => self.draw_points3d(*points, *color, set_axes_rotation)
                        }
                    }
                }
        }
    }
}
