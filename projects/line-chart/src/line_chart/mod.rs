use std::ops::Range;


/// [ListLinePlot](https://reference.wolfram.com/language/ref/ListLinePlot.html)
pub struct LineChart {
    x_range: Range<f64>,
    y_range: Range<f64>,
    interpolation: u8,
}

pub struct LineData {
    points: Vec<f32>,
}


