pub fn sort<T: Ord>(tab: &mut [T]) -> () {
    let len = tab.len();
    if len <= 1 {
        return
    }
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
}