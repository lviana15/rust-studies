fn main() {
    let mut sum: i32 = 0;
    let mut x: i32 = 0;

    while x <= 1000 {
        if x % 3 == 0 || x % 5 == 0 {
            sum += x;
        }

        x += 1;
    } 

    println!("{}", sum);
}
