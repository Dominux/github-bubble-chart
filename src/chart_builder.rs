use crate::common::types::{Number, PI};

pub struct ChartBuilder;

impl ChartBuilder {
    // https://matplotlib.org/stable/gallery/misc/packed_bubbles.html

    /// Applies some function to the given list of numbers to make differences between them smaller
    fn convert_to_radiuses(list: Vec<Number>) -> Vec<Number> {
        // for now the function is a getting a circle radius from the given area
        list.into_iter()
            .map(|n| (n / PI).sqrt()) // sqrt(n/π)
            .collect()
    }
}
