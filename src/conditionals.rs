pub fn run(){
        //Conditionals so if else and stuff

        let age:u8=22;
        let check_id:bool=true;
        let knows_person_of_age:bool=false;
        
        //If Else Statements
        if age>=21 && check_id || knows_person_of_age{
            println!("Bartender: What would you like to drink?");
        }else if age<21 &&check_id || knows_person_of_age{
            println!("Bartender: Sorry you're {}.\nPlease Fuck Off", age);
        }else{
            println!("Bartender: I will need to see your ID.");
        }
        
        //Shorthand If Statement
        let is_of_age=if age>=21{true}else{false};
        println!("Is of age: {}", is_of_age);
        
}