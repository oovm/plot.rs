use std::fs::File;
use svg::Document;
use crate::bar_chart::BarChart;

pub(crate) enum BarChartInner {
    List(Vec<f32>),
}

pub struct Rectangle {
    /// The x-coordinate of the upper left point
    x: f64,
    /// The y-coordinate of the upper left point
    y: f64,
    /// The width of the rectangle
    width: f64,
    /// The height of the rectangle
    height: f64,
}

impl Rectangle {
    pub fn to_svg(&self) -> svg::node::element::Rectangle {
        svg::node::element::Rectangle::new()
            .set("x", self.x)
            .set("y", self.y)
            .set("width", self.width)
            .set("height", self.height)
    }
}


impl BarChartInner {
    pub fn plot(&self, config: &BarChart) -> Document {
        match self {
            Self::List(s) => {
                let mut doc = Document::new().set("viewBox", (0, 0, 100, 100));
                let mut x = config.x_range.start;
                let mut y = config.y_range.start;
                let mut width = (config.x_range.end - config.x_range.start) / s.len() as f64;
                let mut height = (config.y_range.end - config.y_range.start) / s.len() as f64;
                for i in s.iter() {
                    doc = doc.add(Rectangle { x, y, width, height }.to_svg());
                    x += width;
                    y += height;
                }
                doc
            }
        }
    }
}

#[test]
fn test() {
    let data = BarChartInner::List(vec![1.0, 2.0, 3.0]);
    let config = BarChart {
        x_range: 0.0..10.0,
        y_range: 0.0..10.0,
        interpolation: 0,
    };
    let doc = data.plot(&config);
    let mut out = File::create("bar-chart.svg").unwrap();
    svg::write(&mut out, &doc).unwrap();
}