//! Arch for AoC problem 02
//! - Parse file
//! - Calculate each round
//! - Calculate final score
//! - Return score

pub mod rock_paper_scissors {
    use std::fs;

    #[derive(Eq, PartialEq, Clone, Copy)]
    pub enum Moves {
        Rock,
        Paper,
        Scissors,
    }

    pub struct Round {
        opponent_move: Moves,
        your_move: Moves,
    }

    /// `run_real_strat` is a flag for running part 2
    pub fn get_final_score(file_name: String, run_real_strat: bool) -> u32 {
        let mut final_score = 0;
        let rounds = parse_file(file_name);
        for round in rounds {
            final_score += calculate_round_results(round);
        }

        return final_score;
    }

    fn parse_file(file_name: String) -> Vec<Round> {
        let mut rounds: Vec<Round> = vec![];
        let file_lines = fs::read_to_string(file_name).unwrap();
        let lines = file_lines.lines();
        for line in lines {
            let mut round = Round {
                opponent_move: Moves::Rock,
                your_move: Moves::Rock,
            };
            let moves: Vec<&str> = line.split(' ').collect();
            if moves.len() != 2 {
                panic!(
                    "Moves file is improperly formatted, got {} moves",
                    moves.len()
                );
            }
            // Gross?  Is there a better way to parse this data?
            match moves.get(0).unwrap() {
                &"A" => round.opponent_move = Moves::Rock,
                &"B" => round.opponent_move = Moves::Paper,
                &"C" => round.opponent_move = Moves::Scissors,
                _ => panic!("Unknown opp move {}", moves.get(0).unwrap()),
            }

            match moves.get(1).unwrap() {
                &"X" => round.your_move = Moves::Rock,
                &"Y" => round.your_move = Moves::Paper,
                &"Z" => round.your_move = Moves::Scissors,
                _ => panic!("Unknown your move {}", moves.get(1).unwrap()),
            }

            rounds.push(round);
        }
        return rounds;
    }

    fn get_move_points(given_move: Moves) -> u32 {
        return match given_move {
            Moves::Rock => 1,
            Moves::Paper => 2,
            Moves::Scissors => 3,
        };
    }

    fn calculate_round_results(round: Round) -> u32 {
        let your_move_points = get_move_points(round.your_move);
        let round_points = if round.opponent_move == round.your_move {
            3
        } else {
            if round.opponent_move == Moves::Rock && round.your_move == Moves::Paper
                || round.opponent_move == Moves::Paper && round.your_move == Moves::Scissors
                || round.opponent_move == Moves::Scissors && round.your_move == Moves::Rock
            {
                6
            } else {
                0
            }
        };

        return your_move_points + round_points;
    }
}

fn main() {
    println!("Running strat");
    let score = rock_paper_scissors::get_final_score("input2.txt".to_string(), true);
    println!("Score is: {}", score);
}
