pub fn grid_dimensions(s: &str) -> (usize, usize) {
    let mut lines = s.lines();
    let first = lines.next().unwrap();
    (first.len(), lines.count() + 1)
}
