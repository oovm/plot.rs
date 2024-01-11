use std::ops::Range;
use svg::Document;

/// [ListLinePlot](https://reference.wolfram.com/language/ref/ListLinePlot.html)
pub struct LineChart {
    x_range: Option<Range<f64>>,
    y_range: Option<Range<f64>>,
    interpolation: u8,
}

pub struct LineData {
    points: Vec<f32>,
}


impl LineData {
    pub fn to_svg(&self) -> Document {

    }
}