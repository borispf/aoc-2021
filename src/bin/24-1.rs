use itertools::Itertools;
use std::collections::HashSet;
use std::rc::Rc;

type RegSet = HashSet<i64>;
type RegFile = [Rc<HashSet<i64>>; 4];

#[derive(Clone, Copy, Debug)]
enum RegOrLit {
    Reg(usize),
    Lit(i64),
}

#[derive(Clone, Copy, Debug)]
enum OpCode {
    Inp,
    Add(usize, RegOrLit),
    Mul(usize, RegOrLit),
    Div(usize, RegOrLit),
    Mod(usize, RegOrLit),
    Eql(usize, RegOrLit),
}

use OpCode::*;
use RegOrLit::*;

impl OpCode {
    fn call(&self, x: i64, y: i64) -> Option<i64> {
        match *self {
            Inp => unimplemented!(),
            Add(..) => Some(x + y),
            Mul(..) => Some(x * y),
            Div(..) if y != 0 => Some(x / y),
            Mod(..) if x >= 0 && y > 0 => Some(x % y),
            Eql(..) => Some((x == y) as i64),
            _ => None,
        }
    }
    fn src(self) -> RegOrLit {
        match self {
            Inp => unimplemented!(),
            Add(_, rol) => rol,
            Mul(_, rol) => rol,
            Div(_, rol) => rol,
            Mod(_, rol) => rol,
            Eql(_, rol) => rol,
        }
    }
    fn dest(self) -> usize {
        match self {
            Inp => unimplemented!(),
            Add(dest, _) => dest,
            Mul(dest, _) => dest,
            Div(dest, _) => dest,
            Mod(dest, _) => dest,
            Eql(dest, _) => dest,
        }
    }
}

fn reg_idx(s: &str) -> usize {
    (s.as_bytes()[0] - b'w').try_into().unwrap()
}

