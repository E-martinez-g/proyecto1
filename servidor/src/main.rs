use std::env::args;
use std::net::IpAddr;
use std::net::Ipv4Addr;
use tokio::net::TcpStream;
use tokio::net::TcpListener;
use protocolo::mensajes_servidor;
use protocolo::EstadoUsuario;
use std::collections::HashMap;
use std::collections::HashSet;

#[tokio::main]
async fn main() {

    let mut nombres: HashSet<String> = HashSet::new();
    let mut usuarios: HashMap<String, EstadoUsuario> = HashMap::new();
    let mut clientes: HashMap<String, TcpStream> = HashMap::new();
    
    let address = socket_address();
    let listener = match TcpListener::bind(&address).await {
	Ok(a) => a,

	Err(_) => { eprintln!("No se pudo crear el servidor en {}", address);
		    return; },
    };
}

/**
 * Provee la dirección del socket a partir de los argumentos del programa.
 * El puerto por omisión es 42069.
 * La dirección IP por omisión es 127.0.0.1.
 */
fn socket_address() -> String {
    let ip = args().nth(1).unwrap_or_default()
	     .parse::<IpAddr>().unwrap_or(IpAddr::V4(Ipv4Addr::LOCALHOST));
    
    let mut port = args().nth(2).unwrap_or_default()
	           .parse::<u16>().unwrap_or_default();
    if port < 1024 { port = 42069; }
    
    format!("{}:{}", ip, port)
}
