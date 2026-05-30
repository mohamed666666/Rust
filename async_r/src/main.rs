use std::net::TcpListener;
use std::time::Duration;
use std::rc::Rc;
use std::sync::Arc;
use std::sync::Mutex;

fn main() {
    let mut t2shutdown=Arc::new(Mutex::new(false));//t2shutdown:Rc<bool>=Rc::new(false); ^ Rc<bool> cannot be shared between threads safely

    let t1= std::thread::spawn(|| {//spawn dunction return refrance called johnhandle object used as remote controll over the thread
         let listener=TcpListener::bind("0.0.0.0:4000").unwrap();
    println!("Server Start listen on : {:?}",listener);
    let (stream,socket)=listener.accept().unwrap();
    println!("connection recieved on : {:?}",socket);
        stream
    }
    );
    let cloned_shutdown=Arc::clone(&t2shutdown);//pass copy to the thread 
    std::thread::spawn( move|| {//use move to pass the referance from main thread to current thread actually moving it 
         while !*cloned_shutdown.lock().unwrap(){//
            println!("Hello from the Secound thread ");
            std::thread::sleep(Duration::from_secs(2));
         }

    }
    );
    

    let stream=t1.join();//wait untill the thread finish and take the returned data 
    {// *t2shutdown=true;  trait `DerefMut` is required to modify through a dereference, but it is not implemented for `Arc<bool>`
        let mut gaurd=t2shutdown.lock().unwrap();
        *gaurd=true;//update the value throw lock 
        //droped the lock after 
    }
    println!("closed all threads ");

}
