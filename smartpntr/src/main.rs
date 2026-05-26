
#[derive(Debug)]
struct File{
    name:String,
    data:Vec<u8>
}

struct Directory{
    name:String,
    files:Vec<File>
}
impl Directory{
    fn new(name:&str)->Directory{
        Directory{
            name:name.to_string(),
            files:vec![]
        }
    }

    fn add(&mut self ,file:File){
        self.files.push(file);
    }

    fn list(&self){
        println!("Dir : {} ",self.name);
        for f in &self.files{
            println!("File : {} ",f.name);
        }
        println!();
    }
}

impl File{
    fn new(name:String,lenght:usize)->File{
        File{
            name:name,data:vec![0;lenght]
        }
    }

}


fn main() {
    let f = File::new("first_file".to_string(),10);
    println!("The File is : {:?}",f);

    let mut d=Directory::new("User");
    d.add(f);
    d.list();

    let f = File::new("Secounfile".to_string(),10);
    println!("The File is : {:?}",f);
    d.add(f);
    d.list();
}
