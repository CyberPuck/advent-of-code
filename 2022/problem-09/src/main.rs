mod tail_simulator {
    use std::{collections::HashMap, fs, iter::Map};

    #[derive(Debug)]
    /// NOTE: Assuming origin (starting point) is (0,0)
    pub struct Simulator {
        head: Point,
        tail: Point,
        tail_map: HashMap<Point, u32>,
    }

    impl Simulator {
        /// Given a direction, move the head rope 1 space, and check tail.
        fn update_rope(&mut self, direction: Direction) {
            // update head position
            match direction {
                Direction::UP => self.head.y += 1,
                Direction::DOWN => self.head.y -= 1,
                Direction::RIGHT => self.head.x += 1,
                Direction::LEFT => self.head.x -= 1,
            }

            // update tail position
            let y_diff: i32 = self.head.y - self.tail.y;
            let x_diff: i32 = self.head.x - self.tail.x;
            if self.head.x == self.tail.x && y_diff.abs() > 1 {
                self.tail.y += y_diff / y_diff.abs();
                self.tail_map.entry(self.tail).or_insert(0);
                self.tail_map
                    .insert(self.tail, self.tail_map.get(&self.tail).unwrap() + 1);
            } else if self.head.y == self.tail.y && x_diff.abs() > 1 {
                self.tail.x += x_diff / x_diff.abs();
                self.tail_map.entry(self.tail).or_insert(0);
                self.tail_map
                    .insert(self.tail, self.tail_map.get(&self.tail).unwrap() + 1);
            } else if self.head.x != self.tail.x
                && self.head.y != self.tail.y
                && (y_diff.abs() > 1 || x_diff.abs() > 1)
            {
                // diagonal update (update tail X, Y)
                self.tail.x += x_diff / x_diff.abs();
                self.tail.y += y_diff / y_diff.abs();
                self.tail_map.entry(self.tail).or_insert(0);
                self.tail_map
                    .insert(self.tail, self.tail_map.get(&self.tail).unwrap() + 1);
            } else {
                println!(
                    "Should not move.\nHead: {:?}\nTail: {:?}",
                    self.head, self.tail,
                );
            }
        }

        fn get_tail_count(&self) -> u32 {
            return self.tail_map.keys().len() as u32;
        }
    }

    #[derive(Debug, Eq, Hash, PartialEq, Copy, Clone)]
    /// NOTE: Assuming there are no negative coordinates
    pub struct Point {
        x: i32,
        y: i32,
    }

    #[derive(Debug)]
    struct Move {
        direction: Direction,
        steps: u32,
    }

    #[derive(Debug, Copy, Clone)]
    enum Direction {
        UP,
        DOWN,
        LEFT,
        RIGHT,
    }

    impl Direction {
        fn get_direction(string: &str) -> Direction {
            return match string {
                "U" => Direction::UP,
                "D" => Direction::DOWN,
                "L" => Direction::LEFT,
                "R" => Direction::RIGHT,
                _ => panic!("{} is not a valid direction.", string),
            };
        }
    }

    pub fn get_rope_moves(filename: String) -> u32 {
        let (simulator, moves) = parse_file(filename);
        println!("Sim start: {:?}", simulator);
        return run_simulation(simulator, moves);
    }

    fn parse_file(filename: String) -> (Simulator, Vec<Move>) {
        let file = fs::read_to_string(filename).unwrap();
        let lines = file.lines();
        let simulator = Simulator {
            head: Point { x: 0, y: 0 },
            tail: Point { x: 0, y: 0 },
            tail_map: HashMap::new(),
        };
        let mut moves: Vec<Move> = vec![];
        for line in lines {
            let line_components: Vec<&str> = line.split_ascii_whitespace().collect();
            if line_components.len() != 2 {
                panic!(
                    "Input file is malformed, only two entries per line please: {}",
                    line
                );
            }
            moves.push(Move {
                direction: Direction::get_direction(line_components[0]),
                steps: line_components[1].parse().unwrap(),
            });
        }
        return (simulator, moves);
    }

    fn run_simulation(mut simulator: Simulator, moves: Vec<Move>) -> u32 {
        for head_move in moves {
            for _step_number in 0..head_move.steps {
                simulator.update_rope(head_move.direction);
            }
        }
        return simulator.get_tail_count();
    }

    #[cfg(test)]
    mod tests {
        use crate::tail_simulator::Point;

        use super::*;

        #[test]
        fn test_lateral_move() {
            let mut sim = Simulator {
                head: Point { x: 2, y: 1 },
                tail: Point { x: 1, y: 1 },
                tail_map: HashMap::new(),
            };

            sim.update_rope(Direction::RIGHT);

            assert_eq!(sim.tail.x, 2, "Tail did not move");
            assert_eq!(sim.tail.y, 1, "Tail moved incorrectly");
            assert_eq!(sim.get_tail_count(), 1, "Step number is incorrect");
        }

        #[test]
        fn test_vertical_move() {
            let mut sim = Simulator {
                head: Point { x: 1, y: 2 },
                tail: Point { x: 1, y: 3 },
                tail_map: HashMap::new(),
            };

            sim.update_rope(Direction::DOWN);

            assert_eq!(sim.tail.y, 2, "Tail did not move");
            assert_eq!(sim.tail.x, 1, "Tail moved incorrectly");
            assert_eq!(sim.get_tail_count(), 1, "Step number is incorrect");
        }

        #[test]
        fn test_diagonal_moves() {
            let mut sim = Simulator {
                head: Point { x: 2, y: 2 },
                tail: Point { x: 1, y: 1 },
                tail_map: HashMap::new(),
            };

            sim.update_rope(Direction::UP);

            assert_eq!(sim.tail.x, 2, "Tail-1 x moved incorrectly");
            assert_eq!(sim.tail.y, 2, "Tail-1 y moved incorrectly");
            assert_eq!(sim.get_tail_count(), 1, "Step number is incorrect");

            let mut sim = Simulator {
                head: Point { x: 2, y: 2 },
                tail: Point { x: 1, y: 1 },
                tail_map: HashMap::new(),
            };

            sim.update_rope(Direction::RIGHT);

            assert_eq!(sim.tail.x, 2, "Tail-2 x moved incorrectly");
            assert_eq!(sim.tail.y, 2, "Tail-2 y moved incorrectly");
            assert_eq!(sim.get_tail_count(), 1, "Step number is incorrect");
        }
    }
}

fn main() {
    let steps = tail_simulator::get_rope_moves("input1.txt".to_string());
    println!("Number of steps: {}", steps);
}
