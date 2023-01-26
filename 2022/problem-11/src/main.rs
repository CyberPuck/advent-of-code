mod monkey_business {
    use std::fs;

    struct Monkey {
        items: Vec<i32>,
        operation: Operation,
        test_divisor: i32,
        test_true_monkey: usize,
        test_false_monkey: usize,
        inspection_count: u32,
    }

    /// ## NOTE: if `integer` == 0, it refers to the item value.
    enum Operation {
        ADD { integer: i32 },
        MULTIPLE { integer: i32 },
    }

    pub fn get_monkey_business(file_name: String) -> u32 {
        let monkeys: Vec<Monkey> = parse_file(file_name);
        return simulate_monkey_business(monkeys, 20);
    }

    fn parse_file(file_name: String) -> Vec<Monkey> {
        let mut monkeys: Vec<Monkey> = Vec::new();

        let file_string = fs::read_to_string(file_name).unwrap();
        let file_lines: Vec<&str> = file_string.lines().collect();

        let mut monkey_index: usize = 0;

        for line in file_lines {
            // Note three different lines contain `monkey`, only the first line is uppercase
            if line.contains("Monkey") {
                let number_str = line.split_ascii_whitespace().collect::<Vec<&str>>()[1];
                println!("numbers are: {:?}", number_str);
                let number = number_str.split(":").collect::<Vec<&str>>()[0];
                println!("New index is: {}", number);
                monkey_index = number.parse::<usize>().unwrap();
                monkeys.push(Monkey {
                    items: Vec::new(),
                    operation: Operation::ADD { integer: 0 },
                    test_divisor: 0,
                    test_false_monkey: 0,
                    test_true_monkey: 0,
                    inspection_count: 0,
                });
            } else if line.to_ascii_lowercase().contains("items") {
                println!("Parsing items:");
                let items: Vec<&str> = line.split(":").collect();
                println!("Should be vec with 2 entries: {:?}", items);
                let items_list: Vec<&str> =
                    items.get(items.len() - 1).unwrap().split(",").collect();
                println!("Item list: {:?}", items_list);
                for item_str in items_list {
                    println!("Item is: {}", item_str.trim());
                    println!("Item is: {}", item_str.trim().parse::<i32>().unwrap());
                    let item = item_str.trim().parse::<i32>().unwrap();
                    println!("Number is: {}", item);
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
                let test_true_monkey = test_str
                    .get(test_str.len() - 1)
                    .unwrap()
                    .parse::<usize>()
                    .unwrap();
                monkeys[monkey_index].test_true_monkey = test_true_monkey;
            } else {
                println!("The following line was not parsed = {}", line);
            }
        }

        return monkeys;
    }

    fn simulate_monkey_business(mut monkeys: Vec<Monkey>, round_count: u32) -> u32 {
        for _round in 0..round_count {
            for monkey_index in 0..monkeys.len() {
                for item_index in 0..monkeys[monkey_index].items.len() {
                    let item_worry_level = monkeys[monkey_index].items[item_index];
                    let item_worry_increase = match monkeys[monkey_index].operation {
                        Operation::ADD { integer } => item_worry_level + integer,
                        Operation::MULTIPLE { integer } => item_worry_level * integer,
                    };
                    let worry_level: i32 = f32::floor(item_worry_increase as f32 / 3.0) as i32;
                    if worry_level % monkeys[monkey_index].test_divisor == 0 {
                        let item = monkeys[monkey_index].items[item_index];
                        let tossed_monkey_index = monkeys[monkey_index].test_true_monkey;
                        monkeys[tossed_monkey_index].items.push(item);
                    } else {
                        let item = monkeys[monkey_index].items[item_index];
                        let tossed_monkey_index = monkeys[monkey_index].test_false_monkey;
                        monkeys[tossed_monkey_index].items.push(item);
                    }
                    monkeys[monkey_index].inspection_count =
                        monkeys[monkey_index].inspection_count + 1;
                }
            }
        }
        let mut first_monkey_inspector = 0;
        let mut second_monkey_inspector = 0;
        for monkey in monkeys {
            if monkey.inspection_count > first_monkey_inspector {
                first_monkey_inspector = monkey.inspection_count;
            } else if monkey.inspection_count > second_monkey_inspector {
                second_monkey_inspector = monkey.inspection_count;
            }
        }
        println!("first place: {}", first_monkey_inspector);
        println!("second place: {}", second_monkey_inspector);
        return first_monkey_inspector * second_monkey_inspector;
    }
}

fn main() {
    let monkey_business_value = monkey_business::get_monkey_business("sample1.txt".to_string());
    println!("Monkey business = {}", monkey_business_value);
}
