/// Parse a `u32` from the start of the input string.
/// (this is copied/pasted from earlier in the blog post!)
//pub fn parse_numbers(input: &str) -> IResult<&str, u32> {
//    map_res(digit1, u32::from_str)(input)
//}

use nom::{
    character::complete::{digit1, char},
    combinator::map,
    combinator::map_res,
    sequence::separated_pair,
    IResult,
};

/// A point in 2D space
//#[derive(Debug, Eq, PartialEq)]


pub fn parse_numbers(input: &str) -> IResult<&str, u32> {
    map_res(digit1, str::parse)(input)
}

pub struct SectionGroup {
    pub start: u32,
    pub end: u32,
}

pub struct TwoSectionsGroups {
    pub section1: SectionGroup,
    pub section2: SectionGroup,
}

impl SectionGroup {
    fn within(&self, other: &SectionGroup) -> bool {
      return other.start >= self.start && other.end <= self.end 
    }

    fn from_line(line: &str) -> IResult<&str, Self> {
        let parse_section = separated_pair(parse_numbers, char('-'), parse_numbers);
        map(parse_section, |(start, end)| SectionGroup{start, end})(line)
    }
}

impl TwoSectionsGroups {
    fn within(&self) -> bool {
        return self.section2.within(&self.section1) || self.section1.within(&self.section2)  
    }

    fn from_line(line: &str) -> IResult<&str, Self> {
        let parse_two_sections = separated_pair(SectionGroup::from_line, char(','), SectionGroup::from_line);
        map(parse_two_sections, |(section1, section2)| TwoSectionsGroups { section1, section2 })(line)
    }
}





fn solve(input: &str) -> u32 {
    let lines = input.lines();
    let mut total_points = 0;
    for line in lines {
        let (rem_input, two_sections) = TwoSectionsGroups::from_line(line).unwrap();
        println!("rem_input = {rem_input}, two_sec = {}", two_sections.section2.start);
        if two_sections.within() {
            total_points += 1;
        }
    }
    total_points
}

fn main() {
    let input = include_str!("../input_sample.txt");

    let result = solve(input);

    println!("result = {result}");
}
