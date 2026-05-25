use std::fmt::Display;


pub struct Amount<C>(u64);

impl Amount<C>{
    pub fn new(value:u64)->Self{
        Amount(value)
    }
}


