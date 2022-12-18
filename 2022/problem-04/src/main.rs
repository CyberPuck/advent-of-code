pub mod elf_cleanup {
    use std::fs;

    pub struct ElfPair {
        one: CleanRange,
        two: CleanRange,
    }

    pub struct CleanRange {
        start: u32,
        end: u32,
    }

    pub fn find_overlapping_pairs(file_name: String) -> u32 {
        let mut overlapping_pairs = 0;
        let pairs = parse_file(file_name);

        for pair in pairs {
            if pair.one.start <= pair.two.start && pair.one.end >= pair.two.end {
                overlapping_pairs += 1;
            } else if pair.two.start <= pair.one.start && pair.two.end >= pair.one.end {
                overlapping_pairs += 1;
            }
        }
        return overlapping_pairs;
    }

    fn parse_file(file_name: String) -> Vec<ElfPair> {
        let mut elf_pairs: Vec<ElfPair> = vec![];
        let file_data = fs::read_to_string(file_name).unwrap();
        let file_lines = file_data.lines();
        for line in file_lines {
            let elf_pairs_vec: Vec<&str> = line.split(",").collect();
            if elf_pairs_vec.len() != 2 {
                panic!(
                    "Line read in does not have two pairs: {}",
                    elf_pairs_vec.len()
                );
            }
            let pair_one = elf_pairs_vec.get(0).unwrap().to_string();
            let pair_two = elf_pairs_vec.get(1).unwrap().to_string();
            let ranges: Vec<&str> = pair_one.split("-").collect();
            let range_one = CleanRange {
                start: ranges.get(0).unwrap().to_string().trim().parse().unwrap(),
                end: ranges.get(1).unwrap().to_string().trim().parse().unwrap(),
            };
            let ranges: Vec<&str> = pair_two.split("-").collect();
            let range_two = CleanRange {
                start: ranges.get(0).unwrap().to_string().trim().parse().unwrap(),
                end: ranges.get(1).unwrap().to_string().trim().parse().unwrap(),
            };
            elf_pairs.push(ElfPair {
                one: range_one,
                two: range_two,
            });
        }
        return elf_pairs;
    }
}

fn main() {
    let total_pairs = elf_cleanup::find_overlapping_pairs("input1.txt".to_string());
    println!("Pairs: {}", total_pairs);
}
