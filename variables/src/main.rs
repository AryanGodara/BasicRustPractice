fn main() {
    let x = 5 ;
    println!("The value of x is : {}", x) ;
    // x = 6 ;  //! Can't do this
    println!("The value of x is : {}", x) ;

    let mut y = 5 ;
    println!("The value of y is : {}", y) ;

    y = 6 ;
    println!("The value of y is : {}", y) ;

    //* Constants are ALWAYS immutable, you can't use mut keyword with them

    // Constants may be set only to a constant expression, not the result of a value that
    // could only be computed at runtime.
    // Example :-
    const THREE_HOURS_IN_SECONDS: u32 = 60*60*3 ;

    // Shadowing a Variable :-
    let z = 5 ;
    let z = z+1 ;

    {
        let z = z*2 ;
        println!("The value of z is : {}", z) ;
    }

    println!("The value of z is : {}", z) ;

    // Changing data type by shadowing (say, we take input of how many space one wants)
    let spaces = "   " ; // 3 spaces
    let spaces = spaces.len() ;

    // Floating Types : -
    let x = 2.0 ;   // f64 by default
    let y: f32 = 3.0 ; // f32

    // Boolean type : -
    let t = true ;
    let f: bool = false ;   // with explicit type annotation

    // Character type :-
    let c = 'z' ;
    let z = 'Z' ;
    let heart_eyed_cat = '😻';  // 4bute , unicode scalar value (not ascii)

    // Compound Types : Two primary types are tuples and arrays

    // Tuples : Each element can be of different types
    let tup: (i32, f64, u8) = (500, 6.4, 1) ;
    let (x,y,z) = tup ; // Destructuring, breaks a single tuple into three parts.
    println!("The value of y is : {}", y) ;
    let five_hundred = tup.0 ;  // Instead of a[0] in CPP

    // Array : All elements must be of same type
    let a = [1,2,3,4] ;
    // Rust also has vectors yay!!
    let b: [i32;5] = [1,2,3,4,5] ; // data_type;size
    let c: [3;5] ;  // default_initialized_value_for_all_elements;no_of_elements
    let first = a[0] ;  // Same as CPP

    // We can't enter index >= array size (Just like CPP) or we'll get errors.
    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!(
        "The value of the element at index {} is: {}",
        index, element
    );
}

fn another_function()
{
    println1("Another Function!") ;
}