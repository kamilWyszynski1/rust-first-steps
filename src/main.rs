mod generics;

use std::collections::HashMap;

fn main() {
    generics::run();
}

// FIRST STEPS
// crab is a first every written function in Rust.
#[allow(dead_code)]
fn crab() {
    use ferris_says::say;
    use std::io::{stdout, BufWriter, Stdout};

    let stdout: Stdout = stdout();
    let message = String::from("Hello from here!");
    let width = message.chars().count();

    let mut writer = BufWriter::new(stdout.lock());
    say(message.as_bytes(), width, &mut writer).unwrap();
}

// 2. in rust docs
#[allow(dead_code)]
fn guessing_game() {
    use rand::Rng;
    use std::cmp::Ordering;
    use std::io;

    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..101);

    loop {
        println!("Please input your guess.");

        // let creates variable
        // variables are immutable by default
        //
        // String is dynamic string type, mutable
        // str e.g. let a = ""; is immutable sequence of characters
        //
        // ::new() is associated function of String type
        // associated function is implemented on TYPE rather that on INSTANCE == static method
        let mut guess = String::new(); // mutable

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // match is used to handle errors
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        // let guess: u32 = guess.trim().parse().expect("Failed to parse");

        println!("You guessed {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You won!");
                break;
            }
        };
    }
}

// 3.1
#[allow(dead_code)]
fn common_programming_concepts() {
    // VARIABLES
    // variables are immutable by default
    println!("=== VARIABLES ===");
    let x = 5;
    println!("The value of x is: {}", x);
    const Y: i32 = 10;
    println!("The const value is: {}", Y);

    let mut x = x + 2;
    println!("The value of x is: {}", x);
    x = 10;
    println!("The value of x is: {}", x);

    // shadowing the type
    let spaces = "   ";
    let spaces = spaces.len();
    println!("{}", spaces);

    // DATA TYPES
    // scalar types - single value int, floats, boolean, char
    // compound types - tuples, arrays

    println!("=== DATA TYPES ===");

    // tuples saves data on a heap
    let tup = (1, 2, 'a', "siema");
    println!("tup: {:?}", tup);

    // destructuring
    let (a, b, c, d) = tup;
    println!("a {}, b {}, c {}, d {}", a, b, c, d);
    // get first index
    println!("first element {}", tup.0);

    // arrays saves data on a stack
    // arrays are always fixed size
    let arr: [u8; 3] = [1, 2, 3];
    println!("arr {:?}", arr);

    let arr2 = [1; 10];
    println!("arr2 {:?}", arr2);
    println!("first item in arr2 {}", arr2[0]);

    // FUNCTIONS
    // Functions contains statements and expressions.

    // Statements are instructions that perform some action and do not return a value.
    // let y = 6; is a statement
    //
    // Expressions evaluate to a resulting value.

    println!("=== FUNCTIONS ===");

    // expression evaluates to 4
    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of y is: {}", y);

    fn five() -> i32 {
        5
    }
    let x = five();
    println!("The value of x is: {}", x);

    // fn plus_one(x: i32) -> i32 {
    //     x + 1; // missing return, we need to say 'return' or remove semicolon
    // }

    // CONTROL FLOW

    println!("=== CONTROL FLOW ===");

    // IF EXPRESSIONS
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    // Using if in a let Statement
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {}", number);

    // let number = if condition { 5 } else { "six" }; compiling error

    // LOOPS
    println!("=== LOOPS ===");

    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);

    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }

    println!("LIFTOFF!!!");

    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is: {}", element);
    }
    for el in a {
        println!("the value is: {}", el)
    }
}

