use pay::{Amount,Currency};


#[derive(Debug)]
pub struct USD;


#[derive(Debug)]
pub struct EGP;


#[derive(Debug)]
pub struct GPB;

impl Currency for USD{
    const SYMBOL:&'static str ="$";
}




fn main() {
    let a:Amount<USD>=Amount::new(55);
    println!("this is amount  {}" ,a);
    let b:Amount<USD>=Amount::new(55);
    println!("this is amount  {}" ,a);
    //let a:Amount<GPB>=Amount::new(55); error because display not impl for GPB
    println!("this is amount a+b {}" ,a+b);

}
