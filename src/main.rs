
fn quick_sort_nth_element(arr: &mut [i32], left: usize, right: usize, k: usize) -> i32 {
    if left == right {
        return arr[left];
    }

    let pivot_index = partition(arr, left, right);
    if k == pivot_index {
        return arr[k];
    } else if k < pivot_index {
        quick_sort_nth_element(arr, left, pivot_index - 1, k)
    } else {
        quick_sort_nth_element(arr, pivot_index + 1, right, k)
    }
}

fn partition(arr: &mut [i32], left: usize, right: usize) -> usize {
    let pivot = arr[right];
    let mut i = left;
    for j in left..right {
        if arr[j] < pivot {
            arr.swap(i, j);
            i += 1;
        }
    }
    arr.swap(i, right);
    i
}


fn main() {
    let mut arr = [3, 2, 1, 5, 4];
    let right = arr.len() - 1;
    let k = std::env::args().nth(1).unwrap().parse::<usize>().unwrap();
    if k > right {
        println!("k is out of bounds");
        return;
    }
    else if k == 0 {
        println!("k cannot be 0");
        return;
    }
    else{
        let result = quick_sort_nth_element(&mut arr, 0, right, k-1);
        println!("The {}th smallest element is: {}", k, result);
        let result2 = quick_sort_nth_element(&mut arr, 0, right, right - k + 1 );
        println!("The {}th largest element is: {}", k, result2);
    }
}
