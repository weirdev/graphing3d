extern crate image;
#[macro_use]
extern crate ndarray;

mod canvas;

use std::slice;
use canvas::{Canvas, ImageCanvas, Line2D, Shape2D, Ellipse2D, Scene, Shape3D};
use ndarray::{aview_mut1};

#[no_mangle]
pub extern "C" fn subroutine1() {
    let mut img = ImageCanvas::blank(512, 512, 255);
    let mut scene: Vec<(Shape2D, u8)> = Vec::new();
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
    scene.push((Shape2D::Line2D(line1), 0));
    scene.push((Shape2D::Ellipse2D(ellipse1), 0));
    img.render(Scene::Scene2D(scene));
    img.save("line.png").unwrap();
}

#[no_mangle]
pub extern "C" fn plot3d_scatter(point_buffer: *mut f64, pointcount: usize) {
    if point_buffer.is_null() {
        return;
    }
    let point_buffer = unsafe { slice::from_raw_parts_mut(point_buffer, 3*pointcount) };

    let mut points = aview_mut1(point_buffer).into_shape((pointcount, 3)).unwrap();
    let mut img = ImageCanvas::blank(512, 512, 255);
    img.render(Scene::Scene3D(vec![(Shape3D::Points3D(&mut points), 0)]));
    img.save("pointspython.png").unwrap();
}
