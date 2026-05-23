
struct Node{
    a:u8,
    b: u32,

}

fn main(){
    let a=5;
    let b=8;
    let c=a+b;
    let j=b-a;
    println!("Sum of {}  and {} is = {}" ,a,b,c);
    println!("{} minus {}  is {}",b,a,j);

    println!("true and false is : {}",true && false);
    println!("true or false is : {}",true || false);
    println!("not true is : {}",!true && true);
    println!("5 bit wise and 5 is {}" ,5&5);
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
     println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);

}