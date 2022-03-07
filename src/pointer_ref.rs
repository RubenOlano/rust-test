pub fn run() {
    // Primitave
    let array_1 = [1, 2, 3];
    let _array_2 = array_1;
    // Vectors
    let vec_1 = vec![1, 2, 3];
    let vec_2 = &vec_1;

    println!("values: {:?}", (&vec_1, vec_2))
}
