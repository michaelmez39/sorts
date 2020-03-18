use rayon::prelude::*;
use std::sync::atomic;
pub fn selection_sort<T: PartialOrd + Clone>(arr: &mut [T]) {
    for left in 0..arr.len() {
        let mut smallest = left;
        for current in left..arr.len() {
            if arr[current] < arr[smallest] {
                smallest = current;
            }
        }
        arr.swap(left, smallest);
    }
}

// in report
pub fn insertion_sort<T: PartialOrd + Clone>(arr: &mut [T]) {
    if arr.len() < 1 {
        return;
    }
    for i in 1..arr.len() {
        let val = arr[i].clone();
        let mut empty = 0;
        for j in (0..i).rev() {
            arr[j+1] = arr[j].clone();
            if arr[j] < val {
                empty = j + 1;
                break;
            }
        }
        arr[empty] = val; 
    }
}


// in report
pub fn merge_sort<T: PartialOrd + Clone>(arr: &mut [T]) {
    merge_sort_rec(arr, 0, arr.len()-1);
}

fn merge_sort_rec<T: PartialOrd + Clone>(arr: &mut [T], start: usize, end: usize){
    if start < end {
        let mid = (start+end) / 2;
        merge_sort_rec(arr, start, mid);
        merge_sort_rec(arr, mid + 1, end);
        let mut a = Vec::new();
        let mut b = Vec::new();
        let a_len = mid - start + 1;
        let b_len = end - mid;
        for i in 0..a_len {
            a.push(arr[start + i].clone());
        }
        for i in 0..b_len {
            b.push(arr[mid + 1 + i].clone());
        }

        let mut ac = 0;
        let mut bc = 0;
        let mut j = start;
        while ac < a_len && bc < b_len {
            if a[ac] <= b[bc] {
                arr[j] = a[ac].clone();
                ac += 1;
            } else {
                arr[j] = b[bc].clone();
                bc += 1;
            }
            j += 1;
        }
        while ac < a_len {
            arr[j] = a[ac].clone();
            j += 1;
            ac += 1;
        }
        while bc < b_len {
            arr[j] = b[bc].clone();
            j += 1;
            bc += 1;
        }
    }
}


// in report
fn heapify<T: PartialOrd + Clone>(arr: &mut [T], i: usize) {
    let mut largest = i;
    let length = arr.len();
    let left = 2 * i + 1;
    let right = 2 * i + 2;
    if left < length && arr[left] > arr[largest] {
        largest = 2*i+1
    } 
    if right < length && arr[right] > arr[largest] {
        largest = 2*i+2;
    };
    if largest != i {
        arr.swap(largest, i);
        heapify(arr, largest);
    }
}
pub fn heap_sort<T: PartialOrd + Clone>(arr: &mut [T]) {
    let length = arr.len();
    for i in (0..(arr.len() / 2)).rev() {
        heapify(arr, i);
    }
    for i in 0..arr.len() {
        arr.swap(0, length - 1 - i);
        heapify(&mut arr[0..length-i-1], 0);
    }
}


pub fn odd_even_sort<T: PartialOrd + Clone + Sync + Send>(arr: &mut [T]) {
    let sorted = std::sync::atomic::AtomicBool::from(false);
    let even_len = arr.len() - arr.len() % 2;
    let odd_len = if arr.len() % 2 == 1 { arr.len() } else { arr.len() - 1 };
    while !sorted.load(atomic::Ordering::SeqCst) {
        sorted.store(true, atomic::Ordering::SeqCst);
        let arrsl = arr.as_parallel_slice_mut();
        arrsl[0..even_len].as_parallel_slice_mut().par_chunks_mut(2).for_each(|chunk| {
            if chunk[0] > chunk[1] {
                sorted.store(false, atomic::Ordering::Relaxed);
                chunk.swap(0, 1);
            }
        });
        arrsl[1..odd_len].as_parallel_slice_mut().par_chunks_mut(2).for_each(|chunk| {
            if chunk[0] > chunk[1] {
                sorted.store(false, atomic::Ordering::Relaxed);
                chunk.swap(0, 1);
            }
        });
    }
}

pub fn bubble_sort<T: PartialOrd + Clone + Sync + Send>(arr: &mut [T]) {
    let mut sorted = false;
    let mut end = arr.len();
    while !sorted {
        sorted = true;
        end -= 1;
        for i in 0..end {
            if arr[i+1] < arr[i] {
                arr.swap(i+1, i);
                sorted = false;
            }
        }
        
    }
}