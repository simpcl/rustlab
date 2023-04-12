use std::collections::HashMap;

struct IslandFinder {
    marked_points: HashMap<i32, bool>,
    max_area: i32,
    max_x: i32,
    max_y: i32,
}

impl IslandFinder {
    fn find_max_island(&mut self, map2d: &Vec<Vec<i32>>) {
        println!(
            "find_max_island => lines: {}, columns: {}",
            map2d.len(),
            map2d[0].len()
        );
        for i in 0..map2d.len() {
            for j in 0..map2d[0].len() {
                let area = self.DFS(i as i32, j as i32, map2d);
                println!("area: {}, i: {}, j: {}", area, i, j);
                if self.max_area < area {
                    self.max_area = area;
                    self.max_x = i as i32;
                    self.max_y = j as i32;
                }
            }
        }
    }

    fn DFS(&mut self, i: i32, j: i32, map2d: &Vec<Vec<i32>>) -> i32 {
        let lines = map2d.len() as i32;
        let columns = map2d[0].len() as i32;
        if i < 0 || i >= lines || j < 0 || j >= columns {
            return 0;
        }
        if map2d[i as usize][j as usize] == 0 {
            return 0;
        }
        let pos = i * columns + j;
        if self.is_marked(pos) {
            return 0;
        }

        //println!("mark i: {}, j: {}", i, j);
        self.mark(pos);
        return 1
            + self.DFS(i - 1, j, map2d)
            + self.DFS(i, j - 1, map2d)
            + self.DFS(i + 1, j, map2d)
            + self.DFS(i, j + 1, map2d);
    }

    fn is_marked(&self, pos: i32) -> bool {
        match self.marked_points.get(&pos) {
            Some(_) => true,
            None => false,
        }
    }

    fn mark(&mut self, pos: i32) {
        self.marked_points.insert(pos, true);
    }

    fn show(&self) {
        println!(
            "max_area: {}, x: {}, y: {}",
            self.max_area, self.max_x, self.max_y
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_max_island_test() {
        let map2d = vec![
            vec![0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0],
            vec![0, 1, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 1, 0, 0, 1, 1, 0, 0, 1, 0, 1, 0, 0],
            vec![0, 1, 0, 0, 1, 1, 0, 0, 1, 1, 1, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0],
        ];
        let mut finder = IslandFinder {
            marked_points: HashMap::new(),
            max_area: 0,
            max_x: -1,
            max_y: -1,
        };
        finder.find_max_island(&map2d);
        finder.show();
    }
}
