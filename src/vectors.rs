use std::mem;
pub fn run() {
    let mut numbers: Vec<i32> = vec![1, 2, 3, 4, 5];
    println!("{:?}", numbers);
    // Get single val
    println!("Single value: {}", numbers[0]);
    //Reassign val
    numbers[2] = 20;
    println!("{:?}", numbers);
    // Get vector length
    println!("Vector Length: {}", numbers.len());
    // Arrays are stack allocated
    println!("This array occupies {} bytes", mem::size_of_val(&numbers));
    // Slice vector
    let slice: &[i32] = &numbers[1..2];
    println!("Slice: {:?}", slice);

    // Add on to vector
    numbers.push(6);
    println!("Push value: {:?}", numbers);
    numbers.pop();
    println!("Pop: {:?}", numbers);
    for num in numbers.iter() {
        println!("Number: {}", num)
    }

    //Loop and mutate values
    for x in numbers.iter_mut() {
        *x *= 2;
    }
    println!("Numbers vec: {:?}", numbers);
}
