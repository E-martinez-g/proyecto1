use std::sync::LazyLock;
use tokio::sync::RwLock;

use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use tokio::net::{TcpStream, TcpListener};
use tokio::io::{AsyncReadExt, AsyncWriteExt};


use protocolo::mensajes_servidor::*;
use protocolo::ClientType::*;
use protocolo::*;

use std::collections::{HashMap, HashSet};

type Usernames = LazyLock<RwLock<HashSet<String>>>;

static NOMBRES: Usernames = LazyLock::new(|| {RwLock::new(HashSet::new())});

#[tokio::main]
async fn main() {
    
    let direccion_servidor = server_address();
    let servidor = match TcpListener::bind(&direccion_servidor).await {
	Ok(a) => a,

	Err(_) => {
	    eprintln!("No se pudo crear el servidor en {}", direccion_servidor);
	    return;
	},
    };

    loop {
	match servidor.accept().await {
	    Ok((stream, direccion)) => {
		tokio::spawn(maneja_usuario(stream, direccion));
	    }
	    Err(e) => {
		eprintln!("Ocurrió un error al aceptar una conexión ({})", e);
	    }
	}
    }
}

/**
 * Maneja la conexión de cada cliente.
 *
 * # Argumentos
 *
 * `stream` - El `TcpStream` para leer y escribir información del cliente.
 * `direccion` - La dirección del cliente.
 */
async fn maneja_usuario(mut stream: TcpStream, direccion: SocketAddr) {
    let mut buffer = [0u8; 1024];
    
    let name;
    
    loop {
	let n = match stream.read(&mut buffer).await {
	    Ok(0) => return,
	    Ok(n) => n,
	    Err(e) => {
		eprintln!("Al leer de {} ocurrió un error {}.",
			  direccion, e);
		return;
	    },
	};
	match parsea_mensaje_cliente(String::from_utf8_lossy(&buffer[..n])
				     .to_string()) {
	    Err(_) => {
		eprintln!("El mensaje recibido fue inválido");
		return;
	    },
	    Ok(Identify {username: usr}) => {
		if NOMBRES.read().await.contains(&usr) {
		    if let Err(_) =
			stream.write(response_extra("IDENTIFY".to_string(),
						  "USER_ALREADY_EXISTS".to_string(),
						  &usr).as_bytes()).await {
			    eprintln!("Ocurrió un error al responder a {}.",
				      direccion);
			    return;
			}
		    continue;
		}
		if let Err(_) =
		    stream.write(response_extra("IDENTIFY".to_string(),
						"SUCCESS".to_string(),
						&usr).as_bytes()).await {
			eprintln!("Ocurrió un error al responder a {}.",
				  direccion);
			return;
		    }
		name = usr.clone();
		NOMBRES.write().await.insert(usr);
		break;
	    }
	    Ok(_) => {
		if let Err(_) = stream.write(response("INVALID".to_string(),
						      "NOT_IDENTIFIED".to_string())
					     .as_bytes()).await {
		    eprintln!("Ocurrió un error al responder a {}.",
			      direccion);
		    return;
		}
	    }
	}
    }
    loop {
	let n = match stream.read(&mut buffer).await {
	    Ok(0) => break,
	    Ok(n) => n,
	    Err(e) => {
		eprintln!("Al leer de {} ocurrió un error {}.",
			  direccion, e);
		break;
	    },
	};

	match parsea_mensaje_cliente(String::from_utf8_lossy(&buffer[..n])
				     .to_string()) {
	    Err(_) => {
		eprintln!("El mensaje enviado por el cliente {} fue inválido.",
			  direccion);
		break;
	    },
	    Ok(Identify {..}) => {
		eprintln!("El cliente {} se intentó identificar dos veces",
			  direccion);
	    }
	    Ok(ct) => {
		maneja_solicitud(ct);
	    },
	}
    }
    NOMBRES.write().await.remove(&name);
}

/**
 * Realiza las acciones acorde a la solicitud del cliente.
 *
 * # Argumentos
 *
 * `ct` - una instancia de `ClientType` asociada a la instrucción que se
 *        desea realizar y que contiene lo necesario para realizarla.
 */
fn maneja_solicitud(ct: ClientType) {
    match ct {
	_ => println!("No hay implementación de nada.")
    }
}
