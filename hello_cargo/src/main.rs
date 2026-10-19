fn main() {
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
        // ^ this can be used and can use a break to defice a veriable after exit
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


// left off ch3 (start at 4)
