use itertools::Itertools;

fn main() {
    let mut square = [4, 9];
    let mut scores = [0, 0];
    let mut dice = (1..=100).cycle().tuples().map(|(a, b, c)| a + b + c);
    for (turn, player) in (0..=1).cycle().enumerate() {
        let roll = dice.next().unwrap();
        square[player] = (square[player] + roll - 1) % 10 + 1;
        scores[player] += square[player];
        println!("{:4} {} {:4} {:?} {:?}", turn, player, roll, square, scores);
        if scores[player] >= 1000 {
            dbg!(scores[1-player] * 3 * (turn + 1));
            break;
        }
    }
}
