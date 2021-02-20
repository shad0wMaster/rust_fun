  //Fibonacci Code Bullshit
  //Written by Andy "shad0wMast3r" Kukuc
  use std::io;
pub fn run(){
   let clear_screen=String::from("\x1B[2J\x1B[1;1H");
       //First we set the variables to the correct and safe way
       print!("{}",clear_screen);
       loop{
       println!("Please input the Fibonacci Sequence Number you want: ");
       let mut input_number_integer=String::new(); //1st. Set the user input to a string will be converted
       io::stdin().read_line(&mut input_number_integer).expect("Stdin library error"); //don't forget error handling

       //conversion part
       let trimmed=input_number_integer.trim(); //open new variable for science
       let converted_input_string=trimmed.parse::<u128>().unwrap(); //set string to int
       //println!("{}",converted_input_string);

       //now the fibonnaci part 
       // set the first two variables to 0 and 1st and initiate a counter variable
       let mut number1:u128=0; //safety must be set to u128
       let mut number2:u128=1; //safety must be set to u64
       let counter:u128=0; //same initiate counter at 0

       println!("Fibonnaci Bullshit: {:?}",converted_input_string);
       //For loop cool way
       for _x in counter..converted_input_string{ //here we set the ending with the converted input
           let nth:u128=number1+number2; //here we set the nth
           number1=number2; //reassign variables
           number2=nth; //reassgin variables
           println!("{:?}",nth); //print the buullcrap
       }   
   }
}