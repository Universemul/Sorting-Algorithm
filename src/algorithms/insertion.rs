pub fn sort(tab: &mut Vec<i32>) -> () {
    let len = tab.len();
    if len == 0 {
        println!("Vec is empty");
        return
    }
    println!("[INSERTION SORT] BEFORE {:?}", tab);
    for idx in 1..len {
        let mut j = idx;
        /*
            We iterate in reverse (from idx to 0) and swap current `j` element until `j` is not < `j - 1`
        */
        while j > 0 && tab[j] < tab[j - 1] {
            tab.swap(j-1, j);
            j -= 1;
        }
    }
    println!("[INSERTION SORT] BEFORE {:?}", tab);
}