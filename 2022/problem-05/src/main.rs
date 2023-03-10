pub mod crane_operation_plotter {
    use std::fs;

    #[derive(Debug)]
    struct CraneConfig {
        crates: Vec<Vec<Crate>>,
        procedures: Vec<CraneProcedure>,
    }

    impl Default for CraneConfig {
        fn default() -> Self {
            CraneConfig {
                crates: vec![vec![]],
                procedures: vec![],
            }
        }
    }

    #[derive(Clone, Debug)]
    struct Crate {
        name: char,
    }

    impl Default for Crate {
        fn default() -> Self {
            Crate { name: '_' }
        }
    }

    #[derive(Debug, Copy, Clone)]
    struct CraneProcedure {
        start_stack: u32,
        end_stack: u32,
        number_of_crates: u32,
    }

    impl Default for CraneProcedure {
        fn default() -> Self {
            CraneProcedure {
                start_stack: 0,
                end_stack: 0,
                number_of_crates: 0,
            }
        }
    }

    /// Given a crate file configuration, calculate the final crate configuration.
    ///
    /// ## Flag
    ///
    /// The `stack_move` flag (boolean) indicates if the crates are moved one
    /// at a time during a given crane procedure (false), or if all crates are
    /// moved at the same time (true).
    pub fn get_final_crate_config(file_name: String, stack_move: bool) -> String {
        let mut crate_config = "".to_string();
        let mut crane_config = parse_file(file_name);

        // run through the crane procedures
        if !stack_move {
            // each crate moved one at a time
            for procedure in crane_config.procedures {
                let crates = &mut crane_config.crates;
                for _n in 0..procedure.number_of_crates {
                    let moved_crate = crates[procedure.start_stack as usize].pop().unwrap();
                    crates[procedure.end_stack as usize].push(moved_crate);
                }
            }
        } else {
            // crates are stack moved, simulating by starting at an offset
            // index in the starting vector
            for procedure in crane_config.procedures {
                let crates = &mut crane_config.crates;
                for n in (0..procedure.number_of_crates).rev() {
                    let start_stack_length = crates[procedure.start_stack as usize].len() - 1;
                    let moved_crate = crates[procedure.start_stack as usize]
                        .remove(start_stack_length - n as usize);
                    crates[procedure.end_stack as usize].push(moved_crate);
                }
            }
        }

        // build up final answer
        for row in crane_config.crates {
            crate_config.push(row[row.len() - 1].name);
        }

        return crate_config;
    }

    fn parse_file(file_name: String) -> CraneConfig {
        let mut crane_config = CraneConfig::default();
        let file_data = fs::read_to_string(file_name).unwrap();
        let file_lines = file_data.lines();
        // First find the number of columns, generate crate config
        // Might not be necessary? (line 1 width + 1)/4 gets an acurrate answer
        // but I'm not confident this will hold muster to improper file input
        let mut crate_stacks: u32 = 0;
        let mut crate_config: Vec<String> = vec![];
        let mut crane_procedure: Vec<String> = vec![];
        for line in file_lines {
            if line.trim().starts_with('1') {
                let final_stack = line.trim().chars().rev().next().unwrap();
                crate_stacks = final_stack.to_digit(10).unwrap();
            } else if !line.starts_with("move") {
                crate_config.push(line.to_string());
            } else {
                crane_procedure.push(line.to_string());
            }
        }

        // Parse the intial crate config
        let mut initial_crates: Vec<Vec<Crate>> = vec![vec![]; crate_stacks as usize];
        for line in crate_config {
            let crate_entries: Vec<_> = line.match_indices("[").collect();
            for crate_tuple in crate_entries {
                let stack_index = crate_tuple.0 / 4;
                let new_crate = Crate {
                    name: line.chars().nth(crate_tuple.0 + 1).unwrap(),
                };
                let stack_vec = &mut initial_crates[stack_index];
                stack_vec.push(new_crate);
            }
        }

        // flip crate vectors to match last in first out
        for n in 0..initial_crates.len() {
            initial_crates[n].reverse();
        }
        crane_config.crates = initial_crates;

        // Parse the procedures
        for line in crane_procedure {
            if line.starts_with("move") {
                let splits: Vec<_> = line.split_whitespace().collect();
                let mut number_counter = 0;
                let mut procedure: CraneProcedure = CraneProcedure::default();
                for item in splits {
                    if item.chars().next().unwrap().is_numeric() {
                        // remember to subtract by 1 to match vector indexing
                        // NOTE: Keep number_of_crates same to actually move crates
                        if number_counter == 0 {
                            procedure.number_of_crates = u32::from_str_radix(item, 10).unwrap();
                        } else if number_counter == 1 {
                            procedure.start_stack = u32::from_str_radix(item, 10).unwrap() - 1;
                        } else if number_counter == 2 {
                            procedure.end_stack = u32::from_str_radix(item, 10).unwrap() - 1;
                        }
                        number_counter += 1;
                    }
                }
                crane_config.procedures.push(procedure);
            }
        }

        return crane_config;
    }
}

fn main() {
    let final_config =
        crane_operation_plotter::get_final_crate_config("input1.txt".to_string(), true);
    println!("Final config: {}", final_config);
}
