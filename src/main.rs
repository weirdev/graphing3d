extern crate image;

mod canvas;

use canvas::{Canvas, ImageCanvas, Line2D, Shape2D, Ellipse2D, Scene, Vector3D, Shape3D};

fn main() {
    let mut img = ImageCanvas::blank(512, 512);
    let mut scene: Vec<Shape2D> = Vec::new();
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
    scene.push(Shape2D::Line2D(line1));
    scene.push(Shape2D::Ellipse2D(ellipse1));
    img.render(Scene::Scene2D(scene));
    img.save("line.png").unwrap();

    let point1 = Vector3D {
        x: 0.2,
        y: 0.7,
        z: 0.5
    };

    img = ImageCanvas::blank(512, 512);
    img.render(Scene::Scene3D(vec![Shape3D::Point3D(point1)]));
    img.save("line.png").unwrap();
}
