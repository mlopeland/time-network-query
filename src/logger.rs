use std::fs::OpenOptions;
use std::io::Write;

pub fn log(local : String, remote : String) {
  println!("Local time:  {}", local);
  println!("Remote time: {}\n", remote);
  
  let mut file = OpenOptions::new()
    .write(true)
    .append(true)
    .create(true)
    .open("log.txt")
    .unwrap();

  if let Err(err) = writeln!(file, "{},{}", local, remote) {
    eprintln!("Failed to write log.txt {}", err);
  }
}
