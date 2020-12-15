use std::collections::{HashMap,HashSet};
use std::error::Error;
use std::fs::File;
use std::io::{BufReader,BufRead};
use nom::{
    sequence::tuple,
    combinator::opt,
    multi::many0,
    IResult,
};
use nom::bytes::complete::{tag};
use nom::character::complete::{alpha1,one_of,digit1};

#[derive(Debug)]
struct Bag {
    name: String,
    contents: Vec<(usize,String)>,
}

fn bag_tag(input: &str) -> IResult<&str,()> {
    let (input,_) = tag(" bag")(input)?;
    let (input,_) = opt(tag("s"))(input)?;
    Ok((input,()))
}

fn bag_name(input: &str) -> IResult<&str,String> {
    let(input,(first,_,second,_)) = tuple((alpha1,tag(" "),alpha1,bag_tag))(input)?;
    let mut output = String::from(first);
    output.push(' ');
    output.push_str(second);
    Ok((input,output))
}

fn bag_content(input: &str) -> IResult<&str,(usize, String)> {
    let (input,(_,num,_,bag,_)) = tuple((tag(" "),digit1,tag(" "),bag_name,one_of(".,")))(input)?;
    let num = num.parse().unwrap();
    Ok((input,(num,String::from(bag))))
}

fn bag_rule(input: &str) -> Result<Bag,Box<dyn Error + '_>> {
    let (input,(name,_,contents)) = tuple((bag_name,tag(" contain"),many0(bag_content)))(input)?;
    Ok(Bag{name,contents})
}
fn main() -> Result<(), Box<dyn Error>> {
    let raw = File::open("./input.txt")?;
    let buf = BufReader::new(raw);
    let mut bags = HashMap::new();
    let mut stack = Vec::new();
    for line in buf.lines() {
        let line = line.unwrap();
        //let s: &'static str = Box::leak(line.into_boxed_str());
        let bag = bag_rule(&line).unwrap();
        bags.insert(bag.name.clone(),bag.contents);
        stack.push(bag.name);
    }

    let mut seen = HashMap::new();
    println!("{}",bagcount("shiny gold",&bags,&mut seen));
    Ok(())
}

fn bagcount(input: &str, bags: &HashMap<String,Vec<(usize,String)>>, seen: &mut HashMap<String, usize>) -> usize {
    let mut count = 1;
    for (num, bag) in bags.get(input).unwrap() {
        count += match seen.get(bag) {
            Some(i) => {num * i},
            None => {let i = bagcount(bag,bags,seen); seen.insert(String::from(bag), i); i * num},
        }
    }
    return count;
}
