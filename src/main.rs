extern crate image;

mod canvas;

use canvas::{Canvas, ImageCanvas, Line, Shape, Ellipse};

fn main() {
    let mut img = ImageCanvas::blank(512, 512);
    let mut scene: Vec<Shape> = Vec::new();
    let line1 = Line {
        x0: 0.1,
        y0: 0.2,
        x1: 0.9,
        y1: 0.7
    };
    let ellipse1 = Ellipse {
        x0: 0.1,
        y0: 0.1,
        x1: 0.5,
        y1: 0.9
    };
    scene.push(Shape::Line(line1));
    scene.push(Shape::Ellipse(ellipse1));
    img.render(scene);
    img.save("line.png").unwrap();
}
