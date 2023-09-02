use std::collections::HashMap;
use std::io::{self, prelude::*, ErrorKind};
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::sync::mpsc::{channel, Sender};
use std::sync::{Arc, Mutex};
use std::thread;

type ClientData = (SocketAddr, TcpStream);
type EventSender = Arc<Mutex<Option<Sender<ServerEvent>>>>;

/// Data that is sent to user provided FnMut `set in fn start` when a client connects to a
/// started(fn start) server
pub type HandleClientType<'a, T> = (&'a mut ClientData, Option<T>, &'a mut TcpServer<T>);

#[allow(dead_code)]
pub enum ServerEvent {
    None,
    Disconnection(SocketAddr),
}

#[derive(Clone)]
pub struct TcpServer<T: Clone + Send + 'static> {
    listener_ip: Arc<String>,
    clients: Arc<Mutex<HashMap<String, ClientData>>>,
    running: bool,
    logging: bool,
    event_sender: EventSender,
    handle_type: Option<T>,
}

impl<T> TcpServer<T>
where
    T: Clone + Send + 'static,
{
    pub fn new(ip: &str, port: u16) -> Self {
        let mut ip_port: String = ip.to_string();
        ip_port.push_str(&port.to_string());
        TcpServer::<T> {
            listener_ip: Arc::new(ip_port),
            clients: Arc::new(Mutex::new(HashMap::new())),
            running: false,
            logging: true,
            event_sender: Default::default(),
            handle_type: Default::default(),
        }
    }

    fn get_clients(&mut self) -> Arc<Mutex<HashMap<String, ClientData>>> {
        return Arc::clone(&self.clients);
    }

    fn add_client(&mut self, ip_port: String, client: ClientData) -> io::Result<()> {
        self.clients.lock().unwrap().insert(ip_port, client);
        Ok(())
    }

    fn remove_client(&mut self, ip_port: &str) -> io::Result<()> {
        self.clients.lock().unwrap().remove(ip_port);
        Ok(())
    }

    fn send_event(&self, event: ServerEvent) {
        let sender = self.event_sender.as_ref();

        if let Some(sender) = &*sender.lock().unwrap() {
            sender
                .send(event)
                .unwrap_or_else(|e| println!("error sending to error_sender {}", e));
        }
    }

    fn start(&mut self) {
        if self.running {
            println!("server is running");
            return;
        }
        self.running = true;
        let listener = TcpListener::bind(&*self.listener_ip).unwrap();

        //listener.set_nonblocking(true);
        let mut self_ = self.clone();
        thread::spawn(move || loop {
            match listener.accept() {
                Ok((stream, tcp_client)) => {
                    let client_data: ClientData =  (tcp_client, stream );
                    let result =
                        self_.add_client(tcp_client.ip().to_string(), client_data);
                    let log_str = format!(
                        "Connection established. Starting data receiver [{}]{:?}",
                        tcp_client.ip(),
                        result
                    );
                    self_.log(&log_str);
                    let mut func = |client| Self::data_receiver(&mut self_, client);
                    func(&client_data);
                    /* .unwrap_or_else(|e| {
                       println!("Error in handle_client : {}", e);
                    });*/
                }
                Err(_) => println!("Error while connecting"),
            }
        });
    }

    fn log(&self, msg: &str) {
        if self.logging {
            println!("{}", msg);
        }
    }

    /// Runs the user's defined function in a new thread passing in the newly connected socket.
    fn handle_client(
        &mut self,
        socket_data: ClientData,
        handle_client: impl Fn(HandleClientType<T>) + Send + 'static,
    ) -> io::Result<()> {
        let (index, socket) = socket_data;
        let mut socket = socket.try_clone()?;
        let mut _self = self.clone();
        thread::spawn(move || {
            handle_client((&mut socket_data, _self.handle_type.clone(), &mut _self));
            _self
                .handle_socket_disconnection(&(socket_data))
                .unwrap_or_else(|why| {
                    panic!("Error in handling socket disconnection, Err: '{}' ", why);
                });
        });
        Ok(())
    }

    fn handle_socket_disconnection(&mut self, socket_data: &ClientData) -> io::Result<()> {
        let (addr, socket) = socket_data;

        //Remove socket from clients list
        self.remove_client(&addr.to_string())?;
        let sock_addr = socket.peer_addr().unwrap();

        self.send_event(ServerEvent::Disconnection(sock_addr));

        let log_str = format!("Removed client {}, at index {}", sock_addr, addr);
        self.log(&log_str);
        Ok(())
    }

    fn is_client_connected(&self, client: &mut ClientData) -> bool {
        let mut buffer = [0; 1024];
        let (addr, socket) = client;
        match socket.read(&mut buffer) {
            Ok(bytes_read) => {
                println!("Read {:?}", &buffer[..bytes_read]);
                return true;
            }
            Err(e) if e.kind() == ErrorKind::ConnectionAborted => {
                println!("Client disconnected");
                return false;
            }
            Err(e) => {
                println!("Some error occurred: {e}");
                return false;
            }
        }
    }

    fn data_receiver(&mut self, mut client: &ClientData) {
        let (addr, socket) = client;
        loop {
            if !self.is_client_connected(&mut client) {
                break;
            }
        }
        let result = self.remove_client(&addr.ip().to_string());
        let log_str = format!(
            "[{}] DataReceiver disconnect detected {:?}",
            &addr.ip().to_string(),
            result
        );
        self.log(&log_str);
    }
}
