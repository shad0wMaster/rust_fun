//Written by Andy "shad0wMast3r" Kukuc
//This program will calculate the fahrenheit to celsius.
use std::io;
fn get_input()->f32{ //returning a single value thats all its doing can be moved onto the next function
    //Start out by declaring the variables;
    println!("The Program Will Convert Fahrenheit to Celsius!\nPlease Enter The Temperature in Fahrenheit Degrees!"); 
    print!("\x1B[2J\x1B[1;1H"); //clear the terminal make it prettier
    let mut user_input=String::new(); //declaring the user_input as a string
    io::stdin().read_line(&mut user_input).expect("Failed to read from stdin library"); //makes sure the library is reading properly.
    let trimmed=user_input.trim(); // the input must be trimmed in order to work
    let converted_int=trimmed.parse::<f32>().unwrap();//setting the variable to a float
    return converted_int; //return the dumbass float as a 32 bit memory safe leak and bullshit
}

pub fn get_fucked(){ //infinite loop that runs until error
    print!("\x1B[2J\x1B[1;1H"); //clear the terminal make it prettier
    loop{
        //println!("The value from get_input is {:?}",get_input()); this was used to test it and it works correctly
        let current_temperature=get_input(); //assign the dumbass return as curren temperature
        let math=(current_temperature-32.0)/1.8; //do math convert fahrenheit to celsius
        println!("{:?} Fahrenheit to Celsius: {:?}",current_temperature,math.ceil()); //print bullshit to dumbass users
    }
}
