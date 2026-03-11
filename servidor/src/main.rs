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

type Users = HashMap<String, EstadoUsuario>;
static USUARIOS: LazyLock<RwLock<Users>> =
    LazyLock::new(|| {RwLock::new(HashMap::new())});

type Clientes = HashMap<String, mpsc::Receiver<String>>;
static CLIENTES: LazyLock<RwLock<Clientes>> =
    LazyLock::new(|| {RwLock::new(HashMap::new())});

type Cuartos = HashMap<String, broadcast::Sender<String>>;
static CUARTOS: LazyLock<RwLock<Cuartos>> =
    LazyLock::new(|| {RwLock::new(HashMap::new())});

/**
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

    let (_, main_sender) = broadcast::channel::<String>(256);
    CUARTOS.write().await.insert("MAIN", main_sender);
    
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
    
    let name = match espera_identificacion(&mut ts, &d).await {
	Ok(None) => return,
	Ok( Some(u) ) => u,
	Err( e @ Invalido{ direccion: d, nombre: None } ) => {
	    bitacora::error(e);
	    let env = response("INVALID", "INVALID");
	    bitacora::enviado(&env, &d, None);
	    if let Err(e) = ts.write(env.as_bytes()).await {
		bitacora::error(Envio{ error: e, direccion: d,
				       nombre: None });
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
	    Ok(msg) = 
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
    
    let mut buffer = [0u8; 512];

    loop {
	let n = match ts.read(&mut buffer).await {
	    Ok(0) => return Ok(None),
	    Ok(a) => a,
	    Err(e) => return Err(Recepcion{ error: e, direccion: *d,
					    nombre: None }),
	};
	
	let rec = String::from_utf8_lossy(&buffer[..n]).to_string();
	bitacora::recibido(&rec, d, None);
	
	match parsea_mensaje_cliente(rec) {
	    Err(_) => return Err(Invalido{ direccion: *d,
					   nombre: None }),
	    
	    Ok(Identify{ username: u }) => {
		
		if USUARIOS.read().await.contains_key(&u) {
		    let env = response_extra("IDENTIFY",
					     "USER_ALREADY_EXISTS",
					     &u);
		    bitacora::enviado(&env, d, None);
		    if let Err(e) = ts.write(env.as_bytes()).await {
			return Err(Envio{ error: e, direccion: *d,
					  nombre: None});
		    }
		    continue;
		}		

		let env = response_extra("IDENTIFY", "SUCCESS", &u);
		bitacora::enviado(&env, d, Some(&u));
		if let Err(e) = ts.write(env.as_bytes()).await {
		    return Err(Envio{ error: e, direccion: *d,
				      nombre: None });
		}
		return Ok( Some(u) );
	    },

	    Ok(_) => {
		let env = response("INVALID", "NOT_IDENTIFIED");
		bitacora::enviado(&env, d, None);
		if let Err(e) = ts.write(&env.as_bytes()).await {
		    return Err(Envio{ error: e, direccion: *d,
				      nombre: None});
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
fn maneja_solicitud(ct: ClientType) {}

/**
 * Mete al usuario al cuarto principal.
 *
 * # Argumentos
 *
 * `nom` - Un String que contiene el nombre del usuario.
 */
pub join_main_room(nom: &String) {
    let main_receiver = CUARTOS.read().await.get("MAIN").unwrap().subscribe();
    let sender_cliente = CLIENTES.read().await.get(nom).unwrap().clone();
    tokio::spawn(async move {
	
    }) 
}

pub mod bitacora;
pub mod util;
