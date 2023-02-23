mod hill_climber {
    use core::fmt;
    use std::fs;

    struct Map {
        map_data: Vec<Vec<char>>,
        starting_point: Point,
        ending_point: Point,
    }

    impl Map {
        /// Function for finding the shortest path, currently implemented in
        /// Depth First Search.  It is *not* recursive, doing it iteratively.
        fn find_shortest_path(&self) -> u32 {
            println!("Starting search...");

            let mut total_count: u32 = 0;
            let mut exit_found = false;

            let mut current_points: Vec<(Point, u32)> = Vec::new();
            current_points.push((self.starting_point.clone(), total_count));
            let mut visited_points: Vec<Point> = Vec::new();

            // loop until the heat death of the universe... or we find the end solution.
            // Rust does not recommend recursion, switching to iterative model
            // 1. Have list of current points to visit
            // 2. Have list of visited points
            // Pop current points stack top
            // Add new neighbor points that can be visted to the current stack
            // Add current point to visited list
            // Might need to add metadata to points (count, indicator of getting to E)
            // Break out when `E` has been reached or current points is empty
            while current_points.len() > 0 && !exit_found {
                println!("Current at: {:?}", current_points);
                let current_point = current_points.remove(0);
                //let mut current_point_char = self.get_character(&current_point.0);
                let neighbors =
                    self.get_neighbors(&current_point.0, self.next_step(&current_point.0));
                total_count = current_point.1 + 1;
                for neighbor in neighbors {
                    println!(
                        "Neighbor: {}\tValue: {}",
                        neighbor,
                        self.get_character(&neighbor)
                    );
                    if self.get_character(&neighbor) == '{' {
                        println!("GET ME  THE FUCK OUT!");
                        exit_found = true;
                    }
                    if !visited_points.contains(&neighbor) {
                        let mut already_in_queue = false;
                        for point in &current_points {
                            if point.0 == neighbor {
                                already_in_queue = true;
                            }
                        }
                        if !already_in_queue {
                            current_points.push((neighbor, total_count));
                        }
                    }
                }
                visited_points.push(current_point.0);
                println!("Current_points size: {}", current_points.len());
            }

            // print out visited regions
            println!("Visited locations:");
            for y_index in 0..self.map_data.len() {
                let mut line_buffer: String = "".to_string();
                for x_index in 0..self.map_data[y_index].len() {
                    let test_point = Point {
                        x: x_index as u32,
                        y: y_index as u32,
                    };
                    if visited_points.contains(&test_point) {
                        line_buffer.push('x');
                    } else if test_point == self.starting_point {
                        line_buffer.push('S');
                    } else if test_point == self.ending_point {
                        line_buffer.push('E');
                    } else {
                        line_buffer.push('.');
                    }
                }
                println!("{}", line_buffer);
            }

            return total_count;
        }

        /// Given a point on the map, get the next char to visit
        fn next_step(&self, point: &Point) -> char {
            let current_char = self.get_character(point);
            let next_char = char::from_u32(current_char as u32 + 1).unwrap();

            // Note: 'S' next step is 'a' and 'z' next step is 'E'
            // All other ascii based characters should be + 1
            return match current_char {
                'S' => 'a',
                'z' => '{',
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

    #[derive(Debug, PartialEq, Clone)]
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
        return map.find_shortest_path();
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
                if row_chars[column_index] == 'E' {
                    // Please forgive me, '{' == (z as u32) + 1
                    // aka the convert end value 'E' to the correct value '{'
                    map.map_data[row_index].insert(column_index, '{');
                } else {
                    map.map_data[row_index].insert(column_index, row_chars[column_index]);
                }
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
            let steps = map.find_shortest_path();
            assert_eq!(steps, 2, "Steps should be 2 not {}", steps);

            let map = parse_file("sample3.txt".to_string());
            let steps = map.find_shortest_path();
            assert_eq!(steps, 28, "Steps should be 28 not {}", steps);
        }
    }
}

fn main() {
    let path_length = hill_climber::find_shortest_path("input1.txt".to_string());
    println!("Shortest path = {}", path_length);
}
