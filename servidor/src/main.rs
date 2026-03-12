use std::sync::LazyLock;
use tokio::sync::{RwLock, broadcast, mpsc};

use std::net::SocketAddr;
use tokio::net::{TcpStream, TcpListener};

use tokio::io::{AsyncReadExt, AsyncWriteExt};

use protocolo::mensajes_servidor::*;
use protocolo::EstadoUsuario::*;
use protocolo::ClientType::*;
use protocolo::*;

use std::collections::HashMap;
use std::option::Option;

use bitacora::ErrorServidor::*;
use bitacora::*;

use util::*;

type Users = HashMap<String, EstadoUsuario>;
static USUARIOS: LazyLock<RwLock<Users>> =
    LazyLock::new(|| {RwLock::new(HashMap::new())});

type Clientes = HashMap<String, mpsc::Sender<String>>;
static CLIENTES: LazyLock<RwLock<Clientes>> =
    LazyLock::new(|| {RwLock::new(HashMap::new())});

type Cuartos = HashMap<String, broadcast::Sender<String>>;
static CUARTOS: LazyLock<RwLock<Cuartos>> =
    LazyLock::new(|| {RwLock::new(HashMap::new())});

static MAINROOM: LazyLock<RwLock<broadcast::Sender<String>>> =
    LazyLock::new(|| {RwLock::new(broadcast::channel::<String>(256).0)});

/* *
 * Crea el servidor y acepta clientes.
 */
#[tokio::main]
async fn main() {
    
    let direccion_servidor = server_address();
    let servidor = match TcpListener::bind(&direccion_servidor).await {
	Ok(a) => a,

	Err(e) => {
	    bitacora::error(Creacion { error: e,
				       direccion: direccion_servidor });
	    return;
	},
    };
    
    loop {
	match servidor.accept().await {
	    Ok((stream, direccion)) => {
		tokio::spawn(maneja_usuario(stream, direccion));
	    }
	    Err(e) => {
		bitacora::error(Aceptacion { error: e });
	    }
	}
    }
}

/**
 * Maneja la conexión de cada cliente.
 *
 * # Argumentos
 *
 * `ts` - El `TcpStream` para leer y escribir información del cliente.
 * `d` - La dirección del cliente.
 */
async fn maneja_usuario(mut ts: TcpStream, d: SocketAddr) {
    
    let name: String = match espera_identificacion(&mut ts, &d).await {
	Ok(None) => return,
	Ok( Some(u) ) => u,
	Err( e @ Invalido{ direccion: d, nombre: None } ) => {
	    bitacora::error(e);
	    if let Err(e2) = envia(&d, &mut ts, None, response("INVALID", "INVALID")).await {
		bitacora::error(e2);
	    }
	    return;
	},
	Err(e) => {
	    bitacora::error(e);
	    return;
	}
    };
    USUARIOS.write().await.insert(name.clone(), ACTIVE);

    let (sender, mut receiver) = mpsc::channel::<String>(128);
    CLIENTES.write().await.insert(name.clone(), sender);

    join_main_room(&name);
    
    loop {
	tokio::select!{
	    recv = receiver.recv() => {
		println!("Nada");
	    }
	    msg = recibe(&d, &mut ts, Some(&name)) => {
		println!("Nadota");
	    }
	}
    }
    
    USUARIOS.write().await.remove(&name);
    CLIENTES.write().await.remove(&name);
}

/**
 * Espera a que el cliente se identifique, en cuyo caso se regresa el
 * nombre con que lo ha hecho.
 *
 * # Argumentos
 *
 * `ts` - El `TcpStream` para comunicarse con el cliente.
 * `d` - La dirección IP del cliente.
 */
async fn espera_identificacion(ts: &mut TcpStream, d: &SocketAddr)
			     -> Result<Option<String>, ErrorServidor> {

    loop {
	let rec = match recibe(d, ts, None).await {
	    Ok(None) => return Ok(None),
	    Err(e) => return Err(e),
	    Ok(Some(msg)) => msg
	};

	match parsea_mensaje_cliente(rec) {
	    Err(_) => return Err(Invalido{ direccion: *d,
					   nombre: None }),
	    
	    Ok(Identify{ username: u }) => {
		
		if USUARIOS.read().await.contains_key(&u) {
		    if let Err(e) = envia(d, ts, None, response_extra("IDENTIFY",
								      "USER_ALREADY_EXISTS",
								      &u)).await {
			return Err(e);
		    }
		    continue;
		}		
		if let Err(e) = envia(d, ts, Some(&u), response_extra("IDENTIFY",
								      "SUCCESS",
								      &u)).await {
		    return Err(e);
		}
		return Ok(Some(u));
	    },

	    Ok(_) => {
		if let Err(e) = envia(d, ts, None, response("INVALID",
							    "NOT_IDENTIFIED")).await {
		    return Err(e);
		}
	    },
	}
    }
}

/**
 * Realiza las acciones acorde a la solicitud del cliente.
 *
 * # Argumentos
 *
 * `ct` - una instancia de `ClientType` asociada a la instrucción que se
 *        desea realizar y que contiene lo necesario para realizarla.
 */
async fn maneja_solicitud(ct: ClientType) {
    println!("NO HAY IMPLEMENTACIÓN DE NADA");
}

/**
 * Mete al usuario al cuarto principal.
 *
 * # Argumentos
 *
 * `nom` - Un String que contiene el nombre del usuario.
 */
async fn join_main_room(nom: &String) {
    let main_sender = MAINROOM.read().await.clone();
    let mut main_receiver = main_sender.subscribe();
    let sender_cliente = CLIENTES.read().await.get(nom).unwrap().clone();
    tokio::spawn(async move {
	loop {
	    match main_receiver.recv().await {
		Ok(msg) => {
		    if let Err(_) = sender_cliente.send(msg).await { return; }
		},
		Err(broadcast::error::RecvError::Closed) => { return; },
		Err(broadcast::error::RecvError::Lagged(msgs)) => {
		    let mut missed_msgs: u64 = msgs;
		    while missed_msgs > 0 {
			match main_receiver.recv().await {	
			    Ok(msg) => {
				if let Err(_) = sender_cliente.send(msg).await { return; }
				missed_msgs -= 1;
			    },
			    Err(broadcast::error::RecvError::Closed) => { return; },
			    Err(broadcast::error::RecvError::Lagged(m)) => { missed_msgs += m; },
			}
		    }
		},
	    }
	}
    });
}

pub mod bitacora;
pub mod util;
