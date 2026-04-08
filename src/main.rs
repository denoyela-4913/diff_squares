// https://les-suites.fr/arithmetique/somme-des-n-premiers-carres.php
// Somme des carrés des n premiers entiers = n(n+1)(2n+1)/6
// Somme des carrés des n premiers entiers pairs   (de 2 à 2n)   = 2n(n+1)(2n+1)/3
// Somme des carrés des n premiers entiers impairs (de 1 à 2n-1) = n(2n-1)(2n+1)/3

pub fn square_of_sum(n: u32) -> u32 {
    //todo!("square of sum of 1...{n}")
    // let sum = n * (n + 1) / 2;
    let sum = n
        .checked_mul(n + 1)
        .expect(format!("overflow 1 in square_of_sum formula when multiplying {n} and {}", n + 1).as_str())
        / 2;
    // sum * sum
    sum.checked_mul(sum)
        .expect(format!("overflow 2 in square_of_sum formula when squaring the sum of {sum} for {n}").as_str())
}

pub fn sum_of_squares(n: u32) -> u32 {
    //todo!("sum of squares of 1...{n}")
    // let sum_squares = n * (n + 1) * (2 * n + 1) / 6;
    let sum_squares = n
        .checked_mul(n + 1)
        .expect(format!("overflow 1 in sum_of_squares formula when multiplying {n} and {}", n + 1).as_str())
        .checked_mul(2 * n + 1)
        .expect(format!("overflow 2 in sum_of_squares formula when multiplying by {}", 2 * n + 1).as_str())
        / 6;
    sum_squares
    /* n.checked_mul(n + 1)
        .and_then(|t| t.checked_mul(2 * n + 1))
        .expect("overflow in sum_of_squares formula")
        / 6 */

}

pub fn difference(n: u32) -> u32 {
    //todo!("difference between square of sum of 1...{n} and sum of squares of 1...{n}")
    square_of_sum(n) - sum_of_squares(n)
}
fn main() {
    println!("Hello, world! Diff of squares");
    let d = difference(100);
    println!("Difference of squares of 1...100 is {d}");
    let d = difference(300);
    println!("Difference of squares of 1...300 is {d}");
    let d = difference(10000);
    println!("Difference of squares of 1...10000 is {d}");
    let d = difference(100_000_000);
    println!("Difference of squares of 1...100_000_000 is {d}");
}
