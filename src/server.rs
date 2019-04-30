use std::net::{Ipv4Addr, SocketAddr, TcpStream, SocketAddrV4};
use std::io::Error;

pub struct Server {
    pub ip: Ipv4Addr,
    pub port: u16,
    pub rcon_pass: String,
    pub rcon_connection: TcpStream,
    pub info: Option<Info>


}

pub struct Info {

}

impl Server {


    pub fn new(ip: Ipv4Addr, port:u16, rcon_pass:String) -> Result<Server, Error> {


        let socket = SocketAddr::V4(SocketAddrV4::new(ip, port));

        let rcon_connection = match TcpStream::connect(socket) {
            Ok(c) => {
                c
            }
            Err(e) => {
                return Err(e)
            }
        };

        let info = None;

        Ok(Server{
            ip,
            port,
            rcon_pass,
            rcon_connection,
            info,
        })

    }


}