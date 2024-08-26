extern "C" {
    fn spam();
    fn add(a: i32, b: i32) -> i32;
}

fn main() {
    unsafe { spam(); };
    let answer = unsafe { add(10, 32) };
    println!("{:}", answer);
}
