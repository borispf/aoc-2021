#[macro_use]
extern crate lazy_static;

use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap};
use std::io::Write;

// #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
// enum Square {
//     Hall { lr: i8 },
//     Room { lr: i8, ud: i8 },
// }

// impl Square {
//     fn to_usize(self) -> usize {
//         match self {
//             Hall { lr } => lr,
//             Room { lr, ud } => 6 + 2 * lr + ud,
//         }
//         .try_into()
//         .unwrap()
//     }
// }

// 01234567890
//   1 5 9 3
//   2 6 0 4
//   3 7 1 5
//   4 8 2 6

lazy_static! {

    static ref N: [Vec<usize>; 27] = [
        vec![1], // 0
        vec![0, 2],
        vec![1, 3, 11],
        vec![2, 4],
        vec![3, 5, 15],
        vec![4, 6], // 5
        vec![5, 7, 19],
        vec![6, 8],
        vec![7, 9, 23],
        vec![8, 10],
        vec![9], // 10
        vec![2, 12],
        vec![11, 13],
        vec![12, 14],
        vec![13],
        vec![4, 16], // 15
        vec![15, 17],
        vec![16, 18],
        vec![17],
        vec![6, 20],
        vec![19, 21], // 20
        vec![20, 22],
        vec![21],
        vec![8, 24],
        vec![23, 25],
        vec![24, 26], // 25
        vec![25],
    ];

}

// 01234567890
//   1 5 9 3
//   2 6 0 4
//   3 7 1 5
//   4 8 2 6

fn is_home(i: usize, p: u8) -> bool {
    HOME_DIST[i][p as usize] == 0
}

static HOME_DIST: [[i64; 5]; 27] = [
    [0, 3, 5, 7, 9], // 0
    [0, 2, 4, 6, 8],
    [0, 1, 3, 5, 7],
    [0, 2, 2, 4, 6],
    [0, 3, 1, 3, 5],
    [0, 4, 2, 2, 4], // 5
    [0, 5, 3, 1, 3],
    [0, 6, 4, 2, 2],
    [0, 7, 5, 3, 1],
    [0, 8, 6, 4, 2],
    [0, 9, 7, 5, 3], // 10
    [0, 0, 4, 6, 8],
    [0, 0, 5, 7, 9],
    [0, 0, 6, 8, 10],
    [0, 0, 7, 9, 11],
    [0, 4, 0, 4, 6], // 15
    [0, 5, 0, 5, 7],
    [0, 6, 0, 6, 8],
    [0, 7, 0, 7, 9],
    [0, 6, 4, 0, 4],
    [0, 7, 5, 0, 5], // 20
    [0, 8, 6, 0, 6],
    [0, 9, 7, 0, 7],
    [0, 8, 6, 4, 0],
    [0, 9, 7, 5, 0],
    [0, 10, 8, 6, 0], // 25
    [0, 11, 9, 7, 0],
];

static HOME_BOTTOM: [usize; 5] = [0, 14, 18, 22, 26];