// 4
#[allow(dead_code)]
#[allow(unused_variables)]
fn ownership() {
    // memory is managed through a system of ownership with a set of rules that the compiler checks at compile time

    // OWNERSHIP RULES
    // * Each value in Rust has a variable that’s called its owner.
    // * There can only be one owner at a time.
    // * When the owner goes out of scope, the value will be dropped.

    let a = "str stored in stack"; // a as simple data type it goes to a stack.
                                   // // String is being stored as we don't know exact size of that string, it can change over time.
    let b = String::from("String stored in heap");

    {
        let s = String::from("string withing that block");
    } // block ends here so s in freed now, drop function in executed.

    {
        let s = String::from("string withing that block");
        drop(s);
    } // this is the same as above.

    // Ways Variables and Data Interact: Move
    let s1 = String::from("s1");
    let s2 = s1; // move - compiler sees that s1 is no longer need, there won't be "double free"
    println!("s2: {}", s2);

    // Ways Variables and Data Interact: Clone
    let s1 = String::from("s1");
    let s2 = s1.clone(); // clone - creates new pointer in heap, it might be expensive
    println!("s1: {}, s2: {}", s1, s2);

    // Ownership and Functions
    fn test() {
        fn takes_ownership(some_string: String) {
            // some_string comes into scope
            println!("{}", some_string);
        }

        let s = String::new();
        takes_ownership(s); // s is freed  here
                            // println!("s in test: {}", s) // it will rise error
    }

    // Return Values and Scope
    fn return_values_and_scope() {
        let s1 = gives_ownership(); // gives_ownership moves its return
                                    // value into s1
        println!("{}", s1);

        let s2 = String::from("hello"); // s2 comes into scope

        let s3 = takes_and_gives_back(s2); // s2 is moved into
                                           // takes_and_gives_back, which also
                                           // moves its return value into s3
        println!("{}", s3);

        fn gives_ownership() -> String {
            // gives_ownership will move its
            // return value into the function
            // that calls it

            let some_string = String::from("hello"); // some_string comes into scope

            some_string // some_string is returned and
                        // moves out to the calling
                        // function
        }

        // takes_and_gives_back will take a String and return one
        fn takes_and_gives_back(a_string: String) -> String {
            // a_string comes into
            // scope

            a_string // a_string is returned and moves out to the calling function
        }
    }
}

// 4.1
fn references_and_borrowing() {
    // this will not work because of 'move'
    //
    // let s = String::from("elo");
    // let len = calculate_length(s);
    // println!("s: {}, len: {}", s,len)

    let s = String::from("elo");
    // borrowing
    let len = calculate_length_ref(&s);
    println!("s: {}, len: {}", s, len);

    // there can be only one mutable reference in given scope
    let mut s = String::from("elo");
    change_mut(&mut s);
    println!("after change s: {}", s);

    // THIS WON'T WORK
    let r1 = &s; // immutable ref
    let r2 = &s; // another one, it's ok
    let r3 = &mut s; // problem
                     // println!("{}, {}, and {}", r1, r2, r3);

    // THIS WILL WORK
    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);
    // r1 and r2 are no longer used after this point

    let r3 = &mut s; // no problem
    println!("{}", r3);

    // Dangling References
    // fn dangle() -> &String { ownership is not moved thus s will be freed
    //     let s = String::from("hello");
    //
    //     &s // scope of s ends here but we want ro return reference to that object
    // }
    // let reference_to_nothing = dangle();

    fn no_dangle() -> String {
        // no problem, ownership is moved thus s won't be freed
        let s = String::from("hello");

        s
    }

    // The Slice Type
    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];

    let s = String::from("hello");

    let len = s.len();

    let slice = &s[0..len];
    let slice = &s[..];

    fn first_word(s: &String) -> &str {
        let bytes = s.as_bytes();

        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[0..i];
            }
        }

        &s[..]
    }
    let word = first_word(&s);
    println!("first word: {}", word);
}

fn calculate_length(s: String) -> usize {
    s.len()
}

// calculate_length_ref takes reference to an object instead of taking its ownership
fn calculate_length_ref(str: &String) -> usize {
    str.len()
}

fn change(s: &String) {
    // s.push_str("change") // immutable by default
}

fn change_mut(s: &mut String) {
    s.push_str("change muttable")
}

