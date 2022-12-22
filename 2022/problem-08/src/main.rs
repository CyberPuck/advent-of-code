mod tree_spotter {
    use std::fs;

    #[derive(Debug)]
    struct Forest {
        trees: Vec<Vec<u32>>,
    }

    impl Forest {
        fn is_visible_top_trees(&self, row_index: usize, column_index: usize) -> bool {
            let tree_height = self.trees[row_index][column_index];

            for index in (0..(row_index)).rev() {
                if tree_height <= self.trees[index][column_index] {
                    return false;
                }
            }
            return true;
        }

        fn is_visible_bottom_trees(&self, row_index: usize, column_index: usize) -> bool {
            let tree_height = self.trees[row_index][column_index];

            for index in (row_index + 1)..self.trees.len() {
                if tree_height <= self.trees[index][column_index] {
                    return false;
                }
            }
            return true;
        }

        fn is_visible_left_trees(&self, row_index: usize, column_index: usize) -> bool {
            let tree_height = self.trees[row_index][column_index];

            for index in (0..(column_index)).rev() {
                if tree_height <= self.trees[row_index][index] {
                    return false;
                }
            }
            return true;
        }

        fn is_visible_right_trees(&self, row_index: usize, column_index: usize) -> bool {
            let tree_height = self.trees[row_index][column_index];

            for index in column_index + 1..self.trees[row_index].len() {
                if tree_height <= self.trees[row_index][index] {
                    return false;
                }
            }
            return true;
        }
    }

    #[cfg(test)]
    mod test {
        use super::*;

        #[test]
        fn test_top_tree() {
            let test_tree = Forest {
                trees: vec![vec![9, 9, 3, 3], vec![3, 3, 5, 3], vec![3, 3, 5, 3]],
            };

            let result = test_tree.is_visible_top_trees(1, 1);
            assert!(!result, "Tree should not be visible");

            let result = test_tree.is_visible_top_trees(1, 2);
            assert!(result, "Tree should be visible");
        }

        #[test]
        fn test_bottom_tree() {
            let test_tree = Forest {
                trees: vec![vec![9, 9, 3, 3], vec![3, 3, 5, 3], vec![3, 3, 5, 3]],
            };

            let result = test_tree.is_visible_bottom_trees(1, 2);
            assert!(!result, "Tree should not be visible");

            let result = test_tree.is_visible_bottom_trees(0, 1);
            assert!(result, "Tree should be visible");
        }

        #[test]
        fn test_left_tree() {
            let test_tree = Forest {
                trees: vec![vec![9, 9, 3, 3], vec![3, 3, 5, 3], vec![3, 3, 5, 3]],
            };

            let result = test_tree.is_visible_top_trees(1, 1);
            assert!(!result, "Tree should not be visible");

            let result = test_tree.is_visible_top_trees(1, 2);
            assert!(result, "Tree should be visible");
        }

        #[test]
        fn test_right_tree() {
            let test_tree = Forest {
                trees: vec![vec![9, 9, 3, 3], vec![3, 3, 6, 3], vec![3, 3, 5, 3]],
            };

            let result = test_tree.is_visible_right_trees(0, 2);
            assert!(!result, "Tree should not be visible");

            let result = test_tree.is_visible_right_trees(1, 2);
            assert!(result, "Tree should be visible");
        }

        #[test]
        fn test_invisible_tree() {
            let test_tree = Forest {
                trees: vec![
                    vec![3, 0, 3, 7, 3],
                    vec![2, 5, 5, 1, 2],
                    vec![6, 5, 3, 3, 2],
                    vec![3, 3, 5, 4, 9],
                    vec![3, 5, 3, 9, 0],
                ],
            };

            let result1 = test_tree.is_visible_top_trees(3, 1);
            let result2 = test_tree.is_visible_bottom_trees(3, 1);
            let result3 = test_tree.is_visible_left_trees(3, 1);
            let result4 = test_tree.is_visible_right_trees(3, 1);

            assert!(!result1, "Top broken");
            assert!(!result2, "Bottom broken");
            assert!(!result3, "Left broken");
            assert!(!result4, "Right broken");
        }
    }

    pub fn get_visible_tree_count(file_name: String) -> u32 {
        let forest = parse_file(file_name);
        let number_of_visible_trees = count_visible_trees(forest);

        return number_of_visible_trees;
    }

    fn parse_file(file_name: String) -> Forest {
        let file_data = fs::read_to_string(file_name).unwrap();
        let file_lines = file_data.lines();

        let mut forest = Forest { trees: vec![] };

        for line in file_lines {
            let mut row: Vec<u32> = vec![];
            for tree in line.chars() {
                row.push(tree.to_digit(10).unwrap());
            }
            forest.trees.push(row);
        }

        return forest;
    }

    fn count_visible_trees(forest: Forest) -> u32 {
        // start count with the parimeter trees (they are always visible)
        // minus 4 because corner trees are counted twice
        let mut count = forest.trees[0].len() as u32 * 2 + forest.trees.len() as u32 * 2 - 4;

        for row_index in 1..forest.trees.len() - 1 {
            for column_index in 1..forest.trees[row_index].len() - 1 {
                if forest.is_visible_bottom_trees(row_index, column_index)
                    || forest.is_visible_left_trees(row_index, column_index)
                    || forest.is_visible_right_trees(row_index, column_index)
                    || forest.is_visible_top_trees(row_index, column_index)
                {
                    count += 1;
                }
            }
        }
        return count;
    }
}

fn main() {
    let count = tree_spotter::get_visible_tree_count("input1.txt".to_string());
    println!("Trees visible are: {}", count);
}
