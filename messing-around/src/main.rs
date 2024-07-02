mod messingaround2;
fn main() {
  let thing = messingaround2::NetworkMessage::Connect {
    ip: String::from("bruh"),
    port: 723,
  };
  messingaround2::matching2(&thing);
}
