pub fn bubble_sort() {
    let mut arr: Vec<i32> = vec![9, 5, 6, 7, 8, 2, 4, 7];
    let mut swapped: bool = false;
    for i in 0 .. arr.len() {
            for i in 0 .. (arr.len() - 1) {
                let n = arr[i];
                if n > arr[i + 1] {
                    arr[i] = arr[i + 1];
                    arr[i + 1] = n;
                    swapped = true;
                }
            }
            if swapped == false {
                break;
            }
        }
    println!("{:?}", arr)
}