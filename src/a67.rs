use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use rand::Rng;

fn calculate_center(invec: &Vec<Vec<f32>>, vec_size: usize, current_group: &[usize], groups: usize) -> Vec<Vec<f32>> {
    let mut res = vec![];
    for _ in 0..groups {
        res.push(vec![0.0; vec_size]);
    }
    let mut cnt = vec![0; groups];

    for i in 0..invec.len() {
        for j in 0..vec_size {
            res[current_group[i]][j] += invec[i][j];
        }
        cnt[current_group[i]] += 1;
    }

    for g in 0..groups {
        let cnt = cnt[g];
        for j in 0..vec_size {
            res[g][j] /= cnt as f32;
        }
    }
    return res;
}

// fn debug(value: &Vec<Vec<f32>>) {
//     for v in value {
//         v.iter().for_each(|value| print!("{:.2} ", value));
//         print!("\n");
//     }
// }

fn dist(a: &[f32], b: &[f32]) -> f32 {
    let mut res = 0.0;
    a.iter().zip(b).for_each(|(a,b)| res += (a-b)*(a-b));
    res = res.sqrt();
    return res;
}

fn get_nearest(a: &[f32], points: &[Vec<f32>]) -> usize {
    let mut min_dist = f32::INFINITY;
    let mut res = 0;
    for i in 0..points.len() {
        let dstv = dist(&points[i], a);
        if dstv < min_dist {
            min_dist = dstv;
            res = i;
        }
    }
    return res;
}

pub fn a67() -> std::io::Result<()> {
    let file = File::open("./country_vectors.txt")?;
    let reader = BufReader::new(file);
    let mut itr = reader.lines();

    let mut name = itr.next();
    let mut vect = itr.next();

    let mut names = vec![];
    let mut vecs = vec![];

    let mut rng = rand::thread_rng();
    let mut result = vec![];

    let k = 5;

    while name.is_some() {
        let mut buf = vec![];
        for word in vect.unwrap()?.split_whitespace() {
            buf.push(word.parse::<f32>().unwrap());
        }
        names.push(name.unwrap()?);
        vecs.push(buf);
        result.push(rng.gen_range(0, k));

        name = itr.next();
        vect = itr.next();
    }

    let mut weighted_center = calculate_center(&vecs, vecs[0].len(), &result, k);
    // debug(&weighted_center);

    let mut tries = 0;
    loop {
        let mut is_changed = false;
        for i in 0..vecs.len() {
            let nearest = get_nearest(&vecs[i], &weighted_center);

            is_changed = is_changed || (result[i] != nearest);
            result[i] = nearest;
        }

        if !is_changed {
            break;
        }

        weighted_center = calculate_center(&vecs, vecs[0].len(), &result, k);
        tries += 1;
    }

    for i in 0..names.len() {
        println!("{} {}", result[i], names[i]);
    }
    println!("tries {}", tries);

    return Ok(());
}
