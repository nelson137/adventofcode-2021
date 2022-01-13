use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};

use structopt::StructOpt;

use super::{todays_input, Day};

mod model;
use self::model::Board;

todays_input!(INFILE_PATH);

#[derive(StructOpt)]
pub struct Day4 {
    #[structopt(default_value = &INFILE_PATH)]
    infile: PathBuf,
}

impl Day for Day4 {
    fn part1(&self) -> Result<(), Box<dyn Error>> {
        let (numbers, mut boards) = self.parse_bingo()?;

        let mut maybe_answer = None;

        'draw_num: for num in &numbers {
            for b in &mut boards {
                if b.mark_check(*num) {
                    maybe_answer = Some(b.sum_unmarked() * num);
                    break 'draw_num;
                }
            }
        }

        match maybe_answer {
            Some(answer) => {
                println!("{}", answer);
                Ok(())
            }
            None => Err("no solution found".into()),
        }
    }

    fn part2(&self) -> Result<(), Box<dyn Error>> {
        let (numbers, mut boards) = self.parse_bingo()?;

        let num_boards = boards.len();
        let mut num_wins = 0;
        let mut board_wins = vec![false; num_boards];
        let mut maybe_answer = None;
        'draw_num: for num in &numbers {
            for (i, b) in boards.iter_mut().enumerate() {
                if board_wins[i] {
                    continue;
                }
                if b.mark_check(*num) {
                    board_wins[i] = true;
                    num_wins += 1;
                    if num_wins == num_boards {
                        maybe_answer = Some(b.sum_unmarked() * num);
                        break 'draw_num;
                    }
                }
            }
        }

        match maybe_answer {
            Some(answer) => {
                println!("{}", answer);
                Ok(())
            }
            None => Err("no solution found".into()),
        }
    }
}

impl Day4 {
    fn parse_bingo(&self) -> Result<(Vec<usize>, Vec<Board>), Box<dyn Error>> {
        let file = File::open(&self.infile)?;
        let mut lines = BufReader::new(&file).lines();

        let num_str = match lines.next() {
            Some(num_str_res) => match num_str_res {
                Ok(num_str) => num_str,
                Err(e) => return Err(format!("invalid numbers: {}", e).into()),
            },
            None => return Err("no numbers".into()),
        };

        let maybe_nums: Result<Vec<usize>, _> =
            num_str.split(',').map(str::parse).collect();
        let numbers = match maybe_nums {
            Ok(nums) => nums,
            _ => return Err(format!("invalid numbers: {}", num_str).into()),
        };

        let mut boards: Vec<Board> = Vec::new();
        while let Some(_spacer) = lines.next() {
            let row_strs =
                (&mut lines).take(5).collect::<Result<Vec<_>, _>>()?;
            boards.push(row_strs.try_into()?);
        }

        Ok((numbers, boards))
    }
}
