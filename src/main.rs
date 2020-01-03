pub mod algorithms;

fn main() {
    let data: Vec<i32> = vec![4, 2, 3, 1, 5];
    algorithms::bubble::sort(&mut data.clone());
    algorithms::selection::sort(&mut data.clone())
}
