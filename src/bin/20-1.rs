use bit_vec::BitVec;
use itertools::Itertools;

type Image = Vec<BitVec>;
const PAD: usize = 53;

fn parse(s: &str) -> (BitVec, Image) {
    let mut parts = s.split("\n\n");
    let inst_str = parts.next().unwrap();
    let image_str = parts.next().unwrap();

    let inst = inst_str.chars().map(|c| c == '#').collect();

    let mut img = vec![];

    for line in image_str.split_ascii_whitespace() {
        let line = line.trim();
        let mut img_line = BitVec::with_capacity(2 * PAD + line.len());
        img_line.grow(PAD, false);
        for c in line.chars() {
            img_line.push(c == '#');
        }
        img_line.grow(PAD, false);
        img.push(img_line);
    }
    let mut pad_line = img[0].clone();
    pad_line.clear();
    for _ in 0..PAD {
        img.push(pad_line.clone());
        img.insert(0, pad_line.clone());
    }

    (inst, img)
}

fn n9(img: &[BitVec], i: usize, j: usize) -> usize {
    let mut nsum = 0;
    for (ni, nj) in (i - 1..=i + 1).cartesian_product(j - 1..=j + 1) {
        nsum <<= 1;
        nsum |= img[ni][nj] as usize;
    }
    nsum
}

fn step(img: &[BitVec], inst: &BitVec) -> Image {
    let n = img.len();
    let mut img2 = img.to_vec();
    for (i, j) in (1..n - 1).cartesian_product(1..n - 1) {
        img2[i].set(j, inst[n9(img, i, j)]);
    }
    let b2 = img2[1][1];
    for k in 0..n {
        img2[0].set(k, b2);
        img2[k].set(0, b2);
        img2[n - 1].set(k, b2);
        img2[k].set(n - 1, b2);
    }
    img2
}

#[cfg(test)]
fn print_img(img: &[BitVec]) {
    for row in img {
        for pixel in row {
            print!("{}", if pixel { "██" } else { "  " });
        }
        println!();
    }
}

// fn solve

fn main() {
    let input = include_str!("../../inputs/20.txt");
    // let input = include_str!("../../inputs/20-sample.txt");
    let (inst, img0) = parse(input);
    // let img1 = step(&img0, &inst);
    // let img2 = step(&img1, &inst);
    // print_img(&img0);
    // print_img(&img1);
    // print_img(&img2);
    let mut imgn = img0.clone();
    for i in 1..=50 {
        imgn = step(&imgn, &inst);
        if i == 2 || i == 50 {
        dbg!(imgn
            .iter()
            .map(|row| row.iter().filter(|x| *x).count())
            .sum::<usize>());
        }
    }
}

#[test]
fn test_example() {
    let (inst, img0) = parse(include_str!("../../inputs/20-sample.txt"));
    assert_eq!(34, n9(&img0, PAD + 2, PAD + 2));
    assert!(inst[34]);
    let img1 = step(&img0, &inst);
    let img2 = step(&img1, &inst);
    print_img(&img0);
    print_img(&img2);
    assert_eq!(
        35,
        img2.iter()
            .map(|row| row.iter().filter(|x| *x).count())
            .sum::<usize>()
    );
}
