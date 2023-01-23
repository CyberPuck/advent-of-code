mod cpu_simulator {
    use std::fs;

    enum Instruction {
        NOOP,
        ADDX {integer: i32},
    }

    pub fn get_register_value(file_name: String) -> i32 {
        let instructions = parse_file(file_name);
        let cpu_cycles = simulate_cpu_ops(instructions);
        draw_incoming_image(&cpu_cycles);
        let register_x = calculate_register_x_value(cpu_cycles);

        return register_x;
    }

    fn parse_file(file_name: String) -> Vec<Instruction> {
        let mut instruction_list: Vec<Instruction> = Vec::new();
        let file = fs::read_to_string(file_name).unwrap();

        for line in file.lines() {
            if line.to_ascii_lowercase().contains("noop") {
                instruction_list.push(Instruction::NOOP);
            } else {
                let addx_instruction: Vec<&str> = line.split_whitespace().collect();
                instruction_list.push(Instruction::ADDX { integer: addx_instruction[1].parse::<i32>().unwrap() });
            }
        }

        return instruction_list;
    }

    /// Given a list of CPU instructions, execute and return a list of register
    /// `x` at all cycles
    fn simulate_cpu_ops(instructions: Vec<Instruction>) -> Vec<i32> {
        let mut register_x = 1;
        // adding the first cycle to the list
        // looks like indexing was off by one?
        let mut cpu_cycles: Vec<i32> = vec![register_x];
        for instruction in instructions {
            match instruction {
                Instruction::NOOP => {
                    cpu_cycles.push(register_x);
                },
                Instruction::ADDX { integer } => {
                    cpu_cycles.push(register_x);
                    cpu_cycles.push(register_x);
                    register_x = register_x + integer;
                },
            }
            println!("X = {} @ cycle = {}", register_x, cpu_cycles.len());
        }
        return cpu_cycles;
    }

    fn draw_incoming_image(register_values: &Vec<i32>) {
        let mut draw_buffer: String = "".to_string();
        println!("Display is:");
        for register_x_index in 1..register_values.len() {
            let cursor_position = (register_x_index - 1) % 40;
            let diff: f32 = register_values[register_x_index] as f32 - cursor_position as f32;
            if diff >= -1.0 && diff <= 1.0 {
                draw_buffer.push('#');
            } else {
                draw_buffer.push('.');
            }
            
            if register_x_index % 40 == 0 {
                println!("{}", draw_buffer);
                draw_buffer = "".to_string();
            }
        }
    }

    fn calculate_register_x_value(cpu_cycles: Vec<i32>) -> i32 {
        let mut register_x = 0;
        for cycle_index in 0..cpu_cycles.len() {
            if (cycle_index + 20) % 40 == 0 {
                let update_value = i32::try_from(cycle_index).unwrap() * cpu_cycles.get(cycle_index).unwrap();
                register_x = register_x + update_value;
            }
        }
        return register_x;
    }
}

fn main() {
    let reg_x = cpu_simulator::get_register_value("input1.txt".to_string());
    println!("Register X = {}", reg_x);
}
