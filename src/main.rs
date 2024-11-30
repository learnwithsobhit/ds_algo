use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Array of numbers
    #[arg(long, value_delimiter = ',')]
    arr: Vec<i32>,

    /// K value
    #[arg(short, long)]
    k: usize,
}

fn quick_sort_nth_element(arr: &mut [i32], left: usize, right: usize, k: usize) -> i32 {
    if left == right {
        return arr[left];
    }

    let pivot_index = partition(arr, left, right);
    match k.cmp(&pivot_index) {
        std::cmp::Ordering::Equal => arr[k],
        std::cmp::Ordering::Less => quick_sort_nth_element(arr, left, pivot_index - 1, k),
        std::cmp::Ordering::Greater => quick_sort_nth_element(arr, pivot_index + 1, right, k),
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
    let args = Args::parse();
    let mut arr = args.arr;
    let k = args.k;

    if arr.is_empty() {
        println!("Please provide an array using --arr");
        return;
    }

    let right = arr.len() - 1;
    if k > right {
        println!("k is out of bounds");
    } else if k == 0 {
        println!("k cannot be 0");
    } else {
        let result = quick_sort_nth_element(&mut arr, 0, right, k - 1);
        println!("The {}th smallest element is: {}", k, result);
        let result2 = quick_sort_nth_element(&mut arr, 0, right, right - k + 1);
        println!("The {}th largest element is: {}", k, result2);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quick_sort_nth_element() {
        let mut arr = [3, 2, 1, 5, 4];
        let len = arr.len();
        let result = quick_sort_nth_element(&mut arr, 0, len - 1, 2);
        assert_eq!(result, 3);
    }

    #[test]
    fn test_quick_sort_nth_element_large() {
        let mut arr = [3, 2, 1, 5, 4, 6, 7, 8, 9, 10];
        let len = arr.len();
        let result = quick_sort_nth_element(&mut arr, 0, len - 1, 2);
        assert_eq!(result, 3);
    }

    #[test]
    fn test_quick_sort_nth_element_small() {
        let mut arr = [3, 2, 1];
        let len = arr.len();
        let result = quick_sort_nth_element(&mut arr, 0, len - 1, 2);
        assert_eq!(result, 3);
    }

    #[test]
    fn test_first_element() {
        let mut arr = [5, 2, 3, 1, 4];
        let len = arr.len();
        let result = quick_sort_nth_element(&mut arr, 0, len - 1, 0);
        assert_eq!(result, 1); // Should find smallest element
    }

    #[test]
    fn test_last_element() {
        let mut arr = [5, 2, 3, 1, 4];
        let len = arr.len();
        let result = quick_sort_nth_element(&mut arr, 0, len - 1, len - 1);
        assert_eq!(result, 5); // Should find largest element
    }

    #[test]
    fn test_duplicate_elements() {
        let mut arr = [3, 3, 3, 3, 3];
        let len = arr.len();
        let result = quick_sort_nth_element(&mut arr, 0, len - 1, 2);
        assert_eq!(result, 3);
    }

    #[test]
    fn test_negative_numbers() {
        let mut arr = [-5, -2, -10, -1, -8];
        let len = arr.len();
        let result = quick_sort_nth_element(&mut arr, 0, len - 1, 2);
        assert_eq!(result, -5);
    }

    #[test]
    fn test_mixed_numbers() {
        let mut arr = [-5, 2, 0, -1, 8];
        let len = arr.len();
        let result = quick_sort_nth_element(&mut arr, 0, len - 1, 2);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_single_element() {
        let mut arr = [42];
        let result = quick_sort_nth_element(&mut arr, 0, 0, 0);
        assert_eq!(result, 42);
    }

    #[test]
    fn test_two_elements() {
        let mut arr = [42, 17];
        let result = quick_sort_nth_element(&mut arr, 0, 1, 0);
        assert_eq!(result, 17);
        let result = quick_sort_nth_element(&mut arr, 0, 1, 1);
        assert_eq!(result, 42);
    }

    #[test]
    #[should_panic]
    fn test_empty_array() {
        let mut arr: [i32; 0] = [];
        quick_sort_nth_element(&mut arr, 0, 0, 0);
    }
}
