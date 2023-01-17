mod filesystem_parser {
    use std::fs::{self, File};

    /// Really simple tree implementation
    struct FileSystemTree {
        nodes: Vec<FileSystemTree>,
        size: u32,
        name: String,
        is_file: bool,
    }

    /// Functions for working with the tree
    impl FileSystemTree {
        /// Calculate the total size of all directories (is_file == false) <
        /// 100000.
        fn getDirectoriesLessThan100k(&self) -> u32 {
            let total_size = 0;

            return total_size;
        }
    }

    /// Return the calculated tree
    pub fn get_directories_size(file_name: String) -> u32 {
        let fsTree = parse_file(file_name);
        return fsTree.getDirectoriesLessThan100k();
    }

    fn parse_file(file_name: String) -> FileSystemTree {
        let mut fsTree = FileSystemTree {
            nodes: Vec::new(),
            size: 0,
            name: "/".to_string(),
            is_file: false,
        };

        let file = fs::read_to_string(file_name).unwrap();
        let file_lines = file.lines();
        // indicator for when we are parsing an `ls` command output
        let mut inside_ls_flag = false;
        for line in file_lines {
            let contents: Vec<&str> = line.split_ascii_whitespace().collect();
            if line.contains("cd") {
                inside_ls_flag = false;
                // handle change to directory, this has three modes:
                // 1. Moved to a new node
                // 2. Moved back to the old node
                // 3. Moved to the root node
                if contents.get(2).unwrap().contains("..") {
                    // moving to the previous node
                } else if contents.get(2).unwrap().contains("/") {
                    // moving to root node
                } else {
                    // moving to a child node that should already exist
                    let node_name = contents.get(2).unwrap();
                }
            } else if line.contains("ls") {
                // need to read a bunch new lines
                inside_ls_flag = true;
            } else if inside_ls_flag == true {
                // need to add nodes to the current entry
                let mut is_file_flag = true;
                let mut size = 0;
                if contents.get(0).unwrap().contains("dir") {
                    is_file_flag = false;
                } else {
                    // parse file size
                    size = contents.get(0).unwrap().parse::<u32>().unwrap();
                }
                let folder_name = contents.get(1).unwrap().to_string();
                let node = FileSystemTree {
                    nodes: Vec::new(),
                    name: folder_name,
                    size: size,
                    is_file: is_file_flag,
                };
                // TODO: Add node to currently inspected node
            }
        }

        return fsTree;
    }
}

fn main() {
    let total_size = filesystem_parser::get_directories_size("sample1.txt".to_string());
    println!("Total size of all files < 100k is: {}", total_size);
}
