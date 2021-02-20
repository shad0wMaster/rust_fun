pub fn run(){
    //Fixed list where elements are the same data Types
    let mut numbers:[i32;4]=[1,654,32432,2]; //each array element is  4 bytes
    println!("{:?}", numbers);

    //single value
    println!("{}", numbers[0]);
    
    //reassign values
    numbers[1]=69;

    //get array length
    println!("Array Length: {}", numbers.len());

    //Arrays are stack allocated
    println!("Array occupies {} bytes",std::mem::size_of_val(&numbers));

    //Get Slice of Array
    let slice:&[i32]=&numbers[0..3];
    println!("Slice: {:?}",slice);    

    
}
