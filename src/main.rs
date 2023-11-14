//--------------------------------------------------------
// Bubble Sort
//--------------------------------------------------------
fn bubble_sort<T: Ord>(arr: &mut [T]) {
    let len = arr.len();
    for i in 0..len {
        for j in 0..len -1 - i {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }
}

//--------------------------------------------------------
// Merge Sort
//--------------------------------------------------------
fn merge_sort<T: Ord + Clone>(arr: &mut [T]) {
    let len = arr.len();
    if len <= 1 {
        return;
    }

    let mid = len / 2;
    let mut left = arr[0..mid].to_vec();
    let mut right = arr[mid..].to_vec();

    merge_sort(&mut left);
    merge_sort(&mut right);

    let (mut i, mut j, mut k) = (0, 0, 0);

    while i < left.len() && j < right.len() {
        if left[i] <= right[j] {
            arr[k] = left[i].clone();
            i += 1;
        } else {
            arr[k] = right[j].clone();
            j += 1;
        }
        k += 1;
    }

    while i < left.len() {
        arr[k] = left[i].clone();
        i += 1;
        k += 1;
    }

    while i < right.len() {
        arr[k] = right[i].clone();
        i += 1;
        k += 1;
    }
}

//--------------------------------------------------------
// Quick Sort
//--------------------------------------------------------
fn quick_sort<T: Ord>(arr: &mut [T]) {
    let len = arr.len();
    if len <= 1 {
        return;
    }

    let pivot_index = partition(arr);
    quick_sort(&mut arr[0..pivot_index]);
    quick_sort(&mut arr[pivot_index + 1..]);
}

fn partition<T: Ord>(arr: &mut [T]) -> usize { 
    let pivot_index = arr.len() / 2;
    arr.swap(pivot_index, arr.len() - 1);

    let mut i = 0;
    for j in 0..arr.len() - 1 {
        if arr[j] <= arr[arr.len() - 1] {
            arr.swap(i, j);
            i += 1;
        }
    }

    arr.swap(i, arr.len() - 1);
    i
}


fn main() {
    let mut array1 = vec![64, 64, 25, 12, 22, 11];
    bubble_sort(&mut array1);
    println!("Sorted array with bubble sort: {:?}", array1);

    let mut array2 = vec![64, 64, 25, 12, 22, 11];
    merge_sort(&mut array2);
    println!("Sorted array with merge sort: {:?}", array2);

    let mut array3 = vec![64, 64, 25, 12, 22, 11];
    quick_sort(&mut array3);
    println!("Sorted array with quick sort: {:?}", array3);
}
