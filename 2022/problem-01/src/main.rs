//! Problem 1 from AoC 2022
//! 
//! Arch:
//! - Read in a given file from stdin
//! - Parse file with expected inputs
//! - Panic if unexpected input reached
//! - Print out the ELF to go to, with kcal amount
//! 
//! ```rust
//! let file = "example1.txt";
//! let best_elf = kcal_parser::get_elf(file);
//! assertEq!(best_elf.index, 4);
//! assertEq!(best_elf.kcals, 24000);
//! ```
mod kcal_parser {

    //! Represents an Elf, index in array with kcal count

    use core::{fmt, num};
    use std::fs;

    #[derive(Clone, Eq, PartialEq, Ord)]
    pub struct Elf {
        pub index: u32,
        pub kcals: u32
    }

    impl fmt::Display for Elf {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let mut final_output = String::new();
            final_output.push_str("Index: ");
            final_output.push_str(self.index.to_string().as_str());
            final_output.push_str("\nkcals: ");
            final_output.push_str(self.kcals.to_string().as_str());
            final_output.push_str("\n");

            write!(f, "{}", final_output)?;
            Ok(())
        }
    }

    impl PartialOrd for Elf {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            Some(self.kcals.cmp(&other.kcals))
        }
    }

    fn parse_file(file_name: String) -> Vec<Elf> {
        let mut elfs: Vec<Elf> = vec![];
        let mut file = fs::read_to_string(file_name).unwrap();
        // NOTE: Is this a hack to make sure there is an extra newline?  What if there is only one?
        if !file.ends_with("\n\n") {
            file.push_str("\n\n");
        }
        let lines = file.lines();
        //println!("{:?}", lines);
        let mut elf_count = 1;
        let mut total_kcals = 0;
        for line in lines {
            if line.len() == 0 {
                let new_elf = Elf {
                    index: elf_count,
                    kcals: total_kcals
                };
                elfs.push(new_elf);
                elf_count += 1;
                total_kcals = 0;
            } else {
                total_kcals += line.parse::<u32>().unwrap();
            }
        }

        return elfs;
    }

    // Entry function for finding an the top X elfs based on kcal
    pub fn get_elfs(file_name: String, number_elfs: u32) -> Vec<Elf> {
        let mut elfs: Vec<Elf> = parse_file(file_name);
        elfs.sort();
        elfs.reverse();
        println!("testing inputs");
        for elf in &elfs {
            println!("{}", elf);
        }

        return elfs[0..usize::try_from(number_elfs).unwrap()].to_vec();

        /*let mut best_elf = Elf {
            index: 0,
            kcals: 0
        };
        for i in 0..elfs.len() {
            if elfs.get(i).unwrap().kcals > best_elf.kcals {
                best_elf.index = u32::try_from(i).unwrap();
                best_elf.kcals = elfs.get(i).unwrap().kcals;
            }
        }

        return best_elfs;*/
    }
}

fn main() {
    println!("Trying test file");
    let elfs = kcal_parser::get_elfs("input1.txt".to_string(), 3);
    let mut total_kcal = 0;
    for elf in elfs {
        println!("Elf is: {}", elf);
        total_kcal += elf.kcals;
    }
    println!("Total kcal: {}", total_kcal);
}
