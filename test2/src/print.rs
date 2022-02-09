pub fn run() {
    // print to console
    println!("hello from print.rs file");

    let name = "Érik";
    let n = 10;
    println!("Your name is {} and your year old is {}!", name, n);

    println!("{0} is from {1}. {1} is cool!", "Bread", "Steak");

    println!("Hi {name}! You are {n}.", name=name, n=n); // 19:02
}
