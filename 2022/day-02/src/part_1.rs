use super::*;

struct Round {
    player1_move: Move,
    player2_move: Move,
}

impl Round {
    fn score_player_2(&self) -> u32 {
        let game_score = match (&self.player1_move, &self.player2_move) {
            // win
            (Move::Rock, Move::Paper) => 6,
            (Move::Paper, Move::Scissors) => 6,
            (Move::Scissors, Move::Rock) => 6,
            // lose
            (Move::Paper, Move::Rock) => 0,
            (Move::Scissors, Move::Paper) => 0,
            (Move::Rock, Move::Scissors) => 0,
            // draw
            _ => 3,
        };

        game_score + self.player2_move.score()
    }
}

impl FromStr for Round {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split(" ");
        let p1_move = iter.next().ok_or("missing player 1 move")?;
        let p2_move = iter.next().ok_or("missing player 2 move")?;

        let player1_move = match p1_move {
            "A" => Move::Rock,
            "B" => Move::Paper,
            "C" => Move::Scissors,
            _ => return Err("invalid move for player 1"),
        };
        let player2_move = match p2_move {
            "X" => Move::Rock,
            "Y" => Move::Paper,
            "Z" => Move::Scissors,
            _ => return Err("invalid move for player 2"),
        };

        Ok(Round {
            player1_move,
            player2_move,
        })
    }
}

pub fn compute(input: &str) -> String {
    let score: u32 = input
        .lines()
        .map(|line| {
            line.parse::<Round>()
                .expect("invalid round format found")
                .score_player_2()
        })
        .sum();
    score.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_works() {
        let input = "A Y
B X
C Z";
        assert_eq!(compute(input), "15")
    }
}
