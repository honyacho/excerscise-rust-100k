use rand::seq::SliceRandom;
use std::collections::HashSet;
use std::io::{self, Read};

pub mod a12;
pub mod a13;
pub mod a14;
pub mod a15;

fn a00() {
    let s = "stressed";
    let reversed = s.chars().rev().collect::<String>();
    println!("{}", reversed);
}

fn a01() {
    let s = "パタトクカシーー";
    let aa = s.chars().step_by(2).collect::<String>();
    println!("{}", aa);
}

fn a02() {
    let aa = "パトカー";
    let bb = "タクシー";
    let mut buf = String::new();
    // aa.chars().zip(bb.chars()).for_each(|(a,b)| println!("{}{}", a, b));
    for (a, b) in aa.chars().zip(bb.chars()) {
        buf.push(a);
        buf.push(b);
    }
    println!("{}", buf);
}

fn a03() {
    let st = "Now I need a drink, alcoholic of course, after the heavy lectures involving quantum mechanics.".split_whitespace();
    let res = st
        .map(|s| s.replace(|c| c == '.' || c == ',', "").len())
        .fold(String::from(""), |acc, b| acc + &b.to_string());
    println!("{}", res);
}

fn a04() {
    let cs: HashSet<usize> = vec![1, 5, 6, 7, 8, 9, 15, 16, 19].into_iter().collect();

    let res = "Hi He Lied Because Boron Could Not Oxidize Fluorine. New Nations Might Also Sign Peace Security Clause. Arthur King Can."
        .split_whitespace().map(|s| s.replace(|c| c == '.' || c == ',', ""))
        .enumerate()
        .map(|(i, v)| String::from(if cs.contains(&(i+1)) { &v[..1] } else { &v[..2] }))
        .fold(String::from(""), |a, st| a + " " + &st);
    println!("{}", &res[1..]);
}

fn a05a(s: String, n: usize) -> Vec<String> {
    let mut v = vec![];
    // vec![String::from("hoge")];
    for i in 0..(s.len() - n + 1) {
        v.push(String::from(&s[i..(i + n)]));
    }
    return v;
}

fn a05b(s: &str, n: usize) -> Vec<Vec<&str>> {
    let v: Vec<&str> = s.split_whitespace().collect();
    let mut res = vec![];
    for i in 0..(v.len() - n + 1) {
        res.push(Vec::from(&v[i..(i + n)]));
    }
    return res;
}

fn a06() {
    let sample1 = "paraparaparadise";
    let sample2 = "paragraph";
    let mut st1: HashSet<&str> = HashSet::new();
    let mut st2: HashSet<&str> = HashSet::new();
    for i in 0..(sample1.len() - 1) {
        st1.insert(&sample1[i..(i + 2)]);
    }
    for i in 0..(sample2.len() - 1) {
        st2.insert(&sample2[i..(i + 2)]);
    }
    let unin: Vec<&str> = st1.union(&st2).map(|s| *s).collect();
    let intr: Vec<&str> = st1.intersection(&st2).map(|s| *s).collect();
    let diff: Vec<&str> = st1.difference(&st2).map(|s| *s).collect();
    println!("{:?}", unin);
    println!("{:?}", intr);
    println!("{:?}", diff);
}

fn a07() {
    let x = 12;
    let y = "気温";
    let z = 22.4;
    println!("{}時の{}は{}", x, y, z);
}

fn a08(instr: &str) {
    let res: String = String::from(instr)
        .chars()
        .map(|s| {
            if s.is_ascii_lowercase() {
                (219 - (s as u8)) as char
            } else {
                s
            }
        })
        .collect();
    println!("{}", res);
}

fn a09(instr: &str) {
    let words = instr.split_whitespace();
    let mut rng = rand::thread_rng();
    let mut mp = words.map(|w| {
        let mut scrubbled = String::from("");
        let mut char_arr: Vec<char> = w.chars().collect();
        let w_size = char_arr.len();
        if w_size > 4 {
            scrubbled.push(char_arr[0]);
            let sl = &mut char_arr[1..(w_size - 1)];
            sl.shuffle(&mut rng);
            scrubbled.push(char_arr[w_size - 1]);
        }
        return char_arr.iter().collect::<String>();
    });
    let mut res = String::from("");
    res.push_str(&mp.next().unwrap());
    for wd in mp {
        res.push(' ');
        res.push_str(&wd);
    }
    println!("{}", res);
}

fn a10(input: &str) -> i32 {
    let mut res = 0;
    for c in input.chars() {
        if c == '\n' {
            res += 1;
        }
    }
    return res;
}

fn a11(input: &str) -> String {
    let mut res = String::from("");
    for c in input.chars() {
        if c == '\t' {
            res.push(' ');
        } else {
            res.push(c);
        }
    }
    return res;
}

fn main() {
    let src = io::stdin();
    let mut input = String::from("");
    let res = src.read_line(&mut input);
    if res.is_err() {
        println!("input error");
        return;
    }
    match input.trim() {
        "12" => a12::a12().unwrap(),
        "13" => a13::a13().unwrap(),
        "14" => a14::a14().unwrap(),
        "15" => a15::a15().unwrap(),
        _ => {
            a00();
            a01();
            a02();
            a03();
            a04();
            println!("{:?}", a05a(String::from("I am an NLPer"), 2));
            println!("{:?}", a05a(String::from("I am an NLPer"), 3));
            println!("{:?}", a05a(String::from("I am an NLPer"), 4));
            println!("{:?}", a05b("I am an NLPer", 2));
            println!("{:?}", a05b("I am an NLPer", 3));
            println!("{:?}", a05b("I am an NLPer", 4));
            a06();
            a07();
            a08("abcABC114514ほげ");
            a09("I couldn’t believe that I could actually understand what I was reading : the phenomenal power of the human mind .");
            println!("{:?}", a10("dafdsaf\nfsafdsafda\nfsadfdsafda\n"));
            println!(
                "{}",
                a11("\t\tdafdsaf\n\tfsafdsafda\n\t\t\t\tfsadfdsafda\n")
            );
        }
    }
}
