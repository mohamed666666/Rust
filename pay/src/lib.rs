use std::fmt::Display;


pub enum Currency{
    USD,
    EGP,
    EUE
}
impl Display for Currency{
    fn fmt(&self ,f : &mut std::fmt::Formatter<'_> )->std::fmt::Result{
        write!(f,"{}",self)
    }
}


pub struct Amount{
    value:u64,
    currency: Currency
}

impl Amount{
    pub fn new(value:u64,currency:Currency)->Self{
        Amount{value,currency}
    }
}

impl Display for Amount{
    fn fmt(&self ,f : &mut std::fmt::Formatter<'_> )->std::fmt::Result{
        write!(f,"{} {}",self.value,self.currency)
    }
}


impl std::ops::Add<Self> for Amount {
    type Output=Self;
    fn add (self,other:Self)->Self{
       return Amount{value:self.value+other.value,currency:self.currency};
    }

}