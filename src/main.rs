use std::{collections::VecDeque, io::{BufReader, BufRead}, fs::File};


#[derive(Clone, PartialEq, Eq, Debug)]
enum SnailNumber {
    Nat(u8),
    Pair((Box<SnailNumber>, Box<SnailNumber>))
}

impl std::ops::Add for SnailNumber {
    type Output = SnailNumber;

    fn add(self, rhs: Self) -> Self::Output {
        let mut res = Self::Pair((Box::new(self), Box::new(rhs)));
        res.reduce();
        res
    }
}

impl SnailNumber {

    fn reduce(&mut self) {
        while self.explode() || self.split() { }
    }

    fn explode(&mut self) -> bool{
        
        let mut stack: VecDeque<(u8, &mut SnailNumber)> = VecDeque::new();
        
        if let Self::Pair((left, right)) = self {
            stack.push_back((0, right));
            stack.push_back((0, left));
        } else {
            return false;
        }

        let mut left_neighbour: Option<&mut SnailNumber> = Option::None;
        let mut update_right_neighbour = 0;
        let mut explode_pair_found = false;

        while !stack.is_empty() {

            let (level, node) = stack.pop_back().unwrap();

            if !explode_pair_found && level == 3 {

                if let SnailNumber::Pair((ref left_child, ref right_child)) = node {
                    {
                        explode_pair_found = true;

                        let left_child_number = match left_child.as_ref() {
                            SnailNumber::Pair((_l, _r)) => {panic!("invalid snail number")},
                            SnailNumber::Nat(c) => {c}
                        };

                        if let Some(ref mut l) = left_neighbour {
                            if let SnailNumber::Nat(c) = l {
                                *c += left_child_number;
                            } else {
                                panic!("left neighbour cannot be pair");
                            }
                        }

                        update_right_neighbour = match right_child.as_ref() {
                            SnailNumber::Pair((_l, _r)) => {panic!("invalid snail number")},
                            SnailNumber::Nat(c) => {*c}
                        };

                        *node = SnailNumber::Nat(0);

                        continue;
                    }
                }
            }

            match node {
                SnailNumber::Pair((ref mut left_child, ref mut right_child)) => {
                    stack.push_back((level + 1, right_child));
                    stack.push_back((level + 1, left_child));
                },
                SnailNumber::Nat(content) => {
                    if !explode_pair_found {
                        left_neighbour = Some(node);
                    } else {
                        *content += update_right_neighbour;
                        return true;
                    }
                }
            }
        }

        explode_pair_found
    }

    fn split(&mut self) -> bool {

        let mut stack: VecDeque<&mut SnailNumber> = VecDeque::new();
        
        if let Self::Pair((left, right)) = self {
            stack.push_back(right);
            stack.push_back(left);
        } else {
            return false;
        }

        while !stack.is_empty() {

            let node = stack.pop_back().unwrap();

            match node {
                SnailNumber::Pair((ref mut left_child, ref mut right_child)) => {
                    stack.push_back(right_child);
                    stack.push_back(left_child);
                },
                SnailNumber::Nat(ref content) => {
                    if *content >= 10 {
                        *node = *Box::new(SnailNumber::Pair((
                            Box::new(SnailNumber::Nat(*content/2)),
                            Box::new(SnailNumber::Nat((*content + 1) / 2)))));

                        return true;
                    }
                }
            }
        }

        false
    }

    fn magnitude(&self) -> usize {

        let mut stack: VecDeque<(usize, &SnailNumber)> = VecDeque::new();
        stack.push_back((1, self));
        let mut res = 0;

        while let Some((mult, node)) = stack.pop_back() {
            
            match node {
                SnailNumber::Nat(c) => res += mult * (*c as usize),
                SnailNumber::Pair((left, right)) => {
                    stack.push_back((mult * 2, right));
                    stack.push_back((mult * 3, left));
                },
            }
        }
        res
    }

}

impl std::fmt::Display for SnailNumber{

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Nat(c) => write!(f, "{}", *c),
            Self::Pair((left, right)) => write!(f, "[{},{}]", *left, *right)
        }
    }
}

impl std::convert::TryFrom<&str> for SnailNumber {
    type Error = ();

    fn try_from(input: &str) -> Result<Self, Self::Error> {

        if input.len() == 0 {
            return Err(());
        }

        if input.chars().nth(0).unwrap() == '[' &&  input.chars().nth_back(0).unwrap() == ']' {

            let inner = &input[1..input.len()-1];
            let mut bracket_count = 0;

            for (index, c) in inner.chars().enumerate() {
                if c == '[' {
                    bracket_count += 1;
                } else if c == ']' {
                    bracket_count -= 1;
                } else if bracket_count == 0 && c == ',' {
                    let left: SnailNumber = inner[0..index].try_into()?;
                    let right: SnailNumber = inner[index+1..inner.len()].try_into()?;

                    return Ok(SnailNumber::Pair((Box::new(left), Box::new(right))));
                }
            }
        } else {
            return match input.parse::<u8>() {
                Ok(c) => Ok(SnailNumber::Nat(c)),
                Err(_) => Err(())
            }
        }

        Err(())
    }
}


fn main() {

    let lines: Vec<String> = BufReader::new(File::open("input.txt").unwrap()).lines().map(|l| l.unwrap()).collect();

    let mut res = 0;
    
    for (index_x, x) in lines.iter().map(|l| SnailNumber::try_from(&l[..]).unwrap()).enumerate() {
        for y in lines[index_x + 1..].iter().map(|l| SnailNumber::try_from(&l[..]).unwrap()) {

             res = std::cmp::max(res, (x.clone() + y.clone()).magnitude());
             res = std::cmp::max(res, (y + x.clone()).magnitude());
        }
    }

    println!("res {}", res);    
}
