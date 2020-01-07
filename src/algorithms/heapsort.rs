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