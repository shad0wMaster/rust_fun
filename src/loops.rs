pub fn run(){
    //let mut count:i8=0;
    //Infinite loop
    /*loop{
        count+=1;
        println!("Number: {}",count);
        
        //Condition to stop loop 
        if count==70{
            break;
        }
    }
*/
    //While loop fizz buzz
     //this is a challenge so brush up
    /*while count<=100{
        if count%15==0{
            println!("FizzBuzz");
        }
        else if count%3==0{
            println!("Fizz");
        }
        else if count%5==0{
            println!("Buzz");
        }else{
            println!("{}",count)
        }
        //Inc always remember to increment or initiate the counter when in while loop
    count+=1;
    }*/

    //For range loop 
    let fizz:String=String::from("Fizz");//divisible by 3
    let buzz:String=String::from("Buzz");//divisible by 5
    let fizzbuzz:String=String::from("FizzBuzz");//divisible by 15
    for x in 1..=100{
        if x%15==0{println!("{}",fizzbuzz);}
        else if x%3==0{println!("{}",fizz);}
        else if x%5==0{println!("{}",buzz);}
        else{println!("{}",x)}
    }
}