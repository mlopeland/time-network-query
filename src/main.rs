use std::net::{ TcpListener, TcpStream };
use std::{ thread, time, str, env };
use std::io::{ Write, Read };
use chrono::Utc;

mod logger;

fn client_thread(target : String) {
  loop {
    match TcpStream::connect(format!("{}", target)) {
      Ok(mut stream) => {
        let msg = Utc::now().to_rfc3339();
        stream.write(msg.as_bytes()).unwrap();
        // break;
      },
      Err(err) => {
        println!("Client failed to connect: {}", err);
      }
    }
    thread::sleep(time::Duration::from_millis(2000));
  }
}

fn server_thread(local : String) {
  let listener = TcpListener::bind(local).unwrap();
  for stream in listener.incoming() {
    match stream {
      Ok(mut stream) => {
        let mut data = [ 0 as u8 ; 50 ];
        match stream.read(&mut data) {
          Ok(_) => {
            match str::from_utf8(&data) {
              Ok(res) => {
                logger::log(Utc::now().to_rfc3339(), res.to_string());
                // break;
              },
              Err(err) => {
                println!("{}", err);
              }
            }
          },
          Err(err) => {
            println!("Server unable to read message from client: {}", err);
          }
        }
      },
      Err(err) => {
        println!("Server has failed to stablish connection: {}", err);
      }
    }
  }
}

fn main() {
  let args : Vec<String> = env::args().collect();
  let target = format!("{}", args[1]);
  let local = format!("{}", args[2]);

  let server = thread::spawn(|| {
    server_thread(local);
  });  

  let client = thread::spawn(|| {
    client_thread(target);
  });

  server.join().unwrap();
  client.join().unwrap();
}
