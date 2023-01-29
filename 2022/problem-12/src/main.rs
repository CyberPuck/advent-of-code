mod hill_climber {
    use core::fmt;
    use std::fs;

    struct Map {
        map_data: Vec<Vec<char>>,
        starting_point: Point,
        ending_point: Point,
    }

    impl Map {
        fn find_shortest_path(&self) -> u32 {
            let steps = 0;

            return steps;
        }
    }

    impl fmt::Display for Map {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            let mut line_data: String = "".to_string();
            for y_index in 0..self.map_data.len() {
                for x_index in 0..self.map_data[y_index].len() {
                    line_data.push(self.map_data[y_index][x_index]);
                }
                line_data.push('\n');
            }
            write!(f, "{}", line_data)
        }
    }

    /// Index location based on 2D array
    struct Point {
        x: u32,
        y: u32,
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
            // setup the row vector
            map.map_data[row_index] = vec![' '; row_chars.len()];
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
}

fn main() {
    let path_length = hill_climber::find_shortest_path("sample1.txt".to_string());
    println!("Shortest path = {}", path_length);
}
