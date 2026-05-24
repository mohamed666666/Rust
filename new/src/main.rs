

fn even_odd(input:&u32)->&str{//referance can not be returned for local varaible at function because after the functio is return it's frame in stack got forgetten and program counter can not see the va;ues any more
    if input%2==0 {
        return "Even";//string letterlas not stored at stack as local variable 
    }
    else{
        return "odd";
    }
}

fn even_odd_generic<T>(input:u32,even:T,odd:T)->T{//like generate new integr function in compile time 
    if input%2==0 {
        return even;
    }
    else{
        return odd;
    }
}

fn main() {
    let n:u32=rand::random();
    
    println!("the number {} is {} !",n,even_odd(&n));
    println!("the number {} is {} !",n,even_odd_generic(n,100,200));

}
