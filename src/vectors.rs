pub fn run(){
    //Vectors are resizable arrays. Arrays are fixed values.    

    let mut numbers: Vec<i32>=vec![1,2,3];
    //Adding stuff to vectors 
    numbers.push(6);
    numbers.push(87);

    //removing from Vectors
    //numbers.pop();
    //experimental
    //assert_eq!(numbers.pop(),Some(3));
    //assert_eq!(numbers,[1,2]);
    //println!("{:?}", numbers)

    //for loop through Vectors //immutable
    for x in numbers.iter() {
        println!("Number: {}",x);
    }
    
    //for loop that can be mutable (Changed)
    for x in numbers.iter_mut() {
        *x*=5;
    }
    println!("Numbers Vector: {:?}",numbers);
    
}