// 5. Structs
fn structs() {
    #[derive(Debug)]
    struct User {
        username: String,
        email: String,
        sign_in_count: u64,
        active: bool,
    }

    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    println!("user1: {:?}", user1);

    // whole user2 is mutable
    let mut user2 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    fn build_user(email: String, username: String) -> User {
        User {
            email, // we don't need to specify field name as param has the same name
            username,
            active: true,
            sign_in_count: 1,
        }
    }
    let u = build_user(String::from("email"), String::from("username"));
    println!("built user: {:?}", u);

    // Creating Instances From Other Instances With Struct Update Syntax
    let user2 = User {
        email: String::from("user2"),
        ..user1 // update syntax
    };

    // Using Tuple Structs without Named Fields to Create Different Types
    struct Point(i32, i32, i32);
    let p = Point(0, 0, 0);
    println!("p: {}, {}, {}", p.0, p.1, p.2)
}

// 5.2
fn calc_rect_area() {
    struct Rectangle(i32, i32);

    // Method Syntax
    impl Rectangle {
        fn area(&self) -> i32 {
            self.0 * self.1
        }
        fn update_x(&mut self, x: i32) {
            self.0 = x
        }
    }

    fn calc(r: &Rectangle) -> i32 {
        r.0 * r.1
    }

    let rect = Rectangle(10, 30);
    println!("rect: ({},{}); area: {}", rect.0, rect.1, rect.area());

    let mut m_rect = rect;
    m_rect.update_x(2);
    println!("rect: ({},{}); area: {}", m_rect.0, m_rect.1, m_rect.area());

    // Associated Functions
    #[derive(Debug)]
    struct Rect {
        width: u32,
        height: u32,
    }

    impl Rect {
        fn square(size: u32) -> Rect {
            // static method
            Rect {
                width: size,
                height: size,
            }
        }
    }

    let sq = Rect::square(2);
    println!("sq: {:?}", sq)
}

// 6.
fn enums() {
    enum IpAddrKind {
        V4,
        V6,
    }

    impl IpAddrKind {
        fn describe(self) -> String {
            match self {
                IpAddrKind::V4 => String::from("version four"),
                IpAddrKind::V6 => String::from("version six"),
            }
        }
    }

    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    let desc4 = four.describe();
    println!("{}, {}", desc4, six.describe());

    enum IpAddr {
        V4(u8, u8, u8, u8),
        V6(String),
    }

    let no: Option<u8> = None;

    #[derive(Debug)] // so we can inspect the state in a minute
    enum UsState {
        Alabama,
        Alaska,
        // --snip--
    }

    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter(UsState),
    }

    fn value_in_cents(coin: Coin) -> u8 {
        match coin {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter(state) => {
                println!("State quarter from {:?}!", state);
                25
            }
        }
    }
    println!("value: {}", value_in_cents(Coin::Quarter(UsState::Alaska)));

    fn mtch(num: u8) {
        match num {
            5 => println!("5!"),
            10 => println!("10"),
            _ => println!("not specified: {}", num),
        }
    }
    mtch(5);
    mtch(11);
}

// 8.
fn common_collections() {
    // VECTORS
    let mut v: Vec<i32> = Vec::new();
    let mut v1 = vec![1, 2, 3, 4];

    v.push(1);
    v1.pop().expect("failed to pop");

    println!("v: {:?}, v1: {:?}", v, v1);

    // let mut v = vec![1, 2, 3, 4, 5];
    //
    // let first = &v[0]; will break because reference may change after changes in vector
    //
    // v.push(6);
    //
    // println!("The first element is: {}", first);

    // HASH MAP
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    println!("scores: {:?}", scores);

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let mut scores: HashMap<_, _> = teams.into_iter().zip(initial_scores.into_iter()).collect();
    println!("scores: {:?}", scores);

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(&field_name, &field_value);
    println!("scores: {:?}", field_name);

    scores.entry(String::from("Green")).or_insert(20);
    println!("{:?}", scores);
}
