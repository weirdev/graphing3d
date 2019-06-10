extern crate image;
#[macro_use]
extern crate ndarray;

use image::Rgba;

mod canvas;

use canvas::{Canvas, ImageCanvas, Line2D, Shape2D, Ellipse2D, Scene, Shape3D};

fn main() {
    let mut img = ImageCanvas::blank(512, 512, Rgba([255, 255, 255, 255]));
    let mut scene: Vec<(Shape2D, Rgba<u8>)> = Vec::new();
    let line1 = Line2D {
        x0: 0.1,
        y0: 0.2,
        x1: 0.9,
        y1: 0.7
    };
    let ellipse1 = Ellipse2D {
        x0: 0.1,
        y0: 0.1,
        x1: 0.5,
        y1: 0.9
    };
    scene.push((Shape2D::Line2D(line1), Rgba([0, 0, 0, 255])));
    scene.push((Shape2D::Ellipse2D(ellipse1), Rgba([0, 0, 0, 255])));
    img.render(Scene::Scene2D(scene), None);
    img.save("line.png").unwrap();

    let mut points = array![[0.2, 0.7, 0.0],
        [0.2, 0.7, 0.5],
        [0.2, 0.7, 1.0]];

    img = ImageCanvas::blank(512, 512, Rgba([255, 255, 255, 255]));
    img.render(Scene::Scene3D(vec![(Shape3D::Points3D(&mut points.view_mut()), Rgba([0, 0, 0, 255]))]), None);
    img.save("points2.png").unwrap();
}
