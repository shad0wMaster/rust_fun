pub fn run(){
    //Tuples group together values of different elements
    //max 12 elements

    let person:(&str,&str,i8)=("Andy","Kukuc",25);
    println!("First Name: {}\nLast Name: {}\nCurrent Age: {}",person.0,person.1,person.2);
}