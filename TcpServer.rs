use std::collections::HashMap;
use std::error::Error;
use std::io::{prelude::*, ErrorKind};
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::sync::{Arc, Mutex};

/*type ClientConnected = fn(param:str,param1:bool);
type ClientDisconnected = fn(param:str,param1:bool);
type DataReceived = fn(param:str,param1:vec,param2:bool);*/

struct ClientData {
    tcp_client: SocketAddr,
    stream: TcpStream,
}

struct TcpServer {
    receive_buffer_size: u16,
    listener_ip: String,
    port: u16,
    clients: Arc<Mutex<HashMap<String, ClientData>>>,
    listener: TcpListener,
    running: bool,
    logging: bool,
}

impl TcpServer {
    fn new(ip: &str, port: u16,listener:TcpListener) -> Self {
        Self {
            receive_buffer_size: 4096,
            listener_ip: ip.to_string(),
            port: port,
            clients: Arc::new(Mutex::new(HashMap::new())),
            listener: listener,
            running: false,
            logging: true,
        }
    }

    fn get_client(&self, ip_port: &str) -> ClientData {
        return self.clients.lock().unwrap()[ip_port];
    }

    fn add_client(&mut self, ip_port: String, client: ClientData) {
        self.clients.lock().unwrap().insert(ip_port, client);
    }

    fn remove_client(&mut self, ip_port: &str) {
        self.clients.lock().unwrap().remove(ip_port);
    }

    fn start(&self) {
        if (self.running) {
            println!("server is running");
            return;
        }
        self.running = true;
        self.listener = TcpListener::bind((self.listener_ip + ":" + &self.port.to_string()).to_string()).unwrap();

        self.listener.set_nonblocking(true);

        self.accept_connections();
    }

    // Asynchronously send data to the specified client by IP:port.
    async fn send(&self, ip_port: &str, data: Vec<u8>) {
        let mut client = self.get_client(ip_port);
        let mut buffer = vec![0u8;self.receive_buffer_size as usize];

        client.stream.write(&mut buffer);
        client.stream.flush();
    }

    fn Log(&self, msg: &str) {
        if self.logging {
            println!("{}", msg);
        }
    }

    async fn accept_connections(&self) -> Result<(), Box<dyn Error>> {
        match self.listener.accept() {
            (stream, tcp_client) => {
                let client_data: ClientData = ClientData { tcp_client, stream };

                self.add_client(client_data.tcp_client.ip().to_string(), client_data);
                self.Log(
                    "Connection established. Starting data receiver ["
                        + tcp_client.ip().to_string()
                        + "]",
                );
                Ok(())
            },
            None =>  Err("connection not established")
        }
    }

    fn is_client_connected(client: &mut ClientData) -> bool {
        let mut buffer = [0; 1024];
        match client.stream.read(&mut buffer) {
            Ok(bytes_read) => {
                println!("Read {:?}", &buffer[..bytes_read]);
                return true;
            }
            Err(e) if e.kind() == ErrorKind::ConnectionAborted => {
                println!("Client disconnected");
                return false;
            }
            Err(e) => {
                println!("Some error occured: {e}");
                return false;
            }
        }
    }

    async fn data_receiver(&self, client: ClientData) {
        loop {
            if !TcpServer::is_client_connected(&mut client) {
                break;
            }
            //
        }
        self.remove_client(&client.tcp_client.ip().to_string());
        self.Log(
            ("[" + &client.tcp_client.ip().to_string()
                + "] DataReceiver disconnect detected").to_string(),
        );
    }
}
