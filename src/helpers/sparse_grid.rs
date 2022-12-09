use hashbrown::HashMap;

use super::grid_utils::Point;

#[derive(Clone)]
pub struct SparseGrid<T> {
    pub points: HashMap<Point, T>,
}

impl<T> SparseGrid<T> {
    pub fn get(&self, point: &Point) -> Option<&T> {
        self.points.get(point)
    }

    pub fn insert(&mut self, point: Point, data: T) {
        self.points.insert(point, data);
    }
}

impl<T> Default for SparseGrid<T> {
    fn default() -> Self {
        Self {
            points: HashMap::new(),
        }
    }
}
