//mod tuples;
//mod arrays;
//mod vectors;
//mod conditionals;
//mod loops;
//mod control_flow;
//mod converter;
//mod fibonacci;
//mod ownership;
mod translated;
fn main() {
    //ownership::run();
    //ownership::dog_string();
    //converter::get_fucked();
    translated::circle_math::math();
    translated::basic_calculator::run();
    /*fibonacci::run();
    //const MAX_POINTS: u32=1000;
    let mut x=5; //needs to be immutable in order to be used twice or more throughout the code.
    println!("The value of x is: {}",x);
    x=x+1;
    x=x+5;
    println!("The value of x is: {}",x);
    

    let spaces="    ";
    let spaces=spaces.len();
    println!("{}",spaces);

    let guess:u32="10".parse().expect("Not a number!"); //the type "u32,i32, etc..." needs to be assigned in order to work
    println!("{}",guess);

    
    Primitive Types--
    unsigned or u16 etc... means you cannot have negative values
    Integers: u8, i8, u16, i16, u32, i32, u64, i64, u128, i128, {number of bits they take into memory}
    Floats: f32, f64
    Boolean (bool)
    Characters {char} not a string single character
    Tuples
    Arrays
    

    let y=2.0; //f64 64bit float 
    let z:f32=3.0; //f32 32bit float
    println!("{},{}",y,z); 
    //math
    //addition
    let sum=5+10;
    println!("Addition: {}",sum);

    //subtraction
    let difference=95.6-21.3;
    println!("Difference: {}",difference);

    //multiplication
    let multiplication=87*21;
    println!("Multiplication: {}",multiplication);
    
    //remainder
    let remainder=43%5;
    println!("Remainder: {}",remainder);

    //Booleans
    let t=true;
    let f: bool=false;//explicit annotation whatever that means
    println!("{},{}",t,f);

    //Tuples
    let tup:(i32,f64,u8)=(87,2.5,9);
    println!("{}",tup.0);

    /*let tup1=(700,8.7,6);
    let (l,m,n)=tup1;
    println!("The value at index 1 is 8.7: Survey says!: {}", m);*/

    let g:(i32,f64,u8)=(478,3.8,2);
    let _fuck=g.0; //this will be 478
    let _fuck1=g.1; //this will be 3.8
    let _fuck2=g.2; //this will be 2

    //Arrays
    //this is where the money and pussy is at

    //Storing integers in an array.
    let array=[6,7,8,9,0];
    
    //Storing Strings in an array.
    let _months=["January","February","March","April","May","June","July","August","September","October","November","December"];

    //i32 specifies that the array has 32 bit integers
    //the 5 represents there are 5 elements to the array.
    let _a:[i32;5]=[20,21,22,23,24]; 

    //if you want to only add one value say 3 and 5 elements.
    let _three_value_array=[3;5];
    //you can also write it three_value_array=[3,3,3,3,3];

    // Accessing Array Elements The Fun Part
    let first_index=array[0];
    let second_index=array[1];
    println!("The first index is: {}",first_index);
    println!("The second index is: {}", second_index);    
    
    /*Accessing Arrays but invalid index;
    let index=10;
    let element=array[index];
    code does not execute since the index is out of bounds retard*/

    //to execute another function it must be worked in the main function
    test_function(-9,72.7);

    //calling sixty_nine function in main
    let t=sixty_nine();
    println!("The value for sixty_nine function is: {}",t);
    
    //calling the multiply_by_eight function
    let h=multiply_by_eight(8);
    println!("Should be 64: {}",h);
    */
    //tuples::run();
    //arrays::run();
    //vectors::run();
    //conditionals::run();
    //loops::run();
    //control_flow::run();
    //converter::get_fucked();
}
/*
Functions the big stuff
fn test_function(l:i8,w:f32){
    println!("Hello World from TestFunction");
    println!("The value for L was passed from main: {}",l);
    println!("The value for W was passed from main: {}",w);
    let r:i8=32;
    println!("The value for R is a i8 8 bit integer. {}",r);
    let z={
        let c=5;
        c+65;
    };
    println!("{}",z);
    println!("The value for Z is: {}",z);
}

functions can hold oen specific value and return it as well.
fn sixty_nine()->i8{69}

fn multiply_by_eight(h:i8)->i8{h*8}

pub fn run(){
    let hello=String::from("Hello World from TestFunction");
     //loop through string by whitespace
    for word in hello.split_whitespace(){println!("{}",word);}

    //Create a string with capacity
    let mut s=String::with_capacity(2);
    s.push('a');
    s.push('b');
    println!("{}",s);

    //Assertion test
    assert_eq!(2,s.len()); //passes
    //assert_eq!(3,s.len()); fails
    
}*/