use std::{collections::BTreeMap, error::Error};

use crate::file;

pub fn run(part: bool) -> Result<u32, Box<dyn Error>> {
    let inp = file::get_input(1)?;
    let (mut list1, mut list2) = split_input(&inp);
    let sum = if !part {
        part1(&mut list1, &mut list2)
    } else {
        part2(list1, list2)
    };
    Ok(sum)
}
fn split_input(input: &str) -> (Vec<u32>, Vec<u32>) {
    let input: (Vec<_>, Vec<_>) = input
        .split_whitespace()
        .filter_map(|num| num.parse::<u32>().ok())
        .enumerate()
        .partition(|item| item.0 % 2 == 0);
    let [list1, list2] =
        [input.0, input.1].map(|vec| vec.iter().map(|num| num.1).collect::<Vec<_>>());
    (list1, list2)
}
fn part2(list1: Vec<u32>, list2: Vec<u32>) -> u32 {
    let mut map: BTreeMap<u32, u32> = BTreeMap::new();
    for item in list2 {
        map.entry(item).and_modify(|n| *n += 1).or_insert(1);
    }
    let sum: u32 = list1
        .iter()
        .map(|item| map.get(item).unwrap_or(&0) * item)
        .sum();
    sum
}
fn part1(list1: &mut Vec<u32>, list2: &mut Vec<u32>) -> u32 {
    list1.sort();
    list2.sort();
    let sum: u32 = list1
        .iter()
        .zip(list2.iter())
        .map(|nums| nums.0.abs_diff(*nums.1))
        .sum();
    sum
}
