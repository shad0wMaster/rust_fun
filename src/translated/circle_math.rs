pub fn math(){
    let radius:f64=(62.80/std::f64::consts::PI).sqrt();
    let circumference:f64=2.0*std::f64::consts::PI*radius;
    let surface_area:f64=4.0*std::f64::consts::PI*radius.powi(2);
    println!("The Radius is: {:?}\nThe Circumference is: {:?}\nThe Surface Area is: {:?}",radius,circumference,surface_area);
}