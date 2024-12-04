/// All directional offsets from a location in a 2D grid
pub const ALL_OFFSETS: &[(i32, i32)] = &[
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

/// N, S, E, W offsets from a location in a 2d grid
pub const CARDINAL_OFFSETS: &[(i32, i32)] = &[(-1, 0), (0, -1), (0, 1), (1, 0)];

/// Diagonal offsets from a location in a 2d grid
pub const DIAGONAL_OFFSETS: &[(i32, i32)] = &[(-1, -1), (-1, 1), (1, -1), (1, 1)];

pub trait GridUtils {
    fn in_bounds(&self, row: i32, col: i32) -> bool;
    fn height(&self) -> usize;
    fn width(&self) -> usize;
}

impl<T> GridUtils for Vec<Vec<T>> {
    fn in_bounds(&self, row: i32, col: i32) -> bool {
        (0..self.height() as i32).contains(&row) && (0..self.width() as i32).contains(&col)
    }

    fn height(&self) -> usize {
        self.len()
    }

    fn width(&self) -> usize {
        if self.is_empty() {
            0
        } else {
            self[0].len()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bounds_checks() {
        let grid = vec![vec!['a', 'b'], vec!['c', 'd'], vec!['1', '2']];

        assert!(!grid.in_bounds(0, -1));
        assert!(!grid.in_bounds(-1, 0));
        assert!(!grid.in_bounds(0, 2));
        assert!(!grid.in_bounds(3, 0));

        for row in 0..grid.len() {
            for col in 0..grid[0].len() {
                assert!(grid.in_bounds(row as i32, col as i32));
            }
        }
    }

    #[test]
    fn grid_height() {
        let empty: Vec<Vec<u8>> = vec![];
        assert_eq!(empty.height(), 0);
        assert_eq!(vec![vec![1]].height(), 1);
        assert_eq!(vec![vec![1, 2]].height(), 1);
        assert_eq!(vec![vec![1], vec![2]].height(), 2);
    }

    #[test]
    fn grid_width() {
        let empty: Vec<Vec<u8>> = vec![];
        assert_eq!(empty.width(), 0);
        assert_eq!(vec![vec![1]].width(), 1);
        assert_eq!(vec![vec![1], vec![2]].width(), 1);
        assert_eq!(vec![vec![1, 2], vec![2, 3]].width(), 2);
    }
}
