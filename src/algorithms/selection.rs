pub fn sort(tab: &mut Vec<i32>) -> () {
    let len = tab.len();
    if len == 0 {
        println!("Vec is empty");
        return
    }
    println!("[SELECTION SORT] BEFORE {:?}", tab);
    for idx in 0..len-1 {
        let slice = &tab[idx+1 .. len];
        // We select only a part of the main vec.
        // we iterate on the slice with enumerate to get both index and value
        // `Fold` is used for apply the "min predicate". We store the current element of `slice` in the accumulator.
        let min = slice.iter().enumerate().fold((0, slice.first().unwrap()), |acc, it| { if it.1 < acc.1 {it} else {acc}});
        let s_index = {
            if min.1 < &tab[idx] {
                idx + min.0 + 1
            } else {idx}}
        ;
        tab.swap(idx, s_index);
    }
    println!("[SELECTION SORT] AFTER {:?}", tab)
}