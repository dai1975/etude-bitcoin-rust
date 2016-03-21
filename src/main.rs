extern crate bitcoin_rust_etude as etude;

fn main() {
   let mut cli = etude::net::client::Client::new();
   match cli.run("127.0.0.1:48333".to_string()) {
      Ok(_)  => println!("finished."),
      Err(e) => println!("shutdown: {:?}", e),
   }
}
