use tokio::net::TcpStream;
use tokio::io::AsyncWriteExt;
use std::io;
struct login_packet {
    Name: String,
    UUID: String,
}
impl login_packet {
    fn prepare(&self) -> Vec<u8> {
        let mut vector: Vec<u8> = Vec::new();
        vector.push(0x0);
        vector.extend_from_slice(&self.Name.as_bytes());
        vector.extend_from_slice(&self.UUID.as_bytes());
        add_lenght(&mut vector);
        vector
    }
}
#[repr(u8)]
#[derive(Clone, Copy)]
enum Intent {
    Status = 1,
    Login = 2,
    Transfer = 3,
}
struct handshake {
    protocol_version: Vec<u8>,
    server_adresse: String,
    server_port: u16,
    intent: Intent,
}
impl handshake {
    pub fn handshake_prepare(&self) -> Vec<u8> {
        let mut vector: Vec<u8> = Vec::new();
        vector.push(0x0);
        vector.extend_from_slice(&self.protocol_version);
        vector.extend_from_slice(add_string_lenght(*self.server_adresse.clone());
        vector.extend_from_slice(&self.server_port.to_be_bytes());
        vector.extend_from_slice(&(self.intent as u8).to_be_bytes());
        add_string_lenght(2, &mut vector);
        add_lenght(&mut vector);
        println!("{:?}", vector);
        vector
    }
}
#[tokio::main]
async fn main() {
    println!("Botname: ");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Invalid String");
    let user = login_packet {
        Name: input,
        UUID: "f468a7d8-191f-4e79-ad48-67fca48825c6".to_string(),
    };
    let mut stream = TcpStream::connect("5.175.249.209:25565").await.unwrap();
    let handshake = handshake {
        protocol_version: to_VARINT(776),
        server_adresse: "5.175.249.209".to_string(),
        server_port: 25565,
        intent: Intent::Login,
    };
    let data_prelogin_phase = handshake.handshake_prepare();
    let data_login_phase = user.prepare();
    send_data(&data_prelogin_phase, &mut stream).await;
    send_data(&data_login_phase, &mut stream).await;
}
fn add_lenght(vector: &mut Vec<u8>) -> &mut Vec<u8> {
    let laenge: i64 = vector.len() as i64;
    vector.insert(0, to_VARINT(laenge)kp wie ichcdrüber iterieren kann;
    vector
}
async fn send_data(vector: &Vec<u8>, stream: &mut TcpStream) {
    stream.write_all(vector).await;
    stream.flush().await;
}
fn add_string_lenght(ip: String) -> Vec<u8> {
    let leange = ip.len() as i64;
    let mut string: Vec<u8> = Vec::new();
    string.push(ip.to_be_bytes.len());
    string.push(ip);
    string
}
fn to_VARINT(value: i64) -> Vec<u8> {
    let mut value = value as u64;
    let mut out = Vec::with_capacity(10);
    while value >= 0x80 {
        out.push((value as u8 & 0x7F) | 0x80);
        value >>= 7;
    }
    out.push(value as u8);
    out
}