pub fn sort(tab: &mut Vec<i32>) -> () {
    let len = tab.len();
    if len == 0 {
        println!("Vec is empty");
        return
    }
    println!("[BUBBLE SORT] BEFORE {:?}", tab);
    for idx in 0..len {
        let mut has_swap = false;
        for j in 0..len-idx-1 {
            if tab[j] > tab[j+1] {
                tab.swap(j, j+1);
                has_swap = true;
            }
        }
        if !has_swap {
            break
        }
    }
    println!("[BUBBLE SORT] BEFORE {:?}", tab);
}