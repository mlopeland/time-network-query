use std::net::{ TcpListener, TcpStream };
use std::{ thread, time, str, env };
use std::io::{ Write, Read };
use std::vec::Vec;
use chrono::{ Utc, DateTime };

mod logger;

fn client_thread(target : String) {
  loop {
    match TcpStream::connect(format!("{}", target)) {
      Ok(mut stream) => {
        let timestamp : DateTime<Utc> = Utc::now();
        let msg = format!("{},{},", Utc::now().to_rfc3339(), timestamp.timestamp_millis());
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
                let local : DateTime<Utc> = Utc::now();
                let parts : Vec<&str> = res.split(',').collect();
                let remote_millis : i64 = parts[1].parse().unwrap();
                let local_millis : i64 = local.timestamp_millis();
                let diff : i64 = local_millis - remote_millis;
                logger::log(local.to_rfc3339(), parts[0].to_string(), diff.to_string());
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
