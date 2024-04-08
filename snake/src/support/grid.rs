pub struct Grid<T> {
    width: usize,
    height: usize,
    storage: Box<[T]>,
}

#[allow(dead_code)]
impl<T> Grid<T> {
    pub fn new(width: usize, height: usize, cells: impl IntoIterator<Item = T>) -> Self {
        let cells: Vec<T> = cells.into_iter().collect();
        assert_eq!(
            cells.len(),
            width * height,
            "Not the right number of cells for the given width and height"
        );
        Self {
            width,
            height,
            storage: cells.into_boxed_slice(),
        }
    }
}