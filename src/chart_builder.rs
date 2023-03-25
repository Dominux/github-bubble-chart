use crate::common::types::{Float, Int, PI};

pub struct ChartBuilder;

impl ChartBuilder {
    /// Applies some function to the given list of numbers to make differences between them smaller
    fn convert_to_radiuses(list: Vec<Int>) -> Vec<Int> {
        // for now the function is a getting a circle radius from the given area
        list.into_iter()
            .map(|n| (n as Float / PI).sqrt().round() as Int) // sqrt(n/π)
            .collect()
    }
}
