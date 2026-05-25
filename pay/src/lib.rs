use std::fmt::Display;
use std::marker::PhantomData;
//pub struct Amount<C>(u64); is ^ unused type parameter
#[derive(Debug)]
pub struct Amount<C>{
    value:u64,
    _phantom:PhantomData<C>//size is 0 to avoid the unsused error
}

impl<C> Amount<C>{
    pub fn new(value:u64)->Self{
        Amount{value:value,_phantom:PhantomData}
    }
}


