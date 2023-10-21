use std::collections::HashMap;

pub fn is_isomorphic() -> bool {
    let s = String::from("paper");
    let t = String::from("title");
    let mut map: HashMap<char, char> = HashMap::new();
    let iter_s = s.chars();
    let iter_t = t.chars();
    for (iter_s, iter_t) in iter_s.zip(iter_t) {
        match map.entry(iter_s) {
            std::collections::hash_map::Entry::Occupied(_) => match map.get(&iter_s) {
                Some(val) => {
                    if *val != iter_t {
                        return false;
                    }
                }
                None => {}
            },
            std::collections::hash_map::Entry::Vacant(_) => match map.insert(iter_s, iter_t) {
                Some(_) => (),
                None => (),
            },
        }
    }

    let mut redundant_check: HashMap<char, char> = HashMap::new();

    for (k, v) in &map {
        redundant_check.insert(*v, *k);
    }

    if map.len() != redundant_check.len() {
        return false;
    }

    true
}

// is valid paranthesis ()[]{} -> true , {[]() ->false
pub fn is_valid(s: String) -> bool {
    let mut correct_ans = HashMap::new();

    correct_ans.insert('(', ')');
    correct_ans.insert('{', '}');
    correct_ans.insert('[', ']');
    let mut stack: Vec<char> = vec![];

    let chars = s.chars();

    for i in chars {
        let popped = stack.pop();
        match popped {
            Some(val) => {
                let correspinding_ans = match correct_ans.get(&val) {
                    Some(closing) => *closing,
                    None => return false,
                };

                if correspinding_ans != i {
                    stack.push(val);
                    stack.push(i);
                }
            }
            None => (),
        }
    }
    return true;
}

// Input: n = 3
// Output: ["((()))","(()())","(())()","()(())","()()()"]
// Example 2:

// Input: n = 1
// Output: ["()"]
struct Solution {}

impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut ans: Vec<String> = vec![];
        _generate(&mut String::new(), 0, 0, n, &mut ans);
        ans
    }
}

pub fn _generate(
    starting_str: &mut String,
    open: i32,
    closed: i32,
    total: i32,
    ans: &mut Vec<String>,
) {
    if open == total {
        if closed != open {
            let mut i = closed;
            while i != open {
                starting_str.push(')');
                i += 1;
            }
        }
    }
    // println!("{starting_str}");
    if starting_str.len() == (total as usize) * 2 {
        ans.push(starting_str.clone());
        return;
    }

    let last = starting_str.chars().last();
    match last {
        Some(val) => {
            if val == '(' {
                if open < total {
                    let mut clone = starting_str.clone();
                    clone.push('(');
                    _generate(&mut clone, open + 1, closed, total, ans);
                }
                if closed < total {
                    let mut clone = starting_str.clone();
                    clone.push(')');
                    _generate(&mut clone, open, closed + 1, total, ans);
                }
            } else {
                if open < total {
                    let mut clone = starting_str.clone();
                    clone.push('(');
                    _generate(&mut clone, open + 1, closed, total, ans);
                }
                if closed < total && open > closed {
                    let mut clone = starting_str.clone();
                    clone.push(')');
                    _generate(&mut clone, open, closed + 1, total, ans);
                }
            }
        }
        None => {
            if open < total {
                starting_str.push('(');
                _generate(starting_str, open + 1, closed, total, ans);
            }
        }
    }
}

fn main() {
    let sol = Solution::generate_parenthesis(3);
    for i in sol {
        println!("{i}");
    }
}
