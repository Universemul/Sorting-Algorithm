fn heapify<T: Ord>(tab: &mut [T], root: usize, e: usize){
    let mut largest = root;
    let left = 2 * root + 1;
    let right = 2 * root + 2;

    if left < e && tab[root] < tab[left] {
        largest = left;
    }
    if right < e && tab[largest] < tab[right] {
        largest = right;
    }
    if tab[root] < tab[largest] {
        tab.swap(root, largest);
        heapify(tab, largest, e)
    }
}

pub fn sort<T: Ord>(tab: &mut [T]) -> () {
    let len = tab.len();
    if len <= 1 {
        return
    }
    // Put the heap in `max-heap order`
    for i in (0..len / 2).rev() {
        heapify(tab, i, len - 1);
    }
    
    // Tab is now max-heap ordered, so we juste want to sort the value
    for end in (1..len).rev() {
        // Swap the root with the last element.
        tab.swap(0, end);
        // Put the heap Put the heap in `max-heap order`
        heapify(tab, 0, end);
    }
    //heapify(tab, 0)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn heapsort_basic() {
        let mut tab = vec![4, 2, 3, 1, 5, 9, -1];
        sort(&mut tab);

        assert_eq!(tab, vec![-1, 1, 2, 3, 4, 5, 9]);
    }

     #[test]
    fn heapsort_duplicate() {
        let mut tab = vec![5, 2, 2, 2, 5, 5, 5];
        sort(&mut tab);

        assert_eq!(tab, vec![2, 2, 2, 5, 5, 5, 5]);
    }

    #[test]
    fn heapsort_empty() {
        let mut tab: Vec<i32> = vec![];
        sort(&mut tab);

        assert!(tab.is_empty());
    } 
}