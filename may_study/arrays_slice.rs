
use std::mem;



fn main(){
    let array:[i32;5]=[1,2,3,4,5];
    println!(" first element of array {}",array[0]);
    // All elements can be initialized to the same value.
    let ys: [i32; 500] = [0; 500];

    println!(" the length of array is {}",array.len());
    let slice =&array[2..4];

    println!("First element Of slice is {}" , slice[0]);
    println!("Length Of slice is {}" , slice.len());
    // Arrays are stack allocated.
    println!("Array occupies {} bytes", mem::size_of_val(&ys));
    

}