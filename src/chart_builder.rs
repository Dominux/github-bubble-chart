use ndarray::prelude::*;

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

    pub fn new(list: Vec<Number>, bubble_spacing: Option<Number>) {
        let area = Array::from_vec(list);
        let r = Self::convert_to_radiuses(list);

        let bubbles = Array2::<Number>::ones((area.len(), 4).f());
    }
}
