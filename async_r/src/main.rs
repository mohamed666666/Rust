use std::net::TcpListener;
use std::time::Duration;


fn main() {
    let t1= std::thread::spwan(|| {//spwan dunction return refrance called johnhandle object used as remote controll over the thread
         let listener=TcpListener::bind("0.0.0.0:4000").unwrap();
    println!("Server Start listen on : {:?}",listener);
    let (stream,socket)=listener.accept().unwrap();
    println!("connection recieved on : {:?}",socket);
        stream
    }
    )

     std::thread::spwan(|| {//spwan dunction return refrance called johnhandle object used as remote controll over the thread
         loop{
            println!("Hello from the Secound thread ");
            std::thread::sleep(Duration::from_secs(2));
         }

    }
    )
    

    let stream=t1.join();//wait untill the thread finish and take the returned data 
}
