pub fn sort<T: Ord>(tab: &mut [T]) -> () {
    let len = tab.len();
    if len <= 1 {
        return
    }
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
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn insertion_basic() {
        let mut tab = vec![4, 2, 3, 1, 5, 9, -1];
        sort(&mut tab);

        assert_eq!(tab, vec![-1, 1, 2, 3, 4, 5, 9]);
    }

     #[test]
    fn insertion_duplicate() {
        let mut tab = vec![5, 2, 2, 2, 5, 5, 5];
        sort(&mut tab);

        assert_eq!(tab, vec![2, 2, 2, 5, 5, 5, 5]);
    }

    #[test]
    fn insertion_empty() {
        let mut tab: Vec<i32> = vec![];
        sort(&mut tab);

        assert!(tab.is_empty());
    } 
}