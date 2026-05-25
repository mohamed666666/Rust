use std::fmt::Display;




pub struct Amount(u64);

impl Amount{
    pub fn new(value:u64)->Self{
        Amount(value)
    }
}

impl Display for Amount{
    fn fmt(&self ,f : &mut std::fmt::Formatter<'_> )->std::fmt::Result{
        write!(f,"${}",self.0)
    }
}


impl std::ops::Add<Self> for Amount {
    type Output=Self;
    fn add (self,other:Self)->Self{
       return Amount(self.0+other.0);
    }

}