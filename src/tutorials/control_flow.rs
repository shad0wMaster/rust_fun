pub fn run(){
    let number=71;
    if number%4==0{
        println!("{} is divisible by 4!", number);
    }
    else if number%3==0{
        println!("{} is divisible by 3!",number);
    }
    else if number%2==0{
        println!("{} is divisible by 2",number)
    }
    else{
        println!("Number is not divisible by 4,3, or 2")
    }
    //In here we will do Booleans and If statements
    let condition=true;
    let number=if condition {5} else {6};
    println!("Bottom of the code!\nThe value of number is: {}",number);

    //Down here we will do repetition with loops 
    //loop{ //this is infinite and will continue on forever.
        //println!("Again!");
    //}

    //Returning va1ues from loops
    //The following loop counts to 10, by incrementing by 1, stops at 10
    //multiplies by 2 for the final result
    let mut counter=0;
    let result=loop{
        counter+=1;
        if counter==10{
            break counter*2}
        };
    println!("The result is {}!\nFrom the loop statement",result);

    //While Loops are important as well as
    let mut number2=9;
    while number2!=0{
        println!("{}", number2);
        number2-=1;
    }
    println!("This is a while loop that counts backwards from the number 9!\n");

    //Looping with while with an array this shit right here pay attention
    let array=[10,20,30,40,50];
    let mut index=0; //must be set to mut because the number changes to

    //now the while loop gets initiated at the Bottom
    while index<5{
        println!("The value is: {}",array[index]); //this will print all numbers while counting in the array
        index+=1;
    }
    println!("This is the array counter towards the bottom!\n");

    //You can do the same as line 45-53 with iter library
    //Doing it this way makes the code much safer because no bugs can be created.
    for element in array.iter(){
        println!("The value is: {}",element);
    }
    println!("This is the iter library at the bottom\n");

    //So this is prettier code with the reverse library
    //it could come in handy

    for number3 in (1..4).rev(){ //the variable number3 is set to 1..4
        println!("{}",number3);
    }
    println!("This is the reverse library at the bottom")
}
