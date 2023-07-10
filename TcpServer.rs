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

pub struct CancellationSource {
    token: Rc<Cell<bool>>,
}

impl CancellationSource {
    pub fn new() -> Self {
        Self {
            token: Rc::new(Cell::new(false)),
        }
    }

    pub fn request_cancellation(&mut self) {
        self.token.set(true);
    }

    pub fn token(&self) -> CancellationToken {
        CancellationToken {
            token: Rc::clone(&self.token),
        }
    }
}

#[derive(Clone)]
pub struct CancellationToken {
    token: Rc<Cell<bool>>,
}

impl CancellationToken {
    pub fn is_cancellation_requested(&self) -> bool {
        self.token.get()
    }
}

struct ClientData {
    tcpClient: SocketAddr,
    stream: TcpStream,
    lock: Mutex<TcpStream>,
}

struct TcpServer {
    receiveBufferSize: u16,
    listenerIp: u16,
    port: u16,
    clients: HashMap<String, ClientData>,
    listener: TcpListener,
    running: bool,
    logging: bool,
    cancelToken: CancellationSource,
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
            cancelToken: CancellationSource::new(),
        }
    }

    fn get_client(&self, ipPort: &str) -> ClientData {
        return self.clients[ipPort];
    }

    fn add_client(&mut self, ipPort: String, client: ClientData) {
        self.clients.insert(ipPort, client);
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

        //task::Builder::new().name(self.cancelToken).spawn(self.AcceptConnections());
        //Task.Run(() => AcceptConnections(), _Token);
    }

    // Asynchronously send data to the specified client by IP:port.
    async fn send(&self, ipPort: &str, data: Vec) {
        //if (string.IsNullOrEmpty(ipPort)) throw new ArgumentNullException(nameof(ipPort));
        // if (data == null || data.Length < 1) throw new ArgumentNullException(nameof(data));

        let client = self.get_client(ipPort);

        //  lock (client.lock)
        // {
        client.NetworkStream.Write(data, 0, data.Length);
        client.NetworkStream.Flush();
        // }
    }

    fn Log(&self, msg: &str) {
        if self.logging {
            println!("{}", msg);
        }
    }

    async fn accept_connections(&self) -> Result<(), Box<dyn Error>> {
        while (!self.token.is_cancellation_requested()) {
            let (stream, tcpClient) = self.listener.accept().unwrap();
            let clientData: ClientData = ClientData { tcpClient, stream, lock: todo!() };

            self.add_client(clientIp, clientMetadata);
            self.Log(
                "Connection established. Starting data receiver ["
                    + tcpClient.ip().to_string()
                    + "]",
            );
            if (ClientConnected != null) {
                task::spawn(self.client_connected(clientIp));
            }
            //Task unawaited2 = Task.Run(() => DataReceiver(clientMetadata), _Token);
            //task::Builder::new().name("unawaited2".to_string()).spawn(DataReceiver(clientMetadata));
        }
        Ok(())
    }

    fn is_client_connected(&self, client: ClientData) -> bool {
        /*if (client.TcpClient.Connected)
        {
            if ((client.TcpClient.Client.Poll(0, SelectMode.SelectWrite)) && (!client.TcpClient.Client.Poll(0, SelectMode.SelectError)))
            {
                let buffer = new byte[1];
                if (client.TcpClient.Client.Receive(buffer, SocketFlags.Peek) == 0){
                    return false;
                }
                else{
                    return true;
                }
            }
            else {return false;}
        }
        else {return false;}*/
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
        while (true) {
            if !self.is_client_connected(client) {
                break;
            }

            /*byte[] data = await DataReadAsync(client);
            if (data == null)
            {
                // no message available
                await Task.Delay(30);
                continue;
            }

            if (DataReceived != null)
            {
                Task<bool> unawaited = Task.Run(() => DataReceived(client.TcpClient.Client.RemoteEndPoint.ToString(), data));
            }*/
        }
        self.remove_client(client.TcpClient.Client.RemoteEndPoint.ToString());
        self.Log("["
            + client.TcpClient.Client.RemoteEndPoint.ToString()
            + "] DataReceiver disconnect detected");
        //if (ClientDisconnected != null) await Task.Run(() => ClientDisconnected(client.TcpClient.Client.RemoteEndPoint.ToString()));
    }
}
