pub enum NetworkMessage {
  Ping, 
  Pong,
  Connect { ip: String, port: u16 },
  Disconnect {user_id: u32 },
  Message {user_id: u32, content: String},
}

fn matching(message: &NetworkMessage){
  match message{
    ping @ NetworkMessage::Ping => println!("It is a ping"),
    pong @ NetworkMessage::Pong => println!("It is a pong: "),
    connect @ NetworkMessage::Connect{ip: ip, port: port} => println!("ip is {}, port is {}, ", ip, port),
    disconnect @ NetworkMessage::Disconnect{user_id: user_id} => println!("user_id is {}, ", user_id),
    message @ NetworkMessage::Message{user_id: user_id, content: content} => println!("user id is {}, content is {}, " ,user_id, content),
  }
}
pub fn binding() {
  let _ping = NetworkMessage::Ping;
  let _pong = NetworkMessage::Pong;
  let _connect = NetworkMessage::Connect{ip: String::from("Bro"), port: 342};
  let _disconnect = NetworkMessage::Disconnect{user_id: 82374};
  let _message = NetworkMessage::Message{user_id: 32427, content: String::from("Hello")};
  matching(&_ping);
  matching(&_pong);
  matching(&_connect);
  matching(&_disconnect);
  matching(&_message);
}
