fn basic() {
    println!("Hello, world!");
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1

    // remainder
    let remainder = 43 % 5;
    
    // inference
    let x = 2.0; // f64
    
    // explicit
    let y: f32 = 3.0; // f32
    
    let tup: (i32, f64, u8) = (500, 6.4, 1); //son they got tuples #python
    
    let (x, y, z) = tup; //take apart

    let tupx = tup.0;
    let tupy = tup.1;

    // arrays the same (for inference)
    
    let a: [i32; 5] = [1, 2, 3, 4, 5];

    let a = [3; 5]; //repeated 3s
    
    example_func(25);
}

fn example_func(x: i32) -> i32 { //the arrow means it type it returns
    let y = {
        let x = 3;
        x + 1 // no semi-colon cuz its the return
    };

    if y < 5 {
        println!("smth");
    }
    else if y < 10 {
        println!("smth else");
    }

    let foo = if y < 5 {6} else {7};

    loop { //practically a while(1)
        // ^ this can be used and can use a break to define a veriable after exit
    }

    // can break to outer look via 'loop_name: loop {} style. -> break 'loop_name;
    
    //python ahh:
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }

    //countdown
    for number in (1..4).rev() {
        println!("{number}!");
    }
}

// basic syntax define


// ownership understanding

fn owner() {
    // String literals are usually  immutable
    let a = "hello";
    // but
    let mut s = String::from("hello"); // is mutable
    s.push_str(", world!");
    println!("{s}");

    // The following is invalid syntax due to ownership
    let s1 = String::from("hello!");
    let s2 = s1;

    println!("{s1}, world!");
    // This won't work because ownership has been transferred to s2
    // The way the strings work here is that they both point to the same
    // place in memory, but to avoid double free when they leave scope, only
    // s2 will free the memory

    // If above I re-assign s1 to a nother string, "hello" will be free

    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {s1}, s2 = {s2}");
    //Deep copy

    // all this deep and shallow copy does not matter for datatype with
    // known sttatic sizes like ints
    
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{s2}' is {len}.");
}
// instead of making both go through, teh cpp pass by refernce works here too
fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}

//borrrowing only allowed once
fn dontwork() {
    let mut s = String::from("hello");

    let r1 = &mut s;
    let r2 = &mut s;

    println!("{r1}, {r2}");
}

fn slicer() {
    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11]; // #evil macro question
    // works for like most data types and arrays and whatnot
}

// structs and stuff ~ work the same ish

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username: username,
        email: email,
        sign_in_count: 1,
    }
}

struct AlwaysEqual;

fn structs() {
    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    }; // all other fields are copy of user1

    let subject = AlwaysEqual;

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

}

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    } // method funcs
    
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

// rust has no equivalent -> btw (its automatic) :o


// Finished ch 4-5 (start at 6)

fn main() {

}
