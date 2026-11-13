use tokio::net::TcpStream;
use tokio::io::AsyncWriteExt;
use std::io;
mod token;
struct login_packet {
    Name: Vec<u8>,
    UUID: uuid::Uuid, 
}
impl login_packet {
    fn prepare(&self) -> Vec<u8> {
        let mut vector: Vec<u8> = Vec::new();
        vector.push(0x0);
        vector.extend_from_slice(&self.Name);
        vector.extend_from_slice(self.UUID.as_bytes());    
        add_lenght(&mut vector);
        vector
    }
}
#[repr(u8)]
#[derive(Clone, Copy)]
enum Intent { //sollte Varint sein aber unter 128 juckt!!
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
        vector.extend_from_slice(&add_string_lenght(&self.server_adresse.clone()));               // "If It works dont touch it"
        vector.extend_from_slice(&self.server_port.to_be_bytes());                                  //- really smart person
        vector.extend_from_slice(&(self.intent as u8).to_be_bytes());
        add_lenght(&mut vector);
        println!("{:?}", vector);
        vector
    }
}
#[tokio::main]
async fn main() {
    token::get_refresh().await;
    println!("Test stuff: {:?}", add_string_lenght("Test"));
    println!("Botname: ");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Invalid String");
    let user = login_packet {
        Name: add_string_lenght(input.trim()),
        UUID: uuid::Uuid::parse_str("f468a7d8-191f-4e79-ad48-67fca48825c6").unwrap(),
    };
    let mut stream = TcpStream::connect("127.0.0.1:25565").await.unwrap();
    let handshake = handshake {
        protocol_version: to_VARINT(776),
        server_adresse: "127.0.0.1".to_string(),
        server_port: 25565,
        intent: Intent::Login,
    };
    let data_prelogin_phase = handshake.handshake_prepare();
    let data_login_phase = user.prepare();
    send_data(&data_prelogin_phase, &mut stream).await;
    send_data(&data_login_phase, &mut stream).await;
}
fn add_lenght(vector: &mut Vec<u8>) -> &mut Vec<u8> {
    let laenge: i32 = vector.len() as i32;
    vector.splice(0..0, to_VARINT(laenge));
    vector
}
async fn send_data(vector: &Vec<u8>, stream: &mut TcpStream) {
    stream.write_all(vector).await;
    stream.flush().await;
}
fn add_string_lenght(ip: &str) -> Vec<u8> {  //absoluter Müll compiled es?? Antwort vom Zukunfts ich: NEINNN
    let mut out = to_VARINT(ip.len() as i32); // Merken wenn du keine Ahnung hasst wie du es behnnen sollst mach einfach out. kommt bsl drauf an aber
    out.extend_from_slice(ip.as_bytes()); // KEIN BOCK AUF ENGLISCH // WIESO REFERENZ??? //nvm
    out
}
fn to_VARINT(value: i32) -> Vec<u8> {     //Hexerei
    let mut value = value as u64;
    let mut out = Vec::with_capacity(10);
    while value >= 0x80 {
        out.push((value as u8 & 0x7F) | 0x80);
        value >>= 7;
    }
    out.push(value as u8);
    out
}