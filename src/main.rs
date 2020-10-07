use std::net::{SocketAddr, TcpStream};
use std::sync::mpsc::channel;
use std::thread;

fn main() {
    let ip = [192, 168, 0, 18];
    let port_range_start = 0;
    let port_range_end = 1024;
    let port_range = port_range_start..port_range_end;
    let (tx, rx) = channel();
    for port in port_range {
        let tx = tx.clone();
        let socket_address = SocketAddr::from((ip, port));
        thread::spawn(move || {
            TcpStream::connect(socket_address)
                .and_then(|_| {
                    tx.send(Some(socket_address)).ok();
                    Ok(())
                })
                .or_else(|e| {
                    tx.send(None).ok();
                    Err(e)
                })
        });
    }

    let mut total_ports_open = vec![];

    for _ in port_range_start..port_range_end {
        rx
            .recv()
            .ok()
            .map(|port| port.map(|p| total_ports_open.push(p)) );
    }
    
    println!("ports open {:?}", total_ports_open);
}
