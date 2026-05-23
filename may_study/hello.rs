

fn main(){
    println!("Hello world :");
    let logical:bool=true;
    let number:i32=250;//i64 i8 signed number 
    let fnum=3.0; // f32 
    let fnum2:f32 =256.0;
    let sufexnum=45i32;


    // array and tuples 
    let arr: [i32;4]=[1,2,3,4];


    let tup=(1i32,15,3.0,true);

    
    
    let tuple = (1, "hello", 4.5, true);

    let (a, b, c, d) = tuple;
    println!("{:?}, {:?}, {:?}, {:?}", a, b, c, d);

}