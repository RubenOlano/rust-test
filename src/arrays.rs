use std::mem;
pub fn run() {
    let mut numbers: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{:?}", numbers);
    // Get single val
    println!("Single value: {}", numbers[0]);
    //Reassign val
    numbers[2] = 20;
    println!("{:?}", numbers);
    // Get Array length
    println!("Array Length: {}", numbers.len());
    // Arrays are stack allocated
    println!("This array occupies {} bytes", mem::size_of_val(&numbers));
    // Slice array
    let slice: &[i32] = &numbers[1..2];
    println!("Slice: {:?}", slice)
}
