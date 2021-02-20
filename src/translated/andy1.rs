pub fn math(){
    let area_of_circle:f64=62.80;
    let radius:f64=0.0;
    let circumference:f64=0.0;

    //first we will compute the radius
    let radius:f64=(area_of_circle as f64).sqrt() as f64;
    println!("{:?}",radius);

}