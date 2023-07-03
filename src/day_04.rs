use std::str::FromStr;

use crate::utils::get_lines;

pub fn answer_part_1() -> u32 {
    let mut sum = 0;
    for line in get_lines("input/day_04.txt").map(Result::unwrap) {
        let rp = line.parse::<RangePair>().expect("parses range pair");
        if rp.0.contains(&rp.1) || rp.1.contains(&rp.0) {
            sum += 1;
        }
    }
    sum
}

pub fn answer_part_2() -> u32 {
    let mut sum = 0;
    for line in get_lines("input/day_04.txt").map(Result::unwrap) {
        let rp = line.parse::<RangePair>().expect("parses range pair");
        if rp.0.overlaps(&rp.1) {
            sum += 1;
        }
    }
    sum
}

struct RangePair(SectionRange, SectionRange);

impl FromStr for RangePair {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split(',');
        Ok(Self(
            split
                .next()
                .ok_or_else(|| format!("did not contain first of pair"))?
                .parse()
                .map_err(|e| format!("unparseable first of pair: {e}"))?,
            split
                .next()
                .ok_or_else(|| format!("did not contain second of pair"))?
                .parse()
                .map_err(|e| format!("unparseable second of pair: {e}"))?,
        ))
    }
}

struct SectionRange {
    start: usize,
    end: usize,
}

impl SectionRange {
    fn contains(&self, other: &Self) -> bool {
        self.start <= other.start && self.end >= other.end
    }

    fn overlaps(&self, other: &Self) -> bool {
        self.start <= other.end && self.end >= other.start
    }
}

impl FromStr for SectionRange {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split('-');
        Ok(Self {
            start: split
                .next()
                .ok_or_else(|| format!("unable to extract start"))?
                .parse()
                .map_err(|e| format!("unparseable start: {e}"))?,
            end: split
                .next()
                .ok_or_else(|| format!("unable to extract end"))?
                .parse()
                .map_err(|e| format!("unparseable end: {e}"))?,
        })
    }
}
