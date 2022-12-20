pub mod crane_operation_plotter {
    use std::fs;

    struct CraneConfig {
        crates: Vec<Vec<Crate>>,
        procedures: Vec<CraneProcedure>,
    }

    impl Default for CraneConfig {
        fn default() -> Self {
            CraneConfig {
                crates: vec![vec![Crate::default()]],
                procedures: vec![CraneProcedure::default()],
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

    pub fn get_final_crate_config(file_name: String) -> String {
        let crate_config = "".to_string();
        let _crane_config = parse_file(file_name);
        return crate_config;
    }

    fn parse_file(file_name: String) -> CraneConfig {
        let crane_config = CraneConfig::default();
        let file_data = fs::read_to_string(file_name).unwrap();
        let file_lines = file_data.lines();
        // First find the number of columns, generate crate config
        // Might not be necessary? (line 1 width + 1)/4 gets an acurrate answer
        // but I'm not confident this will hold muster to improper file input
        let mut crate_stacks: u32 = 0;
        let mut crate_config: Vec<String> = vec![];
        for line in file_lines {
            if line.trim().starts_with('1') {
                let final_stack = line.trim().chars().rev().next().unwrap();
                crate_stacks = final_stack.to_digit(10).unwrap();
                break;
            } else {
                crate_config.push(line.to_string());
            }
        }

        println!("Number stacks: {}", crate_stacks);

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
        println!("Test 1: {:?}", initial_crates);

        // Parse the instructions

        return crane_config;
    }
}

fn main() {
    let final_config = crane_operation_plotter::get_final_crate_config("sample1.txt".to_string());
    println!("Final config: {}", final_config);
}
