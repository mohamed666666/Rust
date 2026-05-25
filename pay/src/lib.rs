use std::fmt::Display;
use std::marker::PhantomData;
//pub struct Amount<C>(u64); is ^ unused type parameter
#[derive(Debug)]
pub struct Amount<C>{
    value:u64,
    _phantom:PhantomData<C>//size is 0 to avoid the unsused error
}


#[derive(Debug)]
pub struct USD;


#[derive(Debug)]
pub struct EGP;


#[derive(Debug)]
pub struct GPB;


impl<C> Amount<C>{
    pub fn new(value:u64)->Self{
        Amount{value:value,_phantom:PhantomData}
    }
}


impl std::fmt::Display for Amount<USD>{
    fn fmt(&self,f: &mut std::fmt::Formatter<'_>)->std::fmt::Result{
        write!(f,"${}",self.value)
    }
}


impl std::fmt::Display for Amount<EGP>{
    fn fmt(&self,f: &mut std::fmt::Formatter<'_>)->std::fmt::Result{
        write!(f,"E{}",self.value)
    }
}




impl<C> std::ops::Add<Self> for Amount<C> { 
    type Output=Self;

    fn add(self,other:Self)->Self{
        Amount{value:self.value+other.value,_phantom:PhantomData}
    }
}