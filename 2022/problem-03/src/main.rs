pub mod pack_sorter {
    use std::fs;

    #[derive(Debug, Clone)]
    struct Pack {
        compartment_one: String,
        compartment_two: String,
    }

    #[derive(Debug)]
    struct ElfGroup {
        one: Pack,
        two: Pack,
        three: Pack,
    }

    /// Gets the priorities of both individual items and group badges.
    /// Returns: (total_priority, badge_total_priority).
    pub fn get_priority_sum(file_name: String) -> (u32, u32) {
        let mut total_priority = 0;
        let mut total_badge_priority = 0;
        let packs = parse_file(file_name);
        let groups = generate_groups(&packs);
        // Part one calc
        for pack in packs {
            println!("{:?}", pack);
            let output = check_for_duplicates(pack);
            println!("{:?}", output);
            if output.is_some() {
                let priority_char = output.unwrap();
                println!("Ascii #: {}", priority_char as u32);
                let priority = convert_char_to_priority(priority_char);
                println!("{:?}", priority);
                total_priority += priority;
            }
        }
        // Part two calc
        for group in groups {
            let badge = get_group_badge(group);
            let badge_priority = convert_char_to_priority(badge.unwrap());
            total_badge_priority += badge_priority;
        }
        return (total_priority, total_badge_priority);
    }

    fn parse_file(file_name: String) -> Vec<Pack> {
        let mut packs: Vec<Pack> = vec![];

        let file_data = fs::read_to_string(file_name).unwrap();
        let file_lines = file_data.lines();
        let mut pack = Pack {
            compartment_one: "".to_string(),
            compartment_two: "".to_string(),
        };
        for line in file_lines {
            let line = line.to_string();
            let compartments = line.split_at(line.len() / 2);
            let pack = Pack {
                compartment_one: compartments.0.to_string(),
                compartment_two: compartments.1.to_string(),
            };
            packs.push(pack);
        }
        return packs;
    }

    /// Check for duplicates in the given pack (compare compartments one and two).
    /// Will return an Option<char> representing the priority duplicate.
    /// If no duplicate is found, return `None`.
    /// If there is, return Some(char).
    fn check_for_duplicates(pack: Pack) -> Option<char> {
        for item in pack.compartment_one.chars() {
            if pack.compartment_two.contains(item) {
                return Some(item);
            }
        }
        return None;
    }

    /// Converts a given char, convert to a priority.
    ///
    /// Priority is the following:
    /// 1. Items ascii `a` through `z` are 1 - 26
    /// 2. Items ascii `A` through `Z` are 27 - 52
    fn convert_char_to_priority(priority_char: char) -> u32 {
        let ascii_value = priority_char as u32;
        if ascii_value > 96 {
            return ascii_value - 96;
        }
        // 65 is ascii `A`, need to drop to starting priority of 27, 65 - 27 = 38
        return ascii_value - 38;
    }

    fn generate_groups(packs: &Vec<Pack>) -> Vec<ElfGroup> {
        let mut elfGroups: Vec<ElfGroup> = vec![];
        let mut iter = packs.iter();
        for _n in 0..packs.len() / 3 {
            let group = ElfGroup {
                one: iter.next().unwrap().clone(),
                two: iter.next().unwrap().clone(),
                three: iter.next().unwrap().clone(),
            };
            elfGroups.push(group);
        }

        return elfGroups;
    }

    fn get_group_badge(group: ElfGroup) -> Option<char> {
        let badge_priority = 0;
        for item in group.one.compartment_one.chars() {
            if (group.two.compartment_one.contains(item)
                || group.two.compartment_two.contains(item))
                && (group.three.compartment_one.contains(item)
                    || group.three.compartment_two.contains(item))
            {
                return Some(item);
            }
        }

        for item in group.one.compartment_two.chars() {
            if (group.two.compartment_one.contains(item)
                || group.two.compartment_two.contains(item))
                && (group.three.compartment_one.contains(item)
                    || group.three.compartment_two.contains(item))
            {
                return Some(item);
            }
        }

        return None;
    }
}

fn main() {
    let priority = pack_sorter::get_priority_sum("input1.txt".to_string());
    println!("Priority is: {}", priority.0);
    println!("Badge priority is {}", priority.1);
}
