use std::fs::OpenOptions;
use std::io::Write;

pub fn log(local : String, remote : String, diff: String, filename: &String) {
  println!("Latency: {} ms", diff);
  
  let mut file = OpenOptions::new()
    .write(true)
    .append(true)
    .create(true)
    .open(format!("{}.txt", filename))
    .unwrap();

  if let Err(err) = writeln!(file, "{},{},{}", local, remote, diff) {
    eprintln!("Failed to write log.txt {}", err);
  }
}