type Ops = Vec<(&'static str, OpCode)>;

fn parse(input: &'static str) -> Ops {
    let mut ops = vec![];

    for line in input.lines() {
        let line = line.trim();
        let tokens = line.split_whitespace().collect_vec();
        let op_str = tokens[0];
        if op_str == "inp" {
            ops.push((line, Inp));
            continue;
        }

        let dest = reg_idx(tokens[1]);
        let src = match tokens[2].parse::<i64>() {
            Ok(x) => Lit(x),
            Err(_) => Reg(reg_idx(tokens[2])),
        };

        ops.push((
            line,
            match tokens[0] {
                "add" => Add(dest, src),
                "mul" => Mul(dest, src),
                "div" => Div(dest, src),
                "mod" => Mod(dest, src),
                "eql" => Eql(dest, src),
                _ => unimplemented!("{}", line),
            },
        ));
    }

    ops
}

fn print_line_reg(line: &str, reg: &RegFile) {
    println!(
        "{:11} {:25} {:35} {:35} {:35}",
        line.trim(),
        fmt_set(&*reg[0]),
        fmt_set(&*reg[1]),
        fmt_set(&*reg[2]),
        fmt_set(&*reg[3])
    );
}

fn fwd_pass(regs: &mut Vec<RegFile>, ops: &Ops) {
    let is_first_pass = regs.len() == 1;
    for (i, (line, op)) in ops.iter().enumerate() {
        if is_first_pass {
            regs.push(regs[regs.len() - 1].clone());
        }
        if let Inp = *op {
            if is_first_pass {
                regs[i + 1][0] = Rc::new(HashSet::from_iter(1..=9));
            }
            for reg_idx in 1..4 {
                regs[i + 1][reg_idx] = Rc::new(
                    regs[i][reg_idx]
                        .intersection(&*regs[i + 1][reg_idx])
                        .cloned()
                        .collect(),
                );
            }
        } else {
            if !is_first_pass {
                for reg_idx in 0..4 {
                    regs[i + 1][reg_idx] = Rc::new(
                        regs[i][reg_idx]
                            .intersection(&*regs[i + 1][reg_idx])
                            .cloned()
                            .collect(),
                    );
                }
            }
            let src_in = match op.src() {
                Lit(x) => Rc::new(HashSet::from_iter([x])),
                Reg(src_idx) => regs[i][src_idx].clone(),
            };
            let dest_in = Rc::clone(&regs[i][op.dest()]);
            let mut dest_out = HashSet::new();

            for (x, y) in dest_in
                .iter()
                .cloned()
                .cartesian_product(src_in.iter().cloned())
            {
                if let Some(z) = op.call(x, y) {
                    if z < 1_000_000 {
                        dest_out.insert(z);
                    }
                }
            }
            regs[i + 1][op.dest()] = Rc::new(dest_out);
        }

        print_line_reg(line, &regs[i + 1]);
    }
}

fn bwd_pass(regs: &mut Vec<RegFile>, ops: &Ops) {
    regs.reverse();
    for (i, (line, op)) in ops.iter().rev().enumerate() {
        if let Inp = *op {
            for reg_idx in 1..4 {
                regs[i + 1][reg_idx] = Rc::new(
                    regs[i][reg_idx]
                        .intersection(&*regs[i + 1][reg_idx])
                        .cloned()
                        .collect(),
                );
            }
        } else {
            let dest_out = Rc::clone(&regs[i][op.dest()]);
            let dest_in = Rc::clone(&regs[i + 1][op.dest()]);

            for reg_idx in 0..4 {
                regs[i + 1][reg_idx] = Rc::new(
                    regs[i][reg_idx]
                        .intersection(&*regs[i + 1][reg_idx])
                        .cloned()
                        .collect(),
                );
            }

            let (src_in, maybe_src_idx) = match op.src() {
                Lit(x) => (Rc::new(HashSet::from_iter([x])), None),
                Reg(src_idx) => (Rc::clone(&regs[i + 1][src_idx]), Some(src_idx)),
            };

            let mut dest_in_filtered = HashSet::new();
            for x in dest_in.iter() {
                for y in src_in.iter() {
                    if let Some(z) = op.call(*x, *y) {
                        if dest_out.contains(&z) {
                            dest_in_filtered.insert(*x);
                            break;
                        }
                    }
                }
            }
            regs[i + 1][op.dest()] = Rc::new(dest_in_filtered);

            if let Some(src_idx) = maybe_src_idx {
                let dest_in = Rc::clone(&regs[i + 1][op.dest()]);
                let mut src_in_filtered = HashSet::new();
                for y in src_in.iter() {
                    for x in dest_in.iter() {
                        if let Some(z) = op.call(*x, *y) {
                            if dest_out.contains(&z) {
                                src_in_filtered.insert(*y);
                                break;
                            }
                        }
                    }
                }
                regs[i + 1][src_idx] = Rc::new(src_in_filtered);
            }
        }
        print_line_reg(line, &regs[i]);
    }
    regs.reverse();
}

fn run(ops: &Ops, inps: &[i64]) -> i64 {
    let mut inp_idx = 0;
    let mut regs = [0; 4];
    for (_line, op) in ops {
        if let Inp = *op {
            regs[0] = inps[inp_idx];
            inp_idx += 1;
            continue;
        }
        let x = regs[op.dest()];
        let y = match op.src() {
            Lit(y) => y,
            Reg(src_idx) => regs[src_idx],
        };
        regs[op.dest()] = op.call(x, y).unwrap();
    }
    regs[3]
}

fn main() {
    let input = include_str!("../../inputs/24.txt");
    // let input = include_str!("../../inputs/24-sample.txt");
    let zero = Rc::new(RegSet::from_iter([0]));

    let mut regs = vec![[
        Rc::clone(&zero),
        Rc::clone(&zero),
        Rc::clone(&zero),
        Rc::clone(&zero),
    ]];

    let ops = parse(input);

    fwd_pass(&mut regs, &ops);
    regs[ops.len()][3] = Rc::clone(&zero);

    for _ in 0..10 {
        println!();
        bwd_pass(&mut regs, &ops);

        println!();
        fwd_pass(&mut regs, &ops);
    }

    let mut inp_sets = vec![];
    for (i, (_line, op)) in ops.iter().enumerate() {
        if let Inp = op {
            inp_sets.push(Rc::clone(&regs[i + 1][0]));
        }
    }

    for inp in inp_sets.iter().map(|is| is.iter().sorted().rev().cloned()).multi_cartesian_product() {
        if run(&ops, &inp) == 0 {
            dbg!(inp.iter().cloned().fold1(|a, b| a * 10 + b).unwrap());
        }
    }
}

#[allow(unstable_name_collisions)]
fn fmt_set(a: &RegSet) -> String {
    if a.len() > 15 {
        let (min, max) = a.iter().minmax().into_option().unwrap();
        return format!("{{{}-{}; {}}}", min, max, a.len());
    }
    let mut s = String::new();
    s.push_str("{");
    s.extend(
        a.iter()
            .cloned()
            .sorted()
            .map(|x| x.to_string())
            .intersperse(",".to_owned()),
    );
    s.push_str("}");
    s
}
