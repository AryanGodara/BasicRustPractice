fn main() {
    println!("Hello, world!");
    another_function() ;
    another_function_with_parameters(5,'h') ;

    // ? Expressions and Statements
    // Expressions : Return a Value , Statements : Don't return a Value

    let y = {
        let x = 3 ;
        x+1     // No semicolon (Expressions don't need ; at the end)
    // ! If you add ; after an expression, it'll turn into a statement, and won't return a value
    };  // * This {} block returns the value 4
    println!("The value of y is: {}", y);

    let x = five() ;
    println!("The value of x is: {}", x);

    let x = plus_one(5) ;
    println!("The value of x is: {}", x);

    if_else_basic_check() ;

    equivalent_of_ternary_operator() ;


    // ! LOOPS ! Iterators ! BLABLABLABLA

    //* Rust has three types of loops : 'loop' , 'while' and 'for' */

    // ? 'loop' tells rust to execute a piece of code forever until we explicitly tell it to stop
    // loop {
    //     println!("again!") ;
    // }    // ! Press ctrl-c to stop

    // * break and continue keywords are also present in Rust, work same as in C++



    // ? Using 'loop' with break and continue :-
    // * You can optionally specify a loop label on a loop that we can then use with break or 
    // * continue to specify that those keywords apply to the labeled loop instead of the 
    // * innermost loop.
    
    let mut count = 0;

    'counting_up: loop {
        println!("count = {}", count);
        let mut remaining = 10;

        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {}", count);

    // * The outer loop has the label 'counting_up, and it will count up from 0 to 2. 
    // * The inner loop without a label counts down from 10 to 9. The first break that doesn’t 
    // * specify a label will exit the inner loop only. The break 'counting_up; statement will 
    // * exit the outer loop. 



    // ? Returning Values from Loops :-
    // One of the uses of a loop is to retry an operation you know might fail, such as checking 
    // whether a thread has completed its job. You might also need to pass the result of that 
    // operation out of the loop to the rest of your code. To do this, you can add the value 
    // you want returned after the break expression you use to stop the loop; that value will 
    // be returned out of the loop so you can use it, as shown here:

    let mut counter1 = 0 ;
    let result1 = loop {
        counter1 += 1 ;

        if ( counter1 == 10 ) { // ! () can be used, but compiler tells to remove them
            break counter1*2 ;      // Returns counter1*2 == 20, out of this loop
        }
    };
    println!("The result is : {}", result1) ;


    // ? Conditional Loops with WHILE :-
    let mut whilenum = 3 ;

    while whilenum != 0 {
        println!("{}", whilenum) ;
        whilenum -= 1 ;
    }


    // ? Looping through a collection with FOR :-
    // ! We can also use while loop, but it's a TEDIOUS process and ERROR PRONE
    let a = [10, 20, 30, 40, 50];
    let mut whileindex = 0;

    while whileindex < 5 {
        println!("the value is: {}", a[whileindex]);

        whileindex += 1;
    }

    // * Instead, use a for loop
    for element in a {
        println!("the value is: {}", element);
    }
    
    println!("\n") ;
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}

fn another_function ()
{
    println!("This is another Function!") ;
}
fn another_function_with_parameters ( value: i32 , unit_lable: char )
{
    println!("The measurement is : {}{}", value, unit_lable) ;
}

// Function with return value (No need to write return for the last expression inside function)
fn five() -> i32        // -> return_value_type (instead of return_type fun_name () in C/C++)
{
    5       // No need to write 'return', last expression is synonymous with return value
}

fn plus_one(x: i32) -> i32 {
    x+1
}

fn if_else_basic_check() {
    let number = 6 ;
    // ! brackets not necessary, but {} are required, even for single line after if
    if number <= 5
    {
        println!("Condition Satisfied") ;
    }
    else if number > 5 && number < 10
    {
        println!("Condition Somewhat Satisfied") ;
    }
    else
    {
        println!("Condition NotSatisfied") ;
    }

    // if number {      //* Use : if number != 0
    //     println!("Doesn't work here bro") ;
    // }
    // ! Here, we can't just expect number(=3) to convert to (1,true) boolean value, we have
    // ! to explictly write a boolean expression for conditionals
}

fn equivalent_of_ternary_operator() {
    let condition = true ;  // A boolean value

    let number = if condition { 5 } else { 6 } ;
    // ! Both if block and else block variable must be of same type(i32 in this case)
    // * Because variables can't be of multiple types
    // Kind of like : int number = condition ? 5 : 6 in C++
}