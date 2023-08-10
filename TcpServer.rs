use polling::{Event, Poller};
use std::cell::Cell;
use std::collections::HashMap;
use std::io::{prelude::*, ErrorKind};
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::rc::Rc;
use std::sync::mpsc::channel;
use std::sync::{Arc, Mutex};
use std::thread;
use std::{ptr, task};

/*type ClientConnected = fn(param:str,param1:bool);
type ClientDisconnected = fn(param:str,param1:bool);
type DataReceived = fn(param:str,param1:vec,param2:bool);*/

struct ClientData {
    tcpClient: SocketAddr,
    stream: TcpStream,
}

struct TcpServer {
    receiveBufferSize: u16,
    listenerIp: u16,
    port: u16,
    clients: HashMap<String, Arc<Mutex<ClientData>>>,
    listener: TcpListener,
    running: bool,
    logging: bool,
}

impl TcpServer {
    fn new(ip: &str, port: u16) -> Self {
        Self {
            receiveBufferSize: 4096,
            listenerIp: ip,
            port: port,
            clients: HashMap::new(),
            listener: None,
            running: false,
            logging: true,
        }
    }

    fn get_client(&self, ipPort: &str) -> ClientData {
        return &*self.clients[ipPort];
    }

    fn add_client(&mut self, ipPort: String, client: ClientData) {
        self.clients.insert(ipPort, Arc::new(Mutex::new(client)));
    }

    fn remove_client(&mut self, ipPort: &str) {
        self.clients.remove(ipPort);
    }

    fn start(&self) {
        if (self.running) {
            println!("server is running");
            return;
        }
        self.running = true;
        self.listener = TcpListener::bind(self.listenerIp + ":" + self.port).unwrap();

        self.listener.set_nonblocking(true);

        self.accept_connections();
    }

    // Asynchronously send data to the specified client by IP:port.
    async fn send(&self, ipPort: &str, data: Vec) {
        let client = self.get_client(ipPort);

        client.NetworkStream.Write(data, 0, data.Length);
        client.NetworkStream.Flush();
    }

    fn Log(&self, msg: &str) {
        if self.logging {
            println!("{}", msg);
        }
    }

    async fn accept_connections(&self) -> Result<(), Box<dyn Error>> {
        let (stream, tcpClient) = self.listener.accept().unwrap();
        let clientData: ClientData = ClientData {
            tcpClient,
            stream
        };

        self.add_client(clientIp, clientMetadata);
        self.Log(
            "Connection established. Starting data receiver [" + tcpClient.ip().to_string() + "]",
        );
        if (ClientConnected != null) {
            task::spawn(self.client_connected(clientIp));
        }
        Ok(())
    }

    fn is_client_connected(&self, client: ClientData) -> bool {
        let mut buffer = [0; 1024];
        match client.NetworkStream.read(&mut buffer) {
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
        loop{
            if !self.is_client_connected(client) {
                break;
            }
            //
        }
        self.remove_client(client.TcpClient.Client.RemoteEndPoint.ToString());
        self.Log(
            "[" + client.TcpClient.Client.RemoteEndPoint.ToString()
                + "] DataReceiver disconnect detected",
        );
    }
}
