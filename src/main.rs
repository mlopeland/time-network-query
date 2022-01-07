use std::net::{ TcpListener, TcpStream };
use std::{ thread, time, str, env };
use std::io::{ Write, Read, Result };
use std::vec::Vec;
use chrono::{ Utc, DateTime };

mod logger;

fn client_thread(target : String) {
  loop {
    let stream : Result<TcpStream> = TcpStream::connect(format!("{}", target));
    if stream.is_err() {
      println!("Client failed to connect: {:?}", stream.err());
      continue;
    }
    
    let timestamp : DateTime<Utc> = Utc::now();
    let msg = format!("{},{},", Utc::now().to_rfc3339(), timestamp.timestamp_millis());
    
    let wrote : Result<usize> = stream.unwrap().write(msg.as_bytes());
    if wrote.is_err() {
      println!("Client failed to write response: {:?}", wrote.err());
      continue;
    }
    
    thread::sleep(time::Duration::from_millis(2000));
  }
}

fn server_thread(local : String, filename : &String) {
  loop {
    let listener: Result<TcpListener> = TcpListener::bind(&local);
    if listener.is_err() {
      println!("Server failed to bind listener: {:?}", listener.err());
      continue;
    }
    let listener: TcpListener = listener.unwrap();
    
    for stream in listener.incoming() {
      if stream.is_err() {
        println!("Server failed to receive stream: {:?}", stream.err());
        continue;
      }

      let mut data = [ 0 as u8 ; 50 ];
      let read : Result<usize> = stream.unwrap().read(&mut data);
      if read.is_err() {
        println!("Server failed to read from stream: {:?}", read.err());
        continue;
      }

      let res : std::result::Result<&str, std::str::Utf8Error> = str::from_utf8(&data);
      if res.is_err() {
        println!("Server failed to decode data from stream: {:?}", res.err());
        continue;
      }
      let res : &str = res.unwrap();

      let local : DateTime<Utc> = Utc::now();
      let parts : Vec<&str> = res.split(',').collect();
      let remote_millis : i64 = parts[1].parse().unwrap();
      let local_millis : i64 = local.timestamp_millis();
      let diff : i64 = local_millis - remote_millis;
      logger::log(local.to_rfc3339(), parts[0].to_string(), diff.to_string(), filename);
    }
  }
}
  
fn main() {
  let args : Vec<String> = env::args().collect();
  let target = format!("{}", args[1]);
  let local = format!("{}", args[2]);
  let name = format!("{}", args[3]);

  let server = thread::spawn(move || {
    server_thread(local, &name);
  });  

  let client = thread::spawn(|| {
    client_thread(target);
  });

  server.join().unwrap();
  client.join().unwrap();
}
