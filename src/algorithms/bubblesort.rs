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

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn bubblesort_basic() {
        let mut tab = vec![4, 2, 3, 1, 5, 9, -1];
        sort(&mut tab);

        assert_eq!(tab, vec![-1, 1, 2, 3, 4, 5, 9]);
    }

     #[test]
    fn bubblesort_duplicate() {
        let mut tab = vec![5, 2, 2, 2, 5, 5, 5];
        sort(&mut tab);

        assert_eq!(tab, vec![2, 2, 2, 5, 5, 5, 5]);
    }

    #[test]
    fn bubblesort_empty() {
        let mut tab: Vec<i32> = vec![];
        sort(&mut tab);

        assert!(tab.is_empty());
    } 
}