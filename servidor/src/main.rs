use std::sync::LazyLock;
use tokio::sync::{RwLock, broadcast, mpsc};

use std::net::SocketAddr;
use tokio::net::{TcpStream, TcpListener};

use tokio::io::{AsyncReadExt, AsyncWriteExt};

use protocolo::mensajes_servidor::*;
use protocolo::EstadoUsuario::*;
use protocolo::ClientType::*;
use protocolo::*;

use std::collections::{HashMap, HashSet};
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

type Cuartos = HashMap<String, Cuarto>;
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
 * <br>
 * `d` - La dirección del cliente.
 */
async fn maneja_usuario(mut ts: TcpStream, d: SocketAddr) {
    
    let nom: String = match espera_identificacion(&mut ts, &d).await {
	Ok(None) => return,
	Ok(Some(name)) => name,
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

    nuevo_usuario(&nom).await;
    
    USUARIOS.write().await.insert(nom.clone(), ACTIVE);

    let (sender, mut receiver) = mpsc::channel::<String>(128);
    CLIENTES.write().await.insert(nom.clone(), sender);

    join_main_room(&nom).await;
    
    loop {
	tokio::select!{
	    recv = receiver.recv() => {
		match recv {
		    None => {
			desconecta(&nom).await;
			return;
		    },
		    Some(msg) => {
			if let Err(e) = envia(&d, &mut ts, Some(&nom), msg).await {
			    bitacora::error(e);
			    desconecta(&nom).await;
			    return;
			}
		    },
		}
	    }
	    msg = recibe(&d, &mut ts, Some(&nom)) => {
		match msg {
		    Ok(None) => {
			desconecta(&nom).await;
			return;
		    },
		    Err(e) => {
			bitacora::error(e);
			desconecta(&nom).await;
			return;
		    },
		    Ok(Some(rec)) => {
			match parsea_mensaje_cliente(rec) {
			    Err(_) => {
				bitacora::error(Invalido{direccion: d, nombre: Some(nom.clone())});
				if let Err(e2) = envia(&d, &mut ts, Some(&nom),
						       response("INVALID", "INVALID")).await {
				    bitacora::error(e2);
				}
				desconecta(&nom).await;
				return;
			    },
			    Ok(ct) => {
				if let Err(_) = maneja_solicitud(ct, &mut ts, &d, &nom).await {
				    return;
				}
			    },
			}
		    },
		}
	    }
	}
    }
}

/**
 * Espera a que el cliente se identifique, en cuyo caso se regresa el
 * nombre con que lo ha hecho.
 *
 * # Argumentos
 *
 * `ts` - El `TcpStream` para comunicarse con el cliente.
 * <br>
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
	    
	    Ok(Identify{ username: nom }) => {
		
		if USUARIOS.read().await.contains_key(&nom) {
		    if let Err(e) = envia(d, ts, None, response_extra("IDENTIFY",
								      "USER_ALREADY_EXISTS",
								      &nom)).await {
			return Err(e);
		    }
		    continue;
		}
		if let Err(e) = envia(d, ts, Some(&nom), response_extra("IDENTIFY",
								      "SUCCESS",
								      &nom)).await {
		    return Err(e);
		}
		return Ok(Some(nom));
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
 * <br>
 * `ts` - El `TcpStream` para mandar mensajes al cliente.
 * <br>
 * `d` - La dirección ip del cliente.
 * <br>
 * `nom` - Un String con el nombre del cliente.
 */
async fn maneja_solicitud(ct: ClientType, ts: &mut TcpStream, d: &SocketAddr, nom: &String)
			  -> Result<Option<String>, ErrorServidor> {
    match ct {
	Identify { username: u } => return Err(Reidentify { direccion: *d,
							    nombre: nom.clone() }),
	Status { status: eu } => {},
	Users => {},
	Text { username: u, text: t } => {},
	PublicText { text: t } => {},
	NewRoom { roomname: rn } => {},
	Invite { roomname: rn, usernames: us } => {},
	JoinRoom { roomname: rn } => {},
	RoomUsers { roomname: rn } => {},
	RoomText { roomname: rn, text: t } => {},
	LeaveRoom { roomname: rn } => {},
	Disconnect => {},
    }
    Ok(None)
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

/**
 * Mete a un usuario a un cuarto, si este fue invitado.
 *
 * `rn` - Un String con el nombre del cuarto.
 * `nom` - Un String con el nombre del usuario.
 */
async fn join_room(rn: &String, nom: &String) -> String {
    let mut room_receiver;
    match CUARTOS.read().await.get(rn) {
	None => return response_extra("JOIN_ROOM", "NO_SUCH_ROOM", rn),
	Some(room) => {
	    if room.es_invitado(nom) {
		let room_sender = room.sender();
		room_receiver = room_sender.subscribe();
	    } else {
		return response_extra("JOIN_ROOM", "NOT_INVITED", rn);
	    }
	},
    }
    let sender_cliente = CLIENTES.read().await.get(nom).unwrap().clone();
    let roomname = rn.clone();
    let nombre = nom.clone();
    tokio::spawn(async move {
	loop {
	    if !CUARTOS.read().await.get(&roomname).unwrap().es_miembro(&nombre) { return; }
	    match room_receiver.recv().await {
		Ok(msg) => {
		    if let Err(_) = sender_cliente.send(msg).await { return; }
		},
		Err(broadcast::error::RecvError::Closed) => { return; },
		Err(broadcast::error::RecvError::Lagged(msgs)) => {
		    let mut missed_msgs: u64 = msgs;
		    while missed_msgs > 0 {
			match room_receiver.recv().await {	
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
    return response_extra("JOIN_ROOM", "SUCCESS", rn);
}

/**
 * Avisa a todos los usuarios de la llegada de un nuevo usuario.
 *
 * # Argumentos
 *
 * `nom` - Un String con el nombre del usuario que se acaba de conectar.
 */
async fn nuevo_usuario(nom: &String) {
    let clientes = CLIENTES.read().await;
    for (_, sender) in clientes.iter() {
	if let Err(_) = sender.send(new_user(nom)).await { continue; }
    }
}

/**
 * Desconecta al cliente del servidor y le avisa a todos los usuarios de esto.
 *
 * # Argumentos
 *
 * `nom` - Un String que 
 */
async fn desconecta(nom: &String) {
    let clientes = CLIENTES.read().await;
    for (_, sender) in clientes.iter() {
	if let Err(_) = sender.send(disconnected(nom)).await { continue; }
    }

    let mut cuartos = CUARTOS.write().await;
    for (rn, cuarto) in cuartos.iter_mut() {
	if cuarto.es_miembro(nom) {
	    cuarto.salio(nom);
	    if let Err(_) = cuarto.send(left_room(rn, nom)).await { continue; }
	}
    } 
}

pub mod bitacora;
pub mod util;
