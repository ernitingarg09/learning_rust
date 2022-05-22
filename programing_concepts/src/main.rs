fn main() {
    variables();
    data_types();
    let ret = functions(4, true);
    println!("Returned value form function is {ret}");
    control_flow_if();
    control_flow_loop();
    control_flow_while();
    control_flow_for();
}

fn variables() {
    // ****VARIABLES****
    // let x = 5; //compilation error because variables are immutables by default.
    let mut x = 5;
    println!("The value entered is {}", x);

    x = 6;
    println!("Entered value has been changed to {}", x);

    // ****CONST****
    // const are always immutable (not only just bydefault)
    // const must be annotated by type.
    const HOURS_PER_MONTHS: u32 = 30 * 24;
    // or
    // const HOURS_PER_MONTHS = 720;
    println!("The constant value assigned is {}", HOURS_PER_MONTHS);

    // ****SHADOWING****
    let z = 6;
    let z = z + 1;
    println!("Current value is {}", z);

    {
        let z = z * 2;
        println!("Inner scope value is {}", z);
    }

    println!("Outer scope value is {}", z);

    // same variable but with different type
    let spaces = "   ";
    let spaces = spaces.len();
    println!("Length of spaces is {}", spaces);

    // let mut mut_spaces = "   ";
    // mut_spaces = mut_spaces.len(); // compilation error, even if variable is mutable but you can't assign a different type.
}

fn data_types() {
    // **** DATATYPE ****
    /* i8 u8
       i16 u16
       i3 (default) u32
       i64 u64
       i128 u128
       isize usize

       f32 f64(default) => only signed.
       bool
       char
    */

    let f = 3.0; // f64, default
    let f: f32 = 3.2; //f32

    let floored = 2 / 3; // Results in 0

    // bool is 1 byte in size
    // char is 4 bytes in size

    // **** TUPLES****
    // always fixed size, any type
    let tup = (50, 40.3, false, 'a');
    // or
    let tup: (u32, f64, bool, char) = (50, 40.3, false, 'a');
    println!("Tuples values are {} {} {} {}", tup.0, tup.1, tup.2, tup.3);

    let (a, b, c, d) = tup;
    println!("Tuples values are {} {} {} {}", a, b, c, d);

    // **** ARRAY ****
    // fixed size, same type, and memory on stack
    let arr = [3, 3, 3, 3, 3];
    println!(
        "Array values are {} {} {} {} {}",
        arr[0], arr[1], arr[2], arr[3], arr[4]
    );

    // or
    let arr: [u32; 5] = [3, 3, 3, 3, 3];
    println!(
        "Array values are {} {} {} {} {}",
        arr[0], arr[1], arr[2], arr[3], arr[4]
    );

    // or
    let arr = [3; 5]; // intial value; nElements
    println!(
        "Array values are {} {} {} {} {}",
        arr[0], arr[1], arr[2], arr[3], arr[4]
    );
}

fn functions(x: i32, y: bool) -> i32 {
    let x = x + 1;
    println!("Current value has been changed to {x}, with {y}");

    let x = x + 2;
    x
}

fn control_flow_if() {
    let condition = true;

    let z = if condition { 5 } else { 6 };
    println!("If expression result is {z}");

    // or
    let z = {
        if condition {
            5
        } else {
            6
        }
    };
    println!("If expression result is {z}");
}

fn control_flow_loop() {
    let mut i = 0;

    'outer: loop {
        let mut j = 0;
        loop {
            println!("loop value i={i}|j={j}");

            if j == 2 {
                break;
            }

            if i == 3 {
                break 'outer;
            }

            j += 1;
            i += 1;
        }
    }

    let mut z = 0;
    let result = loop {
        if z == 3 {
            break z * 5;
        }

        z = z + 1;
    };

    println!("Break returned the value {result}");
}

fn control_flow_while() {
    let mut i = 10;
    while i >= 0 {
        println!("While loop has number value {i}");
        i -= 1;
    }
}

fn control_flow_for() {
    let a = [2, 4, 5, 6, 8, 5];

    for element in a {
        println!("For loop has element {element}");
    }
}
