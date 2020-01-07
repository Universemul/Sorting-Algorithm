pub mod algorithms;

fn main() {
    let mut data: Vec<i32> = vec![4, 2, 3, 1, 5, 9, -1];
    //algorithms::bubble::sort(&mut data);
    //algorithms::selection::sort(&mut data);
    //algorithms::insertion::sort(&mut data);
    println!("{:?}", data);
    algorithms::heapsort::sort(&mut data);
    println!("{:?}", data);
}
