use super::Solution;
use fxhash::{FxHashMap, FxHashSet};

#[derive(Debug, PartialEq)]
struct Antenna {
    x: i32,
    y: i32,
}

// ...#...... Antinode: (0, 3)
// ..........
// ....a..... Antenna:  (2, 4)
// ..........
// .....a.... Antenna:  (4, 5) dy: -2, dx: -1
// ..........
// ......#... Antinode: (6, 6)

#[derive(Debug)]
struct Bounds {
    lower_x: i32,
    lower_y: i32,
    upper_x: i32,
    upper_y: i32,
}

impl Antenna {
    fn get_antinodes(&self, other: &Antenna, bounds: Bounds) -> Vec<(i32, i32)> {
        let Bounds {
            lower_x,
            lower_y,
            upper_x,
            upper_y,
        } = bounds;
        let mut out = vec![];

        let dx = self.x - other.x;
        let dy = self.y - other.y;

        let mut x = self.x;
        let mut y = self.y;
        let x_bound = lower_x..=upper_x;
        let y_bound = lower_y..=upper_y;
        loop {
            x += dx;
            y += dy;
            if x_bound.contains(&x) && y_bound.contains(&y) {
                out.push((x, y));
                continue;
            }
            break;
        }

        let mut x = other.x;
        let mut y = other.y;
        loop {
            x -= dx;
            y -= dy;
            if x_bound.contains(&x) && y_bound.contains(&y) {
                out.push((x, y));
                continue;
            }
            break;
        }

        out
    }
}

#[derive(Debug, Default)]
pub struct D8 {
    antennas: FxHashMap<char, Vec<Antenna>>,
    x_bound: i32,
    y_bound: i32,
}

impl Solution for D8 {
    fn prepare(&mut self, input: String) {
        for (row, line) in input.lines().enumerate() {
            self.y_bound = row as i32;
            for (col, c) in line.chars().enumerate() {
                self.x_bound = col as i32;
                if c.is_alphanumeric() {
                    self.antennas.entry(c).or_default().push(Antenna {
                        x: col as i32,
                        y: row as i32,
                    });
                }
            }
        }
    }

    fn p1(&mut self) -> String {
        let mut antinode_locations = FxHashSet::default();
        for antenna_list in self.antennas.values() {
            for i in 0..antenna_list.len() {
                for j in (i + 1)..antenna_list.len() {
                    let a = &antenna_list[i];
                    let b = &antenna_list[j];
                    let dx = (antenna_list[j].x - antenna_list[i].x).abs();
                    let dy = (antenna_list[j].y - antenna_list[i].y).abs();
                    let bounds = Bounds {
                        lower_x: (a.x.min(b.x) - dx).max(0),
                        upper_x: (a.x.max(b.x) + dx).min(self.x_bound),
                        lower_y: (a.y.min(b.y) - dy).max(0),
                        upper_y: (a.y.max(b.y) + dy).min(self.y_bound),
                    };
                    antinode_locations.extend(a.get_antinodes(b, bounds));
                }
            }
        }
        antinode_locations.len().to_string()
    }

    fn p2(&mut self) -> String {
        let mut antinode_locations = FxHashSet::default();
        for antenna_list in self.antennas.values() {
            for i in 0..antenna_list.len() {
                for j in (i + 1)..antenna_list.len() {
                    let a = &antenna_list[i];
                    let b = &antenna_list[j];
                    let bounds = Bounds {
                        lower_x: 0,
                        upper_x: self.x_bound,
                        lower_y: 0,
                        upper_y: self.y_bound,
                    };
                    antinode_locations.extend(a.get_antinodes(b, bounds));
                }
            }
        }

        // All antennae are also antinodes now
        for antennae in self.antennas.values() {
            antinode_locations.extend(antennae.iter().map(|a| (a.x, a.y)));
        }
        antinode_locations.len().to_string()
    }
}
