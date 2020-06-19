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
    let mut buf =  String::new();
    // aa.chars().zip(bb.chars()).for_each(|(a,b)| println!("{}{}", a, b));
    for (a, b) in aa.chars().zip(bb.chars()) {
        buf.push(a);
        buf.push(b);
    }
    println!("{}", buf);
}

fn a03() {
    let st = "Now I need a drink, alcoholic of course, after the heavy lectures involving quantum mechanics.".split_whitespace();
    let res = st.map(|s| s.replace(|c| c == '.' || c == ',', "").len()).fold(String::from(""), |acc, b| acc + &b.to_string());
    println!("{}", res);
}

fn a04() {
    let st: Vec<String> = "Hi He Lied Because Boron Could Not Oxidize Fluorine. New Nations Might Also Sign Peace Security Clause. Arthur King Can."
        .split_whitespace().map(|s| s.replace(|c| c == '.' || c == ',', "")).collect();

    let cs = vec![1, 5, 6, 7, 8, 9, 15, 16, 19];
    let mut res = vec![];
    for i in 0..st.len() {
        let output = &st[i];
        if cs.contains(&i) {
            res.push(&output[..1]);
        } else {
            res.push(&output[..2]);
        }
    }
    println!("{}", res.join(" "));
}

fn main() {
    a00();
    a01();
    a02();
    a03();
    a04();
}