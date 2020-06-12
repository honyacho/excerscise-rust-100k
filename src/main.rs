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

fn main() {
    a00();
    a01();
    a02();
}