use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;

use std::collections::HashMap;

type Edge = (String, String);

#[derive(Debug, PartialEq, Clone)]
enum CaveSize {
    Small,
    Big,
}

#[derive(Debug)]
struct CaveSystem {
    edges: HashMap<String, Vec<String>>,
}

impl CaveSystem {
    pub fn new(input: &[Edge]) -> Self {
        let mut edges = HashMap::new();

        for edge in input {
            // connections are bidirectional, so we create two
            // entries per input edge
            edges
                .entry(edge.0.to_owned())
                .or_insert_with(Vec::new)
                .push(edge.1.to_owned());
            edges
                .entry(edge.1.to_owned())
                .or_insert_with(Vec::new)
                .push(edge.0.to_owned());
        }

        Self { edges }
    }

    pub fn search_paths_v1(&self, start: &str, end: &str, partial: &[String]) -> Vec<Vec<String>> {
        // add current node into the partial route
        let new_partial: Vec<String> = vec![partial.to_owned(), vec![start.to_owned()]]
            .into_iter()
            .flatten()
            .collect();

        // terminal condition -> arrived at destination
        if start == end {
            return vec![new_partial];
        }

        let candidates = self.edges[start].iter().filter(|cave| {
            Self::cave_size_for(cave) == CaveSize::Big || !new_partial.contains(cave)
        });

        candidates
            .map(|cave| self.search_paths_v1(cave, end, &new_partial))
            .flatten()
            .collect()
    }

    pub fn search_paths_v2(
        &self,
        start: &str,
        end: &str,
        partial: &[String],
        visited: &str,
    ) -> Vec<Vec<String>> {
        // add current node into the partial route
        let new_partial: Vec<String> = vec![partial.to_owned(), vec![start.to_owned()]]
            .into_iter()
            .flatten()
            .collect();

        // terminal condition -> arrived at destination
        if start == end {
            return vec![new_partial];
        }

        let candidates = self.edges[start].iter().filter(|cave| {
            if *cave == "start" {
                return false;
            }
            if Self::cave_size_for(cave) == CaveSize::Big {
                return true;
            }

            let n_visits = new_partial.iter().filter(|x| x == cave).count();

            if visited == *cave {
                n_visits < 2
            } else {
                n_visits == 0
            }
        });

        let mut paths: Vec<Vec<String>> = candidates
            .map(|cave| {
                if visited.is_empty() && Self::cave_size_for(cave) == CaveSize::Small {
                    [
                        self.search_paths_v2(cave, end, &new_partial, cave),
                        self.search_paths_v2(cave, end, &new_partial, ""),
                    ]
                    .concat()
                } else {
                    [self.search_paths_v2(cave, end, &new_partial, visited)].concat()
                }
            })
            .flatten()
            .collect();

        // remove duplicates
        paths.sort();
        paths.dedup();

        paths
    }

    fn cave_size_for(name: &str) -> CaveSize {
        if name.to_uppercase() == name {
            CaveSize::Big
        } else {
            CaveSize::Small
        }
    }
}

#[aoc_generator(day12)]
pub fn parse_input(input: &str) -> Vec<Edge> {
    input
        .lines()
        .map(|edge| {
            let mut chunks = edge.split('-');
            (
                chunks.next().unwrap().to_string(),
                chunks.next().unwrap().to_string(),
            )
        })
        .collect()
}

#[aoc(day12, part1)]
pub fn solve_part1(input: &[Edge]) -> u64 {
    let caves = CaveSystem::new(input);
    let paths = caves.search_paths_v1("start", "end", &[]);

    paths.len() as u64
}

#[aoc(day12, part2)]
pub fn solve_part2(input: &[Edge]) -> u64 {
    let caves = CaveSystem::new(input);
    let paths = caves.search_paths_v2("start", "end", &[], "");

    paths.len() as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: [&str; 3] = [
        "start-A\nstart-b\nA-c\nA-b\nb-d\nA-end\nb-end",
        "dc-end\nHN-start\nstart-kj\ndc-start\ndc-HN\nLN-dc\nHN-end\nkj-sa\nkj-HN\nkj-dc",
        "fs-end\nhe-DX\nfs-he\nstart-DX\npj-DX\nend-zg\nzg-sl\nzg-pj\npj-he\nRW-he\nfs-DX\npj-RW\nzg-RW\nstart-pj\nhe-WI\nzg-he\npj-fs\nstart-RW",
    ];

    #[test]
    fn test_day12_solve_part1() {
        assert_eq!(solve_part1(&parse_input(INPUT[0])), 10);
        assert_eq!(solve_part1(&parse_input(INPUT[1])), 19);
        assert_eq!(solve_part1(&parse_input(INPUT[2])), 226);
    }

    #[test]
    fn test_day12_solve_part2() {
        assert_eq!(solve_part2(&parse_input(INPUT[0])), 36);
        assert_eq!(solve_part2(&parse_input(INPUT[1])), 103);
        assert_eq!(solve_part2(&parse_input(INPUT[2])), 3509);
    }
}
