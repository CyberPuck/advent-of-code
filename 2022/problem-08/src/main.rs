mod tree_spotter {
    use std::fs;

    #[derive(Debug)]
    struct Forest {
        trees: Vec<Vec<u32>>,
    }

    impl Forest {
        fn is_visible_top_trees(&self, row_index: usize, column_index: usize) -> (bool, u32) {
            let tree_height = self.trees[row_index][column_index];

            for index in (0..(row_index)).rev() {
                if tree_height <= self.trees[index][column_index] {
                    return (false, row_index as u32 - index as u32);
                }
            }
            return (true, row_index as u32);
        }

        fn is_visible_bottom_trees(&self, row_index: usize, column_index: usize) -> (bool, u32) {
            let tree_height = self.trees[row_index][column_index];

            for index in (row_index + 1)..self.trees.len() {
                if tree_height <= self.trees[index][column_index] {
                    return (false, index as u32 - row_index as u32);
                }
            }
            return (true, self.trees.len() as u32 - row_index as u32 - 1);
        }

        fn is_visible_left_trees(&self, row_index: usize, column_index: usize) -> (bool, u32) {
            let tree_height = self.trees[row_index][column_index];

            for index in (0..(column_index)).rev() {
                if tree_height <= self.trees[row_index][index] {
                    return (false, column_index as u32 - index as u32);
                }
            }
            return (true, column_index as u32);
        }

        fn is_visible_right_trees(&self, row_index: usize, column_index: usize) -> (bool, u32) {
            let tree_height = self.trees[row_index][column_index];

            for index in column_index + 1..self.trees[row_index].len() {
                if tree_height <= self.trees[row_index][index] {
                    return (false, index as u32 - column_index as u32);
                }
            }
            return (
                true,
                self.trees[row_index].len() as u32 - column_index as u32 - 1,
            );
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
            assert!(!result.0, "Tree should not be visible");
            assert_eq!(result.1, 1, "Distance is wrong");

            let result = test_tree.is_visible_top_trees(1, 2);
            assert!(result.0, "Tree should be visible");
            assert_eq!(result.1, 1, "Distance is wrong");
        }

        #[test]
        fn test_bottom_tree() {
            let test_tree = Forest {
                trees: vec![vec![9, 9, 3, 3], vec![3, 3, 5, 3], vec![3, 3, 5, 3]],
            };

            let result = test_tree.is_visible_bottom_trees(1, 2);
            assert!(!result.0, "Tree should not be visible");
            assert_eq!(result.1, 1, "Distance is wrong");

            let result = test_tree.is_visible_bottom_trees(0, 1);
            assert!(result.0, "Tree should be visible");
            assert_eq!(result.1, 2, "Distance is wrong");
        }

        #[test]
        fn test_left_tree() {
            let test_tree = Forest {
                trees: vec![vec![9, 9, 3, 3], vec![3, 3, 5, 3], vec![3, 3, 5, 3]],
            };

            let result = test_tree.is_visible_left_trees(1, 1);
            assert!(!result.0, "Tree should not be visible");
            assert_eq!(result.1, 1, "Distance is wrong");

            let result = test_tree.is_visible_left_trees(1, 2);
            assert!(result.0, "Tree should be visible");
            assert_eq!(result.1, 2, "Distance is wrong");
        }

        #[test]
        fn test_right_tree() {
            let test_tree = Forest {
                trees: vec![vec![9, 9, 3, 3], vec![3, 3, 6, 3], vec![3, 3, 5, 3]],
            };

            let result = test_tree.is_visible_right_trees(0, 2);
            assert!(!result.0, "Tree should not be visible");
            assert_eq!(result.1, 1, "Distance is wrong");

            let result = test_tree.is_visible_right_trees(1, 2);
            assert!(result.0, "Tree should be visible");
            assert_eq!(result.1, 1, "Distance is wrong");
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

            assert!(!result1.0, "Top broken");
            assert_eq!(result1.1, 1, "Distance is wrong");
            assert!(!result2.0, "Bottom broken");
            assert_eq!(result2.1, 1, "Distance is wrong");
            assert!(!result3.0, "Left broken");
            assert_eq!(result3.1, 1, "Distance is wrong");
            assert!(!result4.0, "Right broken");
            assert_eq!(result4.1, 1, "Distance is wrong");
        }
    }

    /// Get stats for trees in a forest, two values returned.
    /// # Input
    /// file_name: String, input file representing the forest
    /// # Return
    /// (visible_tree_count: u32, highest_scenic_score: u32)
    pub fn get_tree_stats(file_name: String) -> (u32, u32) {
        let forest = parse_file(file_name);
        let tree_stats = count_visible_trees_stats(forest);

        return tree_stats;
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

    /// Get the count of trees that are visible and the highest scenic score for
    /// a all trees in the forest.
    /// # Returns
    /// (visible_tree_count: u32, highest_scenic_score: u32)
    fn count_visible_trees_stats(forest: Forest) -> (u32, u32) {
        // start count with the parimeter trees (they are always visible)
        // minus 4 because corner trees are counted twice
        let mut count = forest.trees[0].len() as u32 * 2 + forest.trees.len() as u32 * 2 - 4;
        let mut highest_scenic_score: u32 = 0;

        for row_index in 1..forest.trees.len() - 1 {
            for column_index in 1..forest.trees[row_index].len() - 1 {
                let top_stats = forest.is_visible_top_trees(row_index, column_index);
                let bottom_stats = forest.is_visible_bottom_trees(row_index, column_index);
                let left_stats = forest.is_visible_left_trees(row_index, column_index);
                let right_stats = forest.is_visible_right_trees(row_index, column_index);
                if bottom_stats.0 || left_stats.0 || top_stats.0 || right_stats.0 {
                    count += 1;
                }
                if top_stats.1 * bottom_stats.1 * right_stats.1 * left_stats.1
                    > highest_scenic_score
                {
                    highest_scenic_score =
                        top_stats.1 * bottom_stats.1 * right_stats.1 * left_stats.1;
                }
            }
        }
        return (count, highest_scenic_score);
    }
}

fn main() {
    let stats = tree_spotter::get_tree_stats("input1.txt".to_string());
    println!("Trees visible are: {}", stats.0);
    println!("Highest score: {}", stats.1);
}
