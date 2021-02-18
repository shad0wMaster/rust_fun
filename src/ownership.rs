/*pub fn run(){
    //Ownership and shit 
    
    /*Important bullcrap 
    each value in rust has a variable thats called its owner
    there can only be one owner at a time and
    when the owner goes out of scope the value is dropped RECIEVED 
    */
            //s is not valid here and its not yet been declared
    let s="hello"; //from this point s is valid and forward
    //do stuff with s and
}   //scope is now over s is no longer valid 
*/
/*
Essentially you don't want multiple variables running constantly on the program.
You want o close the ones from the brace's as fast as possible to keep stack size up
Crazy bullshit but it does make sense it kind of speeds up the program when you think about it.
*/

//The string type this bullshit lives on the heap. The stack contains primitive stuff.
//So the from function built into String is used because it stores data on the heap that is not available until compile time
 pub fn dog_string(){
    let mut dog=String::from("bone");
    dog.push_str(", ball");
    println!("{}",dog);
 }
  
 //new shit
 fn space(){
    let s=String::from("blackhole");
    //do stuff with s
    println!("{}",s);
} 