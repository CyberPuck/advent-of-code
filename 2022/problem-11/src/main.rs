mod monkey_business {
    use std::fs;

    #[derive(Debug)]
    struct Monkey {
        items: Vec<u128>,
        operation: Operation,
        test_divisor: i32,
        total_test_divisor: u32,
        test_true_monkey: usize,
        test_false_monkey: usize,
        inspection_count: u32,
    }

    /// ## NOTE: if `integer` == 0, it refers to the item value.
    #[derive(Debug)]
    enum Operation {
        ADD { integer: i32 },
        MULTIPLE { integer: i32 },
    }

    pub fn get_monkey_business(file_name: String) -> u128 {
        let monkeys: Vec<Monkey> = parse_file(file_name);
        let part2 = true;
        if !part2 {
            return simulate_monkey_business(monkeys, 20, part2);
        } else {
            return simulate_monkey_business(monkeys, 10000, part2);
        }
    }

    fn parse_file(file_name: String) -> Vec<Monkey> {
        let mut monkeys: Vec<Monkey> = Vec::new();

        let file_string = fs::read_to_string(file_name).unwrap();
        let file_lines: Vec<&str> = file_string.lines().collect();

        let mut monkey_index: usize = 0;

        // part 2 variable
        let mut total_test_divisor: u32 = 1;

        for line in file_lines {
            // Note three different lines contain `monkey`, only the first line is uppercase
            if line.contains("Monkey") {
                let number_str = line.split_ascii_whitespace().collect::<Vec<&str>>()[1];
                let number = number_str.split(":").collect::<Vec<&str>>()[0];
                monkey_index = number.parse::<usize>().unwrap();
                monkeys.push(Monkey {
                    items: Vec::new(),
                    operation: Operation::ADD { integer: 0 },
                    test_divisor: 0,
                    total_test_divisor: 0,
                    test_false_monkey: 0,
                    test_true_monkey: 0,
                    inspection_count: 0,
                });
            } else if line.to_ascii_lowercase().contains("items") {
                let items: Vec<&str> = line.split(":").collect();
                let items_list: Vec<&str> =
                    items.get(items.len() - 1).unwrap().split(",").collect();
                for item_str in items_list {
                    let item = item_str.trim().parse::<u128>().unwrap();
                    monkeys[monkey_index].items.push(item);
                }
            } else if line.to_ascii_lowercase().contains("operation") {
                let components: Vec<&str> = line.split_ascii_whitespace().collect();
                let operation_str = components.get(components.len() - 2).unwrap();
                let integer_str = components.get(components.len() - 1).unwrap();
                let integer = if integer_str.to_ascii_lowercase().contains("old") {
                    0
                } else {
                    integer_str.parse::<i32>().unwrap()
                };
                let operation = if operation_str.to_ascii_lowercase().contains("+") {
                    Operation::ADD { integer: integer }
                } else if operation_str.to_ascii_lowercase().contains("*") {
                    Operation::MULTIPLE { integer: integer }
                } else {
                    panic!("Should have parsed the correct command = {}", line)
                };
                // push operation update
                monkeys[monkey_index].operation = operation;
            } else if line.to_ascii_lowercase().contains("test") {
                let test_str = line.split_ascii_whitespace().collect::<Vec<&str>>();
                let test_divisor = test_str
                    .get(test_str.len() - 1)
                    .unwrap()
                    .parse::<i32>()
                    .unwrap();
                // push test_divisor update
                monkeys[monkey_index].test_divisor = test_divisor;
                total_test_divisor = total_test_divisor * test_divisor as u32;
            } else if line.to_ascii_lowercase().contains("true") {
                let test_str = line.split_ascii_whitespace().collect::<Vec<&str>>();
                let test_true_monkey = test_str
                    .get(test_str.len() - 1)
                    .unwrap()
                    .parse::<usize>()
                    .unwrap();
                monkeys[monkey_index].test_true_monkey = test_true_monkey;
            } else if line.to_ascii_lowercase().contains("false") {
                let test_str = line.split_ascii_whitespace().collect::<Vec<&str>>();
                let test_false_monkey = test_str
                    .get(test_str.len() - 1)
                    .unwrap()
                    .parse::<usize>()
                    .unwrap();
                monkeys[monkey_index].test_false_monkey = test_false_monkey;
            } else if !line.is_empty() {
                println!("The following line was not parsed = {}", line);
            }
        }

        for monkey_index in 0..monkeys.len() {
            monkeys[monkey_index].total_test_divisor = total_test_divisor;
        }

        return monkeys;
    }

    fn simulate_monkey_business(mut monkeys: Vec<Monkey>, round_count: u32, part2: bool) -> u128 {
        for _round in 0..round_count {
            for monkey_index in 0..monkeys.len() {
                for item_index in 0..monkeys[monkey_index].items.len() {
                    let item_worry_level = monkeys[monkey_index].items[item_index];
                    let item_worry_increase = match monkeys[monkey_index].operation {
                        Operation::ADD { integer } => item_worry_level + integer as u128,
                        Operation::MULTIPLE { integer } => {
                            if integer == 0 {
                                item_worry_level * item_worry_level
                            } else {
                                item_worry_level * integer as u128
                            }
                        }
                    };
                    let worry_level: u128 = if !part2 {
                        f64::floor(item_worry_increase as f64 / 3.0) as u128
                    } else {
                        item_worry_increase % monkeys[monkey_index].total_test_divisor as u128
                    };
                    if worry_level % monkeys[monkey_index].test_divisor as u128 == 0 {
                        //let item = monkeys[monkey_index].items[item_index];
                        let tossed_monkey_index = monkeys[monkey_index].test_true_monkey;
                        monkeys[tossed_monkey_index].items.push(worry_level);
                    } else {
                        //let item = monkeys[monkey_index].items[item_index];
                        let tossed_monkey_index = monkeys[monkey_index].test_false_monkey;
                        monkeys[tossed_monkey_index].items.push(worry_level);
                    }
                    monkeys[monkey_index].inspection_count =
                        monkeys[monkey_index].inspection_count + 1;
                }
                // clear out the items in the monkey (no longer there)
                monkeys[monkey_index].items.clear();
            }
            if _round + 1 == 1 || _round + 1 == 20 || (_round + 1) % 1000 == 0 {
                println!("== After round {} ==", _round);
                for index in 0..monkeys.len() {
                    println!(
                        "Monkey {} inspected items {} times.",
                        index, monkeys[index].inspection_count
                    );
                }
                println!("++++ Monkeys ++++\n{:?}", monkeys);
            }
        }
        println!("End monkeys = {:?}", monkeys);
        let mut first_monkey_inspector: u128 = 0;
        let mut second_monkey_inspector: u128 = 0;
        for monkey in monkeys {
            if monkey.inspection_count as u128 > first_monkey_inspector {
                // Make sure to move current first place to second
                second_monkey_inspector = first_monkey_inspector;
                first_monkey_inspector = monkey.inspection_count as u128;
            } else if monkey.inspection_count as u128 > second_monkey_inspector {
                second_monkey_inspector = monkey.inspection_count as u128;
            }
        }
        println!("first place: {}", first_monkey_inspector);
        println!("second place: {}", second_monkey_inspector);
        return first_monkey_inspector * second_monkey_inspector;
    }
}

fn main() {
    let monkey_business_value = monkey_business::get_monkey_business("input1.txt".to_string());
    println!("Monkey business = {}", monkey_business_value);
}
