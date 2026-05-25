use pay::{Amount,Currency};

fn main() {
    let a=Amount::new(55 , Currency::USD);
    println!("Amount {}",a);
    let b=Amount::new(65,Currency::USD);
    println!("Added Amount {}",a+b);

}
