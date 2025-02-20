use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};

use structopt::StructOpt;

use super::{todays_input, Day, PartResult, ANSWER};

mod model;
use self::model::{CaveGraph, Node};

todays_input!(INFILE_PATH);

#[derive(StructOpt)]
pub struct Day12 {
    #[structopt(default_value = &INFILE_PATH)]
    infile: PathBuf,
}

impl Day for Day12 {
    fn part1(&self) -> PartResult {
        let cave = self.parse_cave_graph()?;
        let answer = cave.find_all_paths_with(|visits, _, n| match visits[n] {
            Some(v) if v >= 1 => true,
            _ => false,
        });
        ANSWER!(answer)
    }

    fn part2(&self) -> PartResult {
        let cave = self.parse_cave_graph()?;
        let answer =
            cave.find_all_paths_with(|visits, small_double_visit, n| {
                match visits[n] {
                    Some(v) if v >= 1 && small_double_visit => true,
                    _ => false,
                }
            });
        ANSWER!(answer)
    }
}

impl Day12 {
    fn parse_cave_graph(&self) -> Result<CaveGraph, Box<dyn Error>> {
        let file = BufReader::new(File::open(&self.infile)?);

        let mut edges = Vec::<(Node, Node)>::new();

        for line_res in file.lines() {
            let line = line_res
                .map_err(|e| format!("invalid line: {}", e).to_string())?;

            let mut nodes = line.split('-').map(str::trim);

            match (
                nodes.next().map(TryInto::try_into),
                nodes.next().map(TryInto::try_into),
                nodes.next(),
            ) {
                (Some(Ok(a)), Some(Ok(b)), None) => edges.push((a, b)),
                _ => return Err(format!("invalid entry: {}", line).into()),
            }
        }

        // TODO: ensure edges contains 1 start and 1 end

        Ok(CaveGraph::from_edges(edges))
    }
}
