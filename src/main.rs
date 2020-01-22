fn main() {
    let mut sum = 0;
    let n = 1000;
    let mut i = 1;
    while i < n {
        if i % 3 == 0 || i%5 == 0 {
            sum += i;
        }
        i+=1;
    }
    println!("Sum = {}", sum);
}
