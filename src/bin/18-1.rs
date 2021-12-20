use nom::{
    branch::alt, bytes::complete::tag, character::complete::multispace1, character::complete::u64,
    multi::separated_list1, IResult,
};
// use itertools::Itertools;

#[derive(Debug)]
enum S {
    Lit(u64),
    Pair(Box<S>, Box<S>),
}

#[derive(Debug, Clone, Copy)]
enum T {
    L,
    N(u64),
    R,
}

fn explode(ts: &mut Vec<T>) -> bool {
    let mut depth = 0;
    for i in 0..ts.len() {
        if depth == 5 {
            if let (T::N(l), T::N(r)) = (ts[i], ts[i + 1]) {
                for t in ts[..i - 1].iter_mut().rev() {
                    if let T::N(x) = t {
                        *t = T::N(*x + l);
                        break;
                    }
                }
                for t in ts[i + 2..].iter_mut() {
                    if let T::N(x) = t {
                        *t = T::N(*x + r);
                        break;
                    }
                }
                ts.splice(i - 1..=i + 2, [T::N(0)]);
                return true;
            } else {
                panic!("expected pair: i={} ts={:?}", i, ts)
            }
        }
        match ts[i] {
            T::L => depth += 1,
            T::R => depth -= 1,
            T::N(_) => {}
        }
    }
    false
}

fn split(ts: &mut Vec<T>) -> bool {
    for i in 0..ts.len() {
        match ts[i] {
            T::N(x) if x >= 10 => {
                let l = x / 2;
                let r = x - l;
                ts.splice(i..i + 1, [T::L, T::N(l), T::N(r), T::R]);
                return true;
            }
            _ => {}
        }
    }
    false
}

fn reduce(ts: &mut Vec<T>) {
    while explode(ts) || split(ts) {}
}

fn add(x: &[T], y: &[T]) -> Vec<T> {
    let mut sum = vec![T::L];
    sum.extend(x);
    sum.extend(y);
    sum.push(T::R);
    reduce(&mut sum);
    sum
}

impl S {
    fn magnitude(&self) -> u64 {
        match self {
            Self::Lit(x) => *x,
            Self::Pair(l, r) => 3 * l.magnitude() + 2 * r.magnitude(),
        }
    }
    fn to_list(&self, ts: &mut Vec<T>) {
        match self {
            Self::Lit(x) => ts.push(T::N(*x)),
            Self::Pair(l, r) => {
                ts.push(T::L);
                l.to_list(ts);
                r.to_list(ts);
                ts.push(T::R);
            }
        }
    }
    fn from_list(ts: &[T]) -> (&[T], S) {
        match ts[0] {
            T::N(x) => (&ts[1..], S::Lit(x)),
            T::L => {
                let ts = &ts[1..];
                let (ts, l) = S::from_list(ts);
                let (ts, r) = S::from_list(ts);
                if let T::R = ts[0] {
                    (&ts[1..], S::Pair(l.into(), r.into()))
                } else {
                    panic!();
                }
            }
            _ => {
                panic!()
            }
        }
    }
}

impl std::ops::Add for S {
    type Output = Self;

    fn add(self, other: S) -> Self {
        S::Pair(Box::new(self), Box::new(other))
    }
}

fn lit(i: &[u8]) -> IResult<&[u8], S> {
    let (i, lit) = u64(i)?;
    Ok((i, S::Lit(lit)))
}

fn pair(i: &[u8]) -> IResult<&[u8], S> {
    let (i, _) = tag("[")(i)?;
    let (i, left) = snail(i)?;
    let (i, _) = tag(",")(i)?;
    let (i, right) = snail(i)?;
    let (i, _) = tag("]")(i)?;
    Ok((i, S::Pair(Box::new(left), Box::new(right))))
}

fn snail(i: &[u8]) -> IResult<&[u8], S> {
    alt((lit, pair))(i)
}

