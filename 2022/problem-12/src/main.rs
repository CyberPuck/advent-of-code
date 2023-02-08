mod hill_climber {
    use core::fmt;
    use std::fs;

    struct Map {
        map_data: Vec<Vec<char>>,
        starting_point: Point,
        ending_point: Point,
    }

    impl Map {
        fn find_shortest_path(&self, starting_point: &Point, mut count: u32) -> u32 {
            let next_step = self.next_step(starting_point);
            println!("At {}, finding: {}", starting_point, next_step);

            let next_neighbors = self.get_neighbors(starting_point, next_step);
            println!("Found the following neighbors: {:?}", next_neighbors);

            for neighbor in next_neighbors {
                if self.map_data[neighbor.y as usize][neighbor.x as usize] == 'E' {
                    return 1;
                }
                count = self.find_shortest_path(&neighbor, count + 1);
            }

            return count;
        }

        /// Given a point on the map, get the next char to visit
        fn next_step(&self, point: &Point) -> char {
            let current_char = self.get_character(point);
            let next_char = char::from_u32(current_char as u32 + 1).unwrap();

            // Note: 'S' next step is 'a' and 'z' next step is 'E'
            // All other ascii based characters should be + 1
            return match current_char {
                'S' => 'a',
                'z' => 'E',
                _ => next_char,
            };
        }

        /// Simple function to get the character at the given point
        fn get_character(&self, point: &Point) -> char {
            return *self
                .map_data
                .get(point.y as usize)
                .unwrap()
                .get(point.x as usize)
                .unwrap();
        }

        /// Given a coordinate and the value to look for, list all *orthogonal*
        /// neighbors that match `next_step`.
        fn get_neighbors(&self, point: &Point, next_step: char) -> Vec<Point> {
            let mut matching_points: Vec<Point> = Vec::new();

            // Checking both x-axis points
            let y_axis_row = self.map_data.get(point.y as usize);
            if y_axis_row.is_some() {
                let x_axis_plus = y_axis_row.unwrap().get(point.x as usize + 1);
                if x_axis_plus.is_some()
                    && (x_axis_plus.unwrap().eq(&next_step)
                        || x_axis_plus.unwrap() <= &self.get_character(point))
                {
                    matching_points.push(Point {
                        x: point.x + 1,
                        y: point.y,
                    });
                }
                if point.x > 0 {
                    let x_axis_minus = y_axis_row.unwrap().get(point.x as usize - 1);
                    if x_axis_minus.is_some()
                        && (x_axis_minus.unwrap().eq(&next_step)
                            || x_axis_minus.unwrap() <= &self.get_character(point))
                    {
                        matching_points.push(Point {
                            x: point.x - 1,
                            y: point.y,
                        });
                    }
                }
            }

            // Checking both y-axis points
            if point.y > 0 {
                let y_axis_row = self.map_data.get(point.y as usize - 1);
                if y_axis_row.is_some() {
                    let x_axis = y_axis_row.unwrap().get(point.x as usize);
                    if x_axis.is_some()
                        && (x_axis.unwrap().eq(&next_step)
                            || x_axis.unwrap() <= &self.get_character(point))
                    {
                        matching_points.push(Point {
                            x: point.x,
                            y: point.y - 1,
                        });
                    }
                }
            }

            let y_axis_row = self.map_data.get(point.y as usize + 1);
            if y_axis_row.is_some() {
                let x_axis = y_axis_row.unwrap().get(point.x as usize);
                if x_axis.is_some()
                    && (x_axis.unwrap().eq(&next_step)
                        || x_axis.unwrap() <= &self.get_character(point))
                {
                    matching_points.push(Point {
                        x: point.x,
                        y: point.y + 1,
                    });
                }
            }

            return matching_points;
        }
    }

    impl fmt::Display for Map {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            let mut line_data: String = "".to_string();
            line_data.push_str(format!("Starting point = {}\n", self.starting_point).as_str());
            line_data.push_str(format!("Ending point = {}\n", self.ending_point).as_str());
            for y_index in 0..self.map_data.len() {
                for x_index in 0..self.map_data[y_index].len() {
                    line_data.push(self.map_data[y_index][x_index]);
                }
                line_data.push('|');
                line_data.push('\n');
            }
            write!(f, "{}", line_data)
        }
    }

    #[derive(Debug)]
    /// Index location based on 2D array
    struct Point {
        x: u32,
        y: u32,
    }

    impl fmt::Display for Point {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "({}, {})", self.x, self.y)
        }
    }

    pub fn find_shortest_path(file_name: String) -> u32 {
        let map = parse_file(file_name);
        println!("Map = \n{}", map);
        return map.find_shortest_path(&map.starting_point, 0);
    }

    fn parse_file(file_name: String) -> Map {
        let file_data = fs::read_to_string(file_name).unwrap();
        let file_lines: Vec<&str> = file_data.lines().collect();
        let mut map = Map {
            map_data: vec![Vec::new(); file_lines.len()],
            starting_point: Point { x: 0, y: 0 },
            ending_point: Point { x: 0, y: 0 },
        };

        for row_index in 0..file_lines.len() {
            let row_chars: Vec<char> = file_lines[row_index].chars().collect();

            for column_index in 0..row_chars.len() {
                // store the start and end points
                if row_chars[column_index] == 'S' {
                    map.starting_point = Point {
                        x: column_index as u32,
                        y: row_index as u32,
                    };
                } else if row_chars[column_index] == 'E' {
                    map.ending_point = Point {
                        x: column_index as u32,
                        y: row_index as u32,
                    };
                }
                // store data in map
                map.map_data[row_index].insert(column_index, row_chars[column_index]);
            }
        }

        return map;
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_convert_char() {
            let test_char = 'f';
            let next_char = char::from_u32(test_char as u32 + 1).unwrap();
            assert_eq!(
                next_char, 'g',
                "The next char should be /'g/', not {}",
                next_char
            );
        }

        #[test]
        fn test_neighbors() {
            let map = parse_file("sample1.txt".to_string());

            // check starting point
            let neighbors = map.get_neighbors(&map.starting_point, 'a');
            assert_eq!(
                neighbors.len(),
                2,
                "Neighbors found: {} != 2",
                neighbors.len()
            );
            // check a middle of the map point
            let neighbors = map.get_neighbors(&Point { x: 2, y: 2 }, 'd');
            assert_eq!(
                neighbors.len(),
                3,
                "Neighbors found: {} != 3",
                neighbors.len()
            );
            // check a bottom of the map point
            let neighbors = map.get_neighbors(&Point { x: 2, y: 4 }, 'e');
            assert_eq!(
                neighbors.len(),
                3,
                "Neighbors found: {} != 3",
                neighbors.len()
            );
            // check right of the map point
            let neighbors = map.get_neighbors(&Point { x: 7, y: 3 }, 'k');
            assert_eq!(
                neighbors.len(),
                2,
                "Neighbors found: {} != 2",
                neighbors.len()
            );
        }

        #[test]
        fn test_step_calc_simple() {
            let map = parse_file("sample2.txt".to_string());
            let steps = map.find_shortest_path(&map.starting_point, 0);
            assert_eq!(steps, 1, "Steps should be 1 not {}", steps);
        }
    }
}

fn main() {
    let path_length = hill_climber::find_shortest_path("sample1.txt".to_string());
    println!("Shortest path = {}", path_length);
}
