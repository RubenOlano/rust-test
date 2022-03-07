pub fn run() {
    //Print to console
    println!("Hello from print.rs");
    //Basic Formatting
    println!("Number {}", 1);
    //Positional Arguments
    println!(
        "{0} is from {1} and {0} likes to {2}",
        "Rub", "Peru", "code"
    );
    //Named arguments
    println!(
        "{name} likes to play {activity}",
        name = "Gadu",
        activity = "Game"
    );
    //Placeholder traits
    println!("Binary: {:b} Hex: {:x} Octo: {:o}", 10, 10, 10);

    //Placehodler for debug trait
    println!("{:?}", (12, true, "hello"));

    //Basic math
    println!("10 + 10 = {}", 10 + 10)
}
