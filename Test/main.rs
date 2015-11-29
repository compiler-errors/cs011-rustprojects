extern crate crossbeam;
use crossbeam::scope;

fn main() {
    let mut list = vec![12,26,64,82,47,78,52,86,24,32,49,50,86,94,53,38,16,74,02];
    println!("Before: {:?}", list);
    quicksort(&mut list);
    println!("After: {:?}", list);
}

fn quicksort(array: &mut [i32]) {
    let len = array.len();

    if len == 0 || len == 1 {
        return;
    } else if len == 2 {
        if array[0] > array[1] {
            array.swap(0, 1);
        }
        return;
    }

    let pivot = array[array.len() / 2];

    let mut left = 0;
    let mut right = array.len() - 1;

    while left <= right {
        while array[left] < pivot {
            left = left + 1;
        }
        while array[right] > pivot {
            right = right - 1;
        }
        if left <= right {
            array.swap(left, right);
            left = left + 1;
            right = right - 1;
        }
    }

    crossbeam::scope(|scope| {
        let (leftslice, rightslice) = array.split_at_mut(left);

        scope.spawn(move || {quicksort(leftslice);});
        scope.spawn(move || {quicksort(rightslice);});
    });
}
