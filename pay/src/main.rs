use pay::Amount;

fn main() {
    let a=Amount::new(55);
    println!("Amount {}",a);
    let b=Amount::new(65);
    println!("Added Amount {}",a+b);

}
