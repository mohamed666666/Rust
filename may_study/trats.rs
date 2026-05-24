#[derive(Debug)]
struct MemBudget{
    remaining:usize
}


impl MemBudget {
    fn new (bud:usize)->Self{
        Self{remaining:bud}
    }
    //called refrance for the same object update it's value 
    fn decrement(&mut self,mem :usize){
        self.remaining-=mem;
    }
    //Coppied the Membudget object as local object of the method 
    fn allocat(mut self,mem:usize){
        self.remaining-=mem;
    }

    // destruct 
    fn bye(){}

}



struct Point{
    x:i32,
    y:i32
}


pub trait Serilize{
    fn serialize(&self);
}


impl Serilize for Point{
    fn serialize(&self ){
        println!("The point serialized to {} {}" ,self.x,self.y);
    }
}



fn main(){
    let mut budget=MemBudget::new(1024);
    budget.decrement(100);
    println!(" {:?}",budget);

    let p=Point{x:10,y:15};
    p.serialize();

}