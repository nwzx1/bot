fn main() {
    let rng: i32 = 5;

    for _j in 1..rng {
        for _i in 1.._j + 1 {
            print!("//\\\\");
        }
        println!();
    }
}
