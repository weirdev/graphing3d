use ndarray::ArrayViewMut2;

pub enum Shape3D<'a> {
    Points3D(&'a mut ArrayViewMut2<'a, f64>)
}