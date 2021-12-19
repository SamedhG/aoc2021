use std::{cell::RefCell, default, rc::Rc, str::FromStr};

use crate::util::parse_strings;
use anyhow::Result;

#[derive(Debug, Clone)]
enum SF {
    Pair(Rc<RefCell<SF>>, Rc<RefCell<SF>>),
    Regular(usize),
}
impl default::Default for SF {
    fn default() -> Self {
        SF::Regular(0)
    }
}

impl SF {
    fn is_pair(&self) -> bool {
        match self {
            Self::Pair(_, _) => true,
            Self::Regular(_) => false,
        }
    }

    fn unwrap_regular(&self) -> usize {
        match self {
            Self::Regular(x) => *x,
            Self::Pair(_, _) => panic!("Unwraping a pair as a regular"),
        }
    }

    fn add_to_regular(me: &Rc<RefCell<Self>>, n: usize) {
        match &mut *me.borrow_mut() {
            Self::Regular(x) => *x += n,
            _ => panic!("Cant add single number to pair"),
        }
    }

    fn magnitude(me: &Rc<RefCell<Self>>) -> usize {
        match &*me.borrow() {
            SF::Regular(x) => *x,
            SF::Pair(left, right) => SF::magnitude(left) * 3 + SF::magnitude(right) * 2,
        }
    }

    fn add(me: &Rc<RefCell<Self>>, other: &Rc<RefCell<Self>>) -> Rc<RefCell<Self>> {
        let result = Rc::new(RefCell::new(SF::Pair(me.clone(), other.clone())));
        loop {
            if SF::explode(&result) {
                continue;
            }
            if SF::split(&result) {
                continue;
            }
            break;
        }
        result
    }

    fn explode(me: &Rc<RefCell<Self>>) -> bool {
        let clone = me.clone();
        let exploded = SF::explode_help(me, &clone, 4);
        match exploded {
            None => false,
            Some(x) => {
                x.take();
                true
            }
        }
    }
    // Adds the left and right value of the cell to explode and returns the value to be removed
    // from the tree
    fn explode_help(
        me: &Rc<RefCell<Self>>,
        curr: &Rc<RefCell<SF>>,
        count: usize,
    ) -> Option<Rc<RefCell<Self>>> {
        match &*curr.borrow() {
            Self::Regular(_) => None,
            Self::Pair(left, right) => {
                if count == 0 {
                    // since we always have a max depth of 5 we know that this must be a pair of 2
                    // regulars
                    {
                        let l = left.borrow().unwrap_regular();
                        let r = right.borrow().unwrap_regular();
                        let my_left = SF::left(&me, &left);
                        let my_right = SF::right(&me, &right);
                        if let Some(ml) = my_left {
                            SF::add_to_regular(&ml, l);
                        }
                        if let Some(mr) = my_right {
                            SF::add_to_regular(&mr, r);
                        }
                    }
                    Some(curr.clone())
                } else if let Some(x) = SF::explode_help(me, left, count - 1) {
                    Some(x)
                } else {
                    SF::explode_help(me, right, count - 1)
                }
            }
        }
    }

    fn split(me: &Rc<RefCell<Self>>) -> bool {
        let clone = me.clone();
        let split = SF::find_split(me, &clone);
        match split {
            None => false,
            Some(x) => {
                let val = x.borrow().unwrap_regular();
                let odd = val % 2;
                let new_left = Rc::new(RefCell::new(SF::Regular(val / 2)));
                let new_right = Rc::new(RefCell::new(SF::Regular((val / 2) + odd)));
                let new_node = SF::Pair(new_left, new_right);
                x.replace(new_node);
                true
            }
        }
    }

    fn find_split(me: &Rc<RefCell<Self>>, curr: &Rc<RefCell<SF>>) -> Option<Rc<RefCell<Self>>> {
        match &*curr.borrow() {
            SF::Regular(x) => {
                if *x >= 10 {
                    Some(curr.clone())
                } else {
                    None
                }
            }
            SF::Pair(left, right) => {
                let possible = SF::find_split(me, left);
                if possible.is_some() {
                    possible
                } else {
                    SF::find_split(me, right)
                }
            }
        }
    }

    /// Find the left most in the given tree
    fn left_most(me: &Rc<RefCell<Self>>) -> Rc<RefCell<Self>> {
        match &*me.borrow() {
            SF::Regular(_) => me.clone(),
            SF::Pair(left, _) => Self::left_most(left),
        }
    }

    /// Find the right most pair in the given tree
    fn right_most(me: &Rc<RefCell<Self>>) -> Rc<RefCell<Self>> {
        match &*me.borrow() {
            SF::Regular(_) => me.clone(),
            SF::Pair(_, right) => Self::right_most(right),
        }
    }

