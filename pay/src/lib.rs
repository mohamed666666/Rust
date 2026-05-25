use std::fmt::Display;
use std::marker::PhantomData;
//pub struct Amount<C>(u64); is ^ unused type parameter
#[derive(Debug)]
pub struct Amount<C>{
    value:u64,
    _phantom:PhantomData<C>//size is 0 to avoid the unsused error
}


pub trait Currency{
    const SYMBOL:&'static str;

}


impl<C> Amount<C>{
    pub fn new(value:u64)->Self{
        Amount{value:value,_phantom:PhantomData}
    }
}


impl<C> std::fmt::Display for Amount<C> 
where C:Currency{
    fn fmt(&self,f: &mut std::fmt::Formatter<'_>)->std::fmt::Result{
        write!(f,"{}{}",C::SYMBOL,self.value)
    }
}






impl<C> std::ops::Add<Self> for Amount<C> where C:Currency, { 
    type Output=Self;

    fn add(self,other:Self)->Self{
        Amount{value:self.value+other.value,_phantom:PhantomData}
    }
}