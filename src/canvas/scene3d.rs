use ndarray::Array2;

pub enum Shape3D<'a> {
    Points3D(&'a mut Array2<f64>)
}