    /// Given a tree and a node find the right element to that node
    fn right(me: &Rc<RefCell<Self>>, node: &Rc<RefCell<Self>>) -> Option<Rc<RefCell<Self>>> {
        match &*me.borrow() {
            SF::Regular(_) => None,
            SF::Pair(left, right) => {
                if Rc::ptr_eq(node, &left) {
                    return Some(Self::left_most(right));
                }
                if Rc::ptr_eq(node, &right) {
                    return None;
                }
                if left.borrow().is_pair() {
                    let lrm = Self::right_most(left);
                    if Rc::ptr_eq(&lrm, node) {
                        return Some(Self::left_most(right));
                    }
                    let possible = Self::right(&left, node);
                    if possible.is_some() {
                        return possible;
                    }
                }
                Self::right(&right, node)
            }
        }
    }

    /// Given a tree and a node find the left element to that node
    fn left(me: &Rc<RefCell<Self>>, node: &Rc<RefCell<Self>>) -> Option<Rc<RefCell<Self>>> {
        match &*me.borrow() {
            SF::Regular(_) => None,
            SF::Pair(left, right) => {
                if Rc::ptr_eq(node, &right) {
                    return Some(Self::right_most(left));
                }
                if Rc::ptr_eq(node, &left) {
                    return None;
                }
                if right.borrow().is_pair() {
                    let rlm = Self::left_most(right);
                    if Rc::ptr_eq(&rlm, node) {
                        return Some(Self::right_most(left));
                    }
                    let possible = Self::left(&right, node);
                    if possible.is_some() {
                        return possible;
                    }
                }
                Self::left(&left, node)
            }
        }
    }

    fn print(&self) {
        match self {
            SF::Regular(x) => print!("{}", x),
            SF::Pair(left, right) => {
                print!("[");
                left.borrow().print();
                print!(",");
                right.borrow().print();
                print!("]");
            }
        }
    }

    fn deep_clone(me: &Rc<RefCell<Self>>) -> Rc<RefCell<Self>> {
        match &*me.borrow() {
            SF::Regular(x) => Rc::new(RefCell::new(SF::Regular(*x))),
            SF::Pair(left, right) => Rc::new(RefCell::new(SF::Pair(
                SF::deep_clone(left),
                SF::deep_clone(right),
            ))),
        }
    }
}

impl FromStr for SF {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> std::result::Result<SF, Self::Err> {
        match find_comma(s) {
            None => Ok(Self::Regular(usize::from_str_radix(s, 10)?)),
            Some(x) => {
                let (mut left, mut right) = s.split_at(x);
                left = left.strip_prefix("[").unwrap();
                right = right.strip_suffix("]").unwrap();
                right = right.strip_prefix(",").unwrap();
                let left_sf = SF::from_str(left)?;
                let right_sf = SF::from_str(right)?;
                Ok(Self::Pair(
                    Rc::new(RefCell::new(left_sf)),
                    Rc::new(RefCell::new(right_sf)),
                ))
            }
        }
    }
}

fn find_comma(s: &str) -> Option<usize> {
    let mut counter = 0;
    for (i, c) in s.chars().enumerate() {
        match c {
            '[' => counter += 1,
            ']' => counter -= 1,
            ',' => {
                if counter == 1 {
                    return Some(i);
                }
            }
            _ => {}
        }
    }
    None
}

fn parse(filename: &str) -> Result<Vec<Rc<RefCell<SF>>>> {
    let strings = parse_strings(filename)?;
    let mut output = Vec::new();
    for s in strings {
        output.push(Rc::new(RefCell::new(SF::from_str(&s)?)));
    }
    Ok(output)
}

fn q1(filename: &str) -> Result<usize> {
    let snailfish_numbers = parse(filename)?;
    let mut sum = snailfish_numbers[0].clone();
    for sf in snailfish_numbers.iter().skip(1) {
        sum = SF::add(&sum, sf);
    }
    Ok(SF::magnitude(&sum))
}

fn q2(filename: &str) -> Result<usize> {
    let snailfish_numbers = parse(filename)?;
    let mut max = 0;
    for i in 0..snailfish_numbers.len() {
        for j in 0..snailfish_numbers.len() {
            // Dont like this but had to do a deep clone here :sweat_smile:
            // The first time a number was added to anythine a bunch of its refcells got mutated so
            // we were adding a different number in the next time
            // Should have made add in a way that returned a new sum without destroying the numbers
            let n1 = SF::deep_clone(&snailfish_numbers[i]);
            let n2 = SF::deep_clone(&snailfish_numbers[j]);
            let sum = SF::add(&n1, &n2);
            max = max.max(SF::magnitude(&sum));
        }
    }
    Ok(max)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        assert_eq!(q1("./data/day18.txt").unwrap(), 4088);
        assert_eq!(q2("./data/day18.txt").unwrap(), 4536);
    }
}
