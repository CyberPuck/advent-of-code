mod tail_simulator {
    use std::fs;

    #[derive(Debug)]
    /// NOTE: Assuming origin (starting point) is (0,0)
    struct Simulator {
        head: Point,
        tail: Point,
        tail_steps: u32,
    }

    impl Simulator {
        /// Given a direction, move the head rope 1 space, and check tail.
        fn update_rope(&mut self, direction: Direction) {
            // update head position
            match direction {
                Direction::UP => self.head.x += 1,
                Direction::DOWN => self.head.x -= 1,
                Direction::RIGHT => self.head.y += 1,
                Direction::LEFT => self.head.y -= 1,
            }

            // update tail position
            let y_diff: i32 = self.head.y - self.tail.y;
            let x_diff: i32 = self.head.x - self.tail.x;
            if self.head.x == self.tail.x && y_diff.abs() > 1 {
                self.tail.y += y_diff / y_diff.abs();
                self.tail_steps += 1;
            } else if self.head.y == self.tail.y && x_diff.abs() > 1 {
                self.tail.x += x_diff / x_diff.abs();
                self.tail_steps += 1;
            } else if self.head.x != self.tail.x
                && self.head.y != self.tail.y
                && (y_diff.abs() > 1 || x_diff.abs() > 1)
            {
                // diagonal update (update tail X, Y)
                self.tail.x += x_diff / x_diff.abs();
                self.tail.y += y_diff / y_diff.abs();
                self.tail_steps += 1;
            } else {
                println!(
                    "Should not move.\nHead: {:?}\nTail: {:?}",
                    self.head, self.tail,
                );
            }
        }
    }

    #[derive(Debug)]
    /// NOTE: Assuming there are no negative coordinates
    struct Point {
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
        let mut simulator = Simulator {
            head: Point { x: 0, y: 0 },
            tail: Point { x: 0, y: 0 },
            tail_steps: 0,
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
        return simulator.tail_steps;
    }
}

fn main() {
    let steps = tail_simulator::get_rope_moves("input1.txt".to_string());
    println!("Number of steps: {}", steps);
}
