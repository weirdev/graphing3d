pub struct Line2D {
    pub x0: f64,
    pub y0: f64,
    pub x1: f64,
    pub y1: f64
}

pub struct Ellipse2D {
    pub x0: f64,
    pub y0: f64,
    pub x1: f64,
    pub y1: f64
}

pub enum Shape2D {
    Line2D(Line2D),
    Ellipse2D(Ellipse2D)
}
