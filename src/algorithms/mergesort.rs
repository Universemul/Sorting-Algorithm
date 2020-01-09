fn merge<T: Ord + Copy>(left: &[T], right: &[T], result: &mut [T]) {
    let mut i = 0;
    let mut j = 0;
    let mut idx = 0;

    // Compare element from left and right and insert into the temporary vec
    while i < left.len() && j < right.len() {
        if left[i] <= right[j] {
            result[idx] = left[i];
            i += 1;
        } else {
            result[idx] = right[j];
            j += 1;
        }
        idx += 1;
    }
    if i < left.len() {
        result[idx..].copy_from_slice(&left[i..]);
    }
    if j < right.len() {
        result[idx..].copy_from_slice(&right[j..]);
    }
    
}

fn _sort<T: Ord + Copy>(tab: &mut [T]) -> () {

    let middle = tab.len() / 2;
    if middle == 0 {
        return
    }
    _sort(&mut tab[..middle]);
    _sort(&mut tab[middle..]);
    let mut ret = tab.to_vec();
    merge(&tab[..middle], &tab[middle..], &mut ret);
    tab.copy_from_slice(&ret);
}

pub fn sort<T: Ord + Copy>(tab: &mut [T]) -> (){
    let len = tab.len();
    if len <= 1 {
        return
    }
    _sort(tab)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn mergesort_basic() {
        let mut tab = vec![4, 2, 3, 1, 5, 9, -1];
        sort(&mut tab);

        assert_eq!(tab, vec![-1, 1, 2, 3, 4, 5, 9]);
    }

     #[test]
    fn mergesort_duplicate() {
        let mut tab = vec![5, 2, 2, 2, 5, 5, 5];
        sort(&mut tab);

        assert_eq!(tab, vec![2, 2, 2, 5, 5, 5, 5]);
    }

    #[test]
    fn mergesort_empty() {
        let mut tab: Vec<i32> = vec![];
        sort(&mut tab);

        assert!(tab.is_empty());
    } 
}