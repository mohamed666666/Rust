use pay::{Amount,USD,GPB,EGP};




fn main() {
    let a:Amount<USD>=Amount::new(55);
    println!("this is amount  {}" ,a);
    let b:Amount<EGP>=Amount::new(55);
    println!("this is amount  {}" ,a);
    //let a:Amount<GPB>=Amount::new(55); error because display not impl for GPB
    println!("this is amount a+b {}" ,a+b);

}
