use std::io;
pub fn run(){
    //So in here we will be taking user input which can be anything they want. I think I will input floats for this!
    let clear_screen=String::from("\x1B[2J\x1B[1;1H"); //remember always clear the screen for the users to
    print!("{}",clear_screen);
    println!("Please enter your name! ");
    let mut name_string=String::new();
    io::stdin().read_line(&mut name_string).expect("Stdin library error!");
    println!("\nHello {}",name_string);

    //Now we will prompt the user and validate the input being passed is a float
    println!("Please Enter you first integer(W): ");
    let first_integer_string=String::new();
    let mut check_value=String::from(first_integer_string);
    checker(check_value);
    println!("Please Enter you second integer(W): ");
    let second_integer_string=String::new();
    check_value=String::from(second_integer_string);
    checker(check_value);
    let test1=String::from("Hello");
    let test=checker(test1);
    println!("FUGGGGGGG{:?}",test);
}
fn checker(value:String)->String{
    let mut fuck=String::from(value);
    io::stdin().read_line(&mut fuck).expect("Stdin library");
    println!("This is printed at the bottom: {:?}",fuck);
    return fuck;
}