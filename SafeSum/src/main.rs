fn sum_u32(numbers: &[u32]) -> Option<u32> {
    let mut sum = 0u32;
    for &number in numbers {
        match sum.checked_add(number) {
            Some(result) => sum = result,
            None => return None,
        }
    }
    Some(sum)
}

fn main() {
    let numbers = [5, 2, 3, 4, 5];
    match sum_u32(&numbers) {
        Some(sum) => println!("Sum: {}", sum),
        None => println!("Overflow!"),
    }

    let numbers = [u32::MAX, 1];
    match sum_u32(&numbers) {
        Some(sum) => println!("Sum: {}", sum),
        None => println!("Overflow!"),
    }
}
