use std::io;

fn main() {
    let mut numbers = Vec::new();
    println!("Please enter 5 numbers:");

    for _ in 0..5 {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let number: i32 = input.trim().parse().expect("Please enter a valid number");
        numbers.push(number);
    }

    bubble_sort(&mut numbers);

    println!("Sorted numbers: {:?}", numbers);
}

fn bubble_sort(arr: &mut Vec<i32>) {
    let n = arr.len();
    for i in 0..n {
        for j in 0..n-i-1 {
            if arr[j] > arr[j+1] {
                arr.swap(j, j+1);
            }
        }
    }
}
