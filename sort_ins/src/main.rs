fn main() {
    let mut arr = [
        30, 5, 20, 17, 34, 51, 87, 49, 7, 57, 73, 47, 100, 59, 60, 97, 62, 87, 5, 68, 70, 33, 89,
        96, 14, 47, 42, 83, 55, 5, 77, 70, 86, 79, 52, 95, 26, 64, 25, 35, 25, 80, 9, 21, 42, 45,
        88, 23, 30, 56, 13, 82, 1, 35, 99, 60, 85, 29, 16, 19, 25, 15, 18, 98, 48, 8, 67, 2, 7, 75,
        88, 82, 97, 27, 14, 1, 25, 17, 61, 99, 18, 82, 1, 35, 96, 78, 9, 46, 41, 23, 18, 37, 25,
        74, 24, 39, 41, 94, 97, 16,
    ];

    insertion_sort(&mut arr);
    println!("{:?}", arr);
}

fn insertion_sort(arr: &mut [i32]) {
    let mut j: usize = 1;

    while j < arr.len() {
        let key = arr[j];
        let mut i: i32 = (j - 1) as i32;
        while i >= 0 && arr[i as usize] > key {
            arr[(i + 1) as usize] = arr[i as usize];
            i = i - 1;
        }
        arr[(i + 1) as usize] = key;
        j = j + 1;
    }
}

#[test]
fn test_insertion_sort() {
    let mut data = [3, 2, 1, 5, 4];
    insertion_sort(&mut data);
    assert_eq!(data, [1, 2, 3, 4, 5]);
}
