/*
This function takes last as pivot and it is completely arbitrary :)
*/
fn _partition<T: Ord>(tab: &mut [T], left: isize, right: isize) -> isize {
    let pivot = right as usize;
    let mut j = right;
    let mut i = left;
    loop {
        // Left value are less than pivot, move to the right 
        while tab[i as usize] < tab[pivot as usize] {
            i += 1;
        }
        // Left value are greater than pivot, move to the left 
		while j > 0 && tab[j as usize] > tab[pivot as usize] {
            j -= 1;
        }
        // New pivot point
        if i >= j {
            break;
        } else {
            tab.swap(i as usize, j as usize);
        }
    }
    j
}


fn _sort<T: Ord>(tab: &mut [T], left: isize, right: isize) -> () {
    if left < right {
        let pi = _partition(tab, left, right);
        _sort(tab, left, pi - 1);
        _sort(tab, pi + 1, right)
    }
}

pub fn sort<T: Ord>(tab: &mut [T]) -> () {
    let len = tab.len() as isize;
    if len <= 1 {
        return
    }
    _sort(tab, 0, len - 1)
}