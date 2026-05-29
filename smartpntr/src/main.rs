use std::rc::Rc;
#[derive(Clone)]
struct File{
    name:String,
    data:Vec<u8>
}

impl File{
    fn new(name:&str,lenght:usize)->File{
        File{
            name:name.to_string(),data:vec![0;lenght]
        }
    }

}

impl Drop for File{
    fn drop(&mut self){
        println!("Droping file {} " ,self.name)
    }
}

struct Directory{
    name:String,
    files:Vec<Rc<File>>
}
impl Directory{
    fn new(name:&str)->Directory{
        Directory{
            name:name.to_string(),
            files:vec![]
        }
    }

    fn add(&mut self ,file:Rc<File>){
        self.files.push(file);
    }

    fn list(&self){
        println!("Dir : {} ",self.name);
        for f in &self.files{
            println!("File : {}  links : {}",f.name ,Rc::strong_count(&f));
        }
        println!();
    }
}


fn main() {
    let f =Rc::new(File::new("first_file",12)) ;
   
    let mut d=Directory::new("User");
    d.add(Rc::clone(&f));
    d.list();

    let f2 = Rc::new(File::new("Secounfile",10));
 
    d.add(Rc::clone(&f2));
    d.list();
}