static EMPTY: u8 = 0;
fn energy(p: u8) -> i64 {
    [0, 1, 10, 100, 1000][p as usize]
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct State {
    r: [u8; 27],
}

impl State {
    fn is_done(&self) -> bool {
        self.r
            .iter()
            .enumerate()
            .all(|(i, p)| *p == EMPTY || is_home(i, *p))
    }

    fn h(&self) -> i64 {
        // 0
        let mut home_sum = self
            .r
            .iter()
            .cloned()
            .enumerate()
            .map(|(i, p)| energy(p) * HOME_DIST[i][p as usize])
            .sum();
        let mut home_count = [0; 5];
        for (i, p) in self.r.iter().cloned().enumerate() {
            if is_home(i, p) {
                home_count[p as usize] += 1;
            }
        }
        for (p, hc) in home_count.iter().cloned().enumerate() {
            let nh = 4 - hc;
            home_sum += energy(p as u8) * (nh - 1) * nh / 2;
        }
        home_sum
    }

    fn is_done_each(self) -> [bool; 27] {
        let mut out = [false; 27];
        for bot in [14, 18, 22, 26] {
            for i in (bot - 3..=bot).rev() {
                let p = self.r[i];
                if is_home(i, p) {
                    out[i] = true;
                } else {
                    break;
                }
            }
        }
        out
    }

    fn n(self) -> Vec<(i64, State)> {
        let mut n = vec![];
        let is_done_each = self.is_done_each();
        for (i, p) in self.r.iter().cloned().enumerate() {
            if p == EMPTY || is_done_each[i] {
                continue;
            }
            if i <= 10 {
                let mut is_path_home_free = false;
                for (_cost, j) in self.dfs(i, p) {
                    if is_home(j, p) {
                        is_path_home_free = true;
                        break;
                    }
                }
                if is_path_home_free {
                    let hb = HOME_BOTTOM[p as usize];
                    for (home_pos_bot, j) in (hb - 3..=hb).rev().enumerate() {
                        if self.r[j] == EMPTY {
                            let cost =
                                energy(p) * (HOME_DIST[i][p as usize] + (3 - home_pos_bot as i64));
                            let mut s_n = self;
                            s_n.r[i] = EMPTY;
                            s_n.r[j] = p;
                            n.push((cost, s_n));
                            break;
                        } else if !is_home(j, self.r[j]) {
                            break;
                        }
                    }
                }
            } else {
                for (cost, j) in self.dfs(i, p) {
                    if j <= 10 && j != 2 && j != 4 && j != 6 && j != 8 {
                        let mut s_n = self;
                        s_n.r[i] = EMPTY;
                        s_n.r[j] = p;
                        n.push((cost, s_n));
                    }
                }
            }
        }
        n
    }
    fn dfs(self, i: usize, p: u8) -> Vec<(i64, usize)> {
        let mut stack = vec![(0, i)];
        let mut is_visited = self.r.map(|p| p != EMPTY);
        let mut out = vec![];
        while let Some((cost, u)) = stack.pop() {
            for v in N[u].iter().cloned() {
                if !is_visited[v] {
                    stack.push((cost + energy(p), v));
                    out.push((cost + energy(p), v));
                    is_visited[v] = true;
                }
            }
        }
        out
    }
}

fn astar(start: State) -> (i64, Vec<State>) {
    let mut prev = BTreeMap::<State, State>::new();
    let mut cost = BTreeMap::<State, i64>::new();
    let mut heap = BTreeSet::<(i64, i64, State)>::new();
    let mut is_closed = BTreeSet::<State>::new();

    heap.insert((start.h(), 0, start));
    cost.insert(start, start.h());

    let mut end: Option<(i64, State)> = None;

    while !heap.is_empty() {
        let e = *heap.iter().next().unwrap();
        heap.remove(&e);
        let (u_f, u_g, u) = e;

        assert!(!is_closed.contains(&u), "{}", u);
        assert_eq!(u_f, u_g + u.h());
        assert_eq!(u_f, *cost.get(&u).unwrap());
        is_closed.insert(u);

        // while u_g >= print_cost {
        //     // print!("{}", (print_cost / 1000) % 10);
        //     std::io::stdout().flush().unwrap();
        //     print_cost += 1000;
        // }

        if is_closed.len() % 10000 == 0 {
            eprintln!("{} {} {} {}/{}", u, u_f, u_g, heap.len(), is_closed.len());
        }

        if u.is_done() {
            assert_eq!(0, u.h());
            end = Some((u_g, u));
            break;
        }

        // dbg!(u.n().len());

        for (c, v) in u.n() {
            if is_closed.contains(&v) {
                continue;
            }
            let v_g = c + u_g;
            let v_h = v.h();
            let v_f = v_g + v_h;
            let prev_v_f = cost.get(&v).cloned().unwrap_or(i64::MAX / 4);
            if v_f < prev_v_f {
                heap.remove(&(prev_v_f, prev_v_f - v_h, v));
                heap.insert((v_f, v_g, v));
                cost.insert(v, v_f);
                prev.insert(v, u);
            }
        }
        // return (-1, vec![]);
    }

    dbg!(is_closed.len());

    let (total_cost, mut u) = end.unwrap();
    let mut path = vec![u];
    while let Some(v) = prev.get(&u) {
        path.push(*v);
        u = *v;
    }
    path.reverse();
    (total_cost, path)
}

fn main() {
    // #############
    // #...........#
    // ###B#C#B#D###
    //   #D#C#B#A#
    //   #D#B#A#C#
    //   #A#D#C#A#
    //   #########
    // let start_state = State {
    //     r: [
    //         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 4, 4, 1, 3, 3, 2, 4, 2, 2, 1, 3, 4, 1, 3, 1,
    //     ],
    // };
    let start_state = State {
        r: [
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 4, 4, 3, 2, 3, 2, 3, 4, 2, 1, 1, 4, 1, 3, 1,
        ],
    };

    let (cost, path) = astar(start_state);
    for s in path {
        println!("{}", s);
        println!();
    }
    dbg!(cost);
}

// #[test]
// fn test_square_usize() {
//     for (i, sq) in SQUARES.iter().enumerate() {
//         assert_eq!(i, sq.to_usize());
//     }
// }
// #[test]
// fn test_d_sym() {
//     for (a, b) in SQUARES.iter().cartesian_product(SQUARES.iter()) {
//         assert_eq!(d(*a, *b), d(*b, *a));
//     }
// }
// #[test]
// fn test_energy() {
//     assert_eq!(1, energy(0));
//     assert_eq!(1, energy(1));
//     assert_eq!(10, energy(4));
//     assert_eq!(100, energy(11));
//     assert_eq!(1000, energy(13));
// }

impl std::fmt::Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let chars = self.r.map(|p| if p == 0 { b' ' } else { b'A' + p - 1 });

        writeln!(f, "·············")?;

        write!(f, "·")?;
        write!(f, "{}", String::from_utf8(chars[..11].to_vec()).unwrap())?;
        writeln!(f, "·")?;

        for r in 0..4 {
            writeln!(
                f,
                "···{}·{}·{}·{}···",
                chars[11 + r] as char,
                chars[15 + r] as char,
                chars[19 + r] as char,
                chars[23 + r] as char,
            )?;
        }
        write!(f, "  ·········  ")
    }
}

#[test]
fn test_is_home() {
    assert!(is_home(0, 0));
    assert!(is_home(1, 0));
    assert!(is_home(2, 0));
    assert!(is_home(3, 0));
    assert!(is_home(4, 0));
    assert!(is_home(5, 0));
    assert!(is_home(6, 0));
    assert!(is_home(7, 0));
    assert!(is_home(8, 0));
    assert!(is_home(9, 0));
    assert!(is_home(10, 0));

    assert!(is_home(11, 1));
    assert!(is_home(12, 1));
    assert!(is_home(13, 1));
    assert!(is_home(14, 1));

    assert!(is_home(15, 2));
    assert!(is_home(16, 2));
    assert!(is_home(17, 2));
    assert!(is_home(18, 2));

    assert!(is_home(19, 3));
    assert!(is_home(20, 3));
    assert!(is_home(21, 3));
    assert!(is_home(22, 3));

    assert!(is_home(23, 4));
    assert!(is_home(24, 4));
    assert!(is_home(25, 4));
    assert!(is_home(26, 4));
}
