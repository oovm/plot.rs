use std::ops::Range;
use std::slice::Iter;
use svg::Document;
use self::data::BarChartInner;

mod data;


/// [ListLinePlot](https://reference.wolfram.com/language/ref/ListLinePlot.html)
pub struct BarChart {
    x_range: Range<f64>,
    y_range: Range<f64>,
    interpolation: u8,
}


impl BarChart {
    pub fn plot(&self, data: &BarChartInner) -> Document {
        data.plot(self)
    }
}