fn snails(i: &[u8]) -> IResult<&[u8], Vec<S>> {
    separated_list1(multispace1, snail)(i)
}

fn parse(i: &[u8]) -> Vec<S> {
    snails(i).unwrap().1
}

fn add_snails(ss: &[S]) -> S {
    let mut ts_sum = vec![];
    ss[0].to_list(&mut ts_sum);
    for s in &ss[1..] {
        let mut ts = vec![];
        s.to_list(&mut ts);
        ts_sum = add(&ts_sum, &ts);
    }
    S::from_list(&ts_sum).1
}

fn solve(i: &[u8]) -> u64 {
    add_snails(&parse(i)).magnitude()
}

fn main() {
    let input = include_bytes!("../../inputs/18.txt");
    dbg!(solve(input));
}

#[cfg(test)]
fn ts_str(ts: &[T]) -> String {
    S::from_list(ts).1.to_string()
}

#[test]
fn test_explode() {
    fn ex(i: &str) -> String {
        let s = snail(i.as_bytes()).unwrap().1;
        let mut ts = vec![];
        s.to_list(&mut ts);
        explode(&mut ts);
        ts_str(&ts)
    }
    assert_eq!("[[[[0,9],2],3],4]", ex("[[[[[9,8],1],2],3],4]"));
    assert_eq!("[7,[6,[5,[7,0]]]]", ex("[7,[6,[5,[4,[3,2]]]]]"));
    assert_eq!("[[6,[5,[7,0]]],3]", ex("[[6,[5,[4,[3,2]]]],1]"));
    assert_eq!(
        "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]",
        ex("[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]")
    );
    assert_eq!(
        "[[3,[2,[8,0]]],[9,[5,[7,0]]]]",
        ex("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]")
    );
}

#[test]
fn test_reduce() {
    let s = snail(b"[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]").unwrap().1;
    let mut ts = vec![];
    s.to_list(&mut ts);
    let mut ts2 = ts.clone();
    explode(&mut ts);
    assert_eq!("[[[[0,7],4],[7,[[8,4],9]]],[1,1]]", ts_str(&ts));
    explode(&mut ts);
    assert_eq!("[[[[0,7],4],[15,[0,13]]],[1,1]]", ts_str(&ts));
    split(&mut ts);
    assert_eq!("[[[[0,7],4],[[7,8],[0,13]]],[1,1]]", ts_str(&ts));
    split(&mut ts);
    assert_eq!("[[[[0,7],4],[[7,8],[0,[6,7]]]],[1,1]]", ts_str(&ts));
    reduce(&mut ts);
    assert_eq!("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]", ts_str(&ts));

    reduce(&mut ts2);
    assert_eq!("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]", ts_str(&ts2));
}

#[test]
fn test_add() {
    assert_eq!(
        "[[[[5,0],[7,4]],[5,5]],[6,6]]",
        add_snails(&parse(b"[1,1]\n[2,2]\n[3,3]\n[4,4]\n[5,5]\n[6,6]")).to_string()
    );
}

#[test]
fn test_solve() {
    assert_eq!(solve(b"[[1,2],[[3,4],5]]"), 143);
    assert_eq!(solve(b"[[[[0,7],4],[[7,8],[6,0]]],[8,1]]"), 1384);
    assert_eq!(solve(b"[[[[1,1],[2,2]],[3,3]],[4,4]]"), 445);
    assert_eq!(solve(b"[[[[3,0],[5,3]],[4,4]],[5,5]]"), 791);
    assert_eq!(solve(b"[[[[5,0],[7,4]],[5,5]],[6,6]]"), 1137);
    assert_eq!(
        solve(b"[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]"),
        3488
    );
}

impl std::fmt::Display for S {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            S::Lit(ref i) => write!(f, "{}", i),
            S::Pair(l, r) => {
                write!(f, "[")?;
                l.fmt(f)?;
                write!(f, ",")?;
                r.fmt(f)?;
                write!(f, "]")
            }
        }
    }
}
