use std::sync::LazyLock;
use tokio::sync::{RwLock, mpsc};

use std::net::SocketAddr;
use tokio::net::{TcpStream, TcpListener};
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};

use protocolo::{mensajes_servidor::*, EstadoUsuario::*, ClientType::*, *};

use std::collections::HashMap;
use std::option::Option;

use bitacora::{*, ErrorServidor::*};

use util::*;


/**
 * Maneja la comunicación con un cliente.
 *
 * # Campos
 *
 * `buffer` - El `Vec<u8>` para recibir información del cliente. <br>
 * `lector` - El `BufReader` con la `OwnedReadHalf` para recibir información
 *            del cliente. <br>
 * `escritor` - La `OwnedWriteHalf` para mandar información al cliente.
 */
struct Conexion {
    direccion: SocketAddr,
    nombre: Option<String>,
    buffer: Vec<u8>,
    lector: BufReader<OwnedReadHalf>,
    escritor: OwnedWriteHalf,
}

impl Conexion {

    /**
     * Crea una `Conexion` con el cliente a partir de la conexión con este.
     *
     * # Argumentos
     *
     * `ts` - El `TcpStream` para comunicarse con el cliente.
     * `dir` - La dirección IP del cliente.
     * `nom` - El nombre con el que se identificó el cliente.
     */
    fn new(ts: TcpStream, dir: SocketAddr) -> Self {
	let (l, e) = ts.into_split();
	Conexion {
	    direccion: dir,
	    nombre: None,
	    buffer: vec![0u8;512],
	    lector: BufReader::new(l),
	    escritor: e,
	}
    }

    /**
     * Obtiene lo que el cliente envíe.
     */
    async fn cliente_in(&mut self) -> Result<Option<String>, ErrorServidor> {
	self.buffer.clear();
	let n = match self.lector.read_until(b'\0', &mut self.buffer).await {
	    Ok(0) => return Ok(None),
	    Ok(a) => a,
	    Err(e) => return Err(Recepcion{ error: e,
					    direccion: self.direccion.clone(),
					    nombre: self.nombre.clone() }),
	};
	if n > 512 { return Ok(None); }
	let rec = String::from_utf8_lossy(&self.buffer[..n]);
	recibido(&rec.to_string(), &self.direccion, self.nombre());
	Ok(Some(rec.trim_end_matches('\0').to_string()))
    }

    /**
     * Envía un mensaje al cliente.
     *
     * # Argumentos
     *
     * `msg` - Un `String` con el mensaje a enviar.
     */
    async fn cliente_out(&mut self, msg: String) -> Result<(), ErrorServidor> {
	enviado(&msg, &self.direccion, self.nombre.clone());
	if let Err(e) = self.escritor.write(msg.as_bytes()).await {
	    return Err(Recepcion{ error: e,
				  direccion: self.direccion.clone(),
				  nombre: self.nombre.clone() });
	}
	Ok(())
    }

    /**
     * Obtiene una copia de la dirección del cliente.
     */
    fn direccion(&self) -> SocketAddr {
	self.direccion.clone()
    }

    /**
     * Obtiene una copia del nombre del cliente.
     */
    fn nombre(&self) -> Option<String> {
	self.nombre.clone()
    }

    /**
     * Asigna un nombre al cliente.
     *
     * # Argumentos
     *
     * `nom` - El nombre a asignar.
     */
    fn asigna_nombre(&mut self, nom: String) {
	self.nombre = Some(nom);
    }
}

type Users = HashMap<String, EstadoUsuario>;
static USUARIOS: LazyLock<RwLock<Users>> =
    LazyLock::new(|| {RwLock::new(HashMap::new())});

type Clientes = HashMap<String, mpsc::Sender<String>>;
static CLIENTES: LazyLock<RwLock<Clientes>> =
    LazyLock::new(|| {RwLock::new(HashMap::new())});

type Cuartos = HashMap<String, Cuarto>;
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
    
    loop {
	tokio::select! {
	    result = servidor.accept() => {
		match result {
		    Ok((stream, direccion)) => {
			tokio::spawn(maneja_usuario(stream, direccion));
		    }
		    Err(e) => {
			bitacora::error(Aceptacion { error: e });
		    }
		}
	    },
	    _ = tokio::signal::ctrl_c() => {
		println!("** TERMINANDO LA EJECUCIÓN DEL SERVIDOR.");
		return;
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
async fn maneja_usuario(ts: TcpStream, d: SocketAddr) {

    let mut conexion = Conexion::new(ts, d);
    
    let nom: String = match espera_identificacion(&mut conexion).await {
	Ok(None) => return,
	Ok(Some(name)) => name,
	Err( e @ Invalido{ nombre: None, .. } ) => {
	    bitacora::error(e);
	    if let Err(e2) = conexion.cliente_out(response("INVALID",
							   "INVALID")).await {
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

    conexion.asigna_nombre(nom);
    
    USUARIOS.write().await.insert(conexion.nombre().unwrap(), Active);

    let (sender, mut receiver) = mpsc::channel::<String>(128);
    CLIENTES.write().await.insert(conexion.nombre().unwrap(), sender);
    
    loop {
	conexion.buffer.clear();
	tokio::select!{
	    recv = receiver.recv() => {
		match recv {
		    None => {
			desconecta(&conexion.nombre().unwrap()).await;
			return;
		    },
		    Some(msg) => {
			if let Err(e) = conexion.cliente_out(msg).await {
			    bitacora::error(e);
			    desconecta(&conexion.nombre().unwrap()).await;
			    return;
			}
		    },
		}
	    }
	    msg = conexion.cliente_in() => {
		match msg {
		    Ok(None) => {},
		    Err(e) => {
			bitacora::error(e);
			desconecta(&conexion.nombre().unwrap()).await;
			return;
		    },
		    Ok(Some(rec)) => {
			match parsea_mensaje_cliente(rec) {
			    Err(_) => {
				bitacora::error(Invalido{direccion: d, nombre: Some(conexion.nombre().unwrap())});
				if let Err(e2) = conexion.cliente_out( response("INVALID",
										"INVALID")).await {
				    bitacora::error(e2);
				}
				desconecta(&conexion.nombre().unwrap()).await;
				return;
			    },
			    Ok(Some(ct)) => {
				match maneja_solicitud(ct, &mut conexion).await {
				    Ok(None) => {},
				    Ok(Some(s)) => {
					if let Err(e) = conexion.cliente_out(s).await {
					    bitacora::error(e);
					    desconecta(&conexion.nombre().unwrap()).await;
					    return;
					}
				    },
				    Err(Desconectado) => return,
				    Err(e @ Invalido{..}) => {
					bitacora::error(e);
					if let Err(e2) = conexion.cliente_out(response("INVALID",
										       "INVALID")).await {
					    bitacora::error(e2);
					}
					desconecta(&conexion.nombre().unwrap()).await;
					return;
				    },
				    Err(e) => {
					bitacora::error(e);
					desconecta(&conexion.nombre().unwrap()).await;
					return;
				    },
				}
			    },
			    Ok(None) => {},
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
async fn espera_identificacion(conexion: &mut Conexion)
			     -> Result<Option<String>, ErrorServidor> {
    loop {
	let rec = match conexion.cliente_in().await {
	    Ok(None) => return Ok(None),
	    Err(e) => return Err(e),
	    Ok(Some(msg)) => msg
	};

	match parsea_mensaje_cliente(rec) {
	    Err(_) => return Err(Invalido{ direccion: conexion.direccion(), nombre: None }),
	    
	    Ok(Some(Identify{ username: nom })) => {

		if nom.chars().count() > 8 {
		    return Err(NombreInvalido{ direccion: conexion.direccion(),
					       nombre: None});
		}
		
		if USUARIOS.read().await.contains_key(&nom) {
		    if let Err(e) = conexion.cliente_out(response_extra("IDENTIFY",
									"USER_ALREADY_EXISTS",
									&nom)).await {
			return Err(e);
		    }
		    continue;
		}
		if let Err(e) = conexion.cliente_out(response_extra("IDENTIFY",
								    "SUCCESS",
								    &nom)).await {
		    return Err(e);
		}
		return Ok(Some(nom));
	    },
	    Ok(None) => {},
	    Ok(_) => {
		if let Err(e) = conexion.cliente_out(response("INVALID",
							    "NOT_IDENTIFIED")).await {
		    return Err(e);
		}
	    },
	}
    }
}

/**
 * Realiza las acciones acorde a la solicitud del cliente.
 * * # Argumentos
 *
 * `ct` - una instancia de `ClientType` asociada a la instrucción que se
 *        desea realizar y que contiene lo necesario para realizarla.
 * <br>
 * `d` - La dirección ip del cliente.
 * <br>
 * `nom` - Un String con el nombre del cliente.
 */
async fn maneja_solicitud(ct: ClientType, conexion: &mut Conexion)
			  -> Result<Option<String>, ErrorServidor> {
    match ct {
	Identify { .. } => return Err(Reidentify { direccion: conexion.direccion(),
						   nombre: conexion.nombre().unwrap() }),
	Status { status: eu } => {
	    let no_cambio = USUARIOS.read().await.get(&conexion.nombre().unwrap()).unwrap() == &eu;
	    if no_cambio { return Ok(None); }
	    USUARIOS.write().await.insert(conexion.nombre().unwrap(), eu.clone());
	    todos_menos(new_status(&conexion.nombre().unwrap(), &eu), &conexion.nombre().unwrap()).await;
	},
	
	Users => return Ok(Some(user_list(&*USUARIOS.read().await))),
	
	Text { username: u, text: t } => {
	    if u.chars().count() > 16 {
		return Err(NombreInvalido{ direccion: conexion.direccion(),
					   nombre: Some(conexion.nombre().unwrap()) });
	    }
	    return Ok(mensaje_privado(u, t, &conexion.nombre().unwrap()).await);
	},
	
	PublicText { text: t } => { todos_menos(public_text_from(&conexion.nombre().unwrap(), t),
						&conexion.nombre().unwrap()).await; },

	NewRoom { roomname: rn } => {
	    if rn.chars().count() > 16 {
		return Err(NombreInvalido{ direccion: conexion.direccion(),
					   nombre: Some(conexion.nombre().unwrap()) });
	    }
	    return Ok(Some(crea_cuarto(rn, &conexion.nombre().unwrap()).await));
	},

	Invite { roomname: rn, usernames: us } => {
	    if rn.chars().count() > 16 {
		return Err(NombreInvalido{ direccion: conexion.direccion(),
					   nombre: Some(conexion.nombre().unwrap()) });
	    }
	    return invitaciones(us, rn, &conexion.nombre().unwrap(), &conexion.direccion()).await;
	},

	JoinRoom { roomname: rn } =>{
	    if rn.chars().count() > 16 {
		return Err(NombreInvalido{ direccion: conexion.direccion(),
					   nombre: Some(conexion.nombre().unwrap()) });
	    }
	    return Ok(Some(join_room(&rn, &conexion.nombre().unwrap()).await));
	},

	RoomUsers { roomname: rn } => {
	    if rn.chars().count() > 16 {
		return Err(NombreInvalido{ direccion: conexion.direccion(),
					   nombre: Some(conexion.nombre().unwrap()) });
	    }
	    return Ok(Some(usuarios_cuarto(rn, &conexion.nombre().unwrap()).await));
	},

	RoomText { roomname: rn, text: t } => {
	    if rn.chars().count() > 16 {
		return Err(NombreInvalido{ direccion: conexion.direccion(),
					   nombre: Some(conexion.nombre().unwrap()) });
	    }
	    return Ok(mensaje_cuarto(rn, t, &conexion.nombre().unwrap()).await);
	},

	LeaveRoom { roomname: rn } => {
	    if rn.chars().count() > 16 {
		return Err(NombreInvalido{ direccion: conexion.direccion(),
					   nombre: Some(conexion.nombre().unwrap()) });
	    }
	    return Ok(abandonar_cuarto(rn, &conexion.nombre().unwrap()).await);
	},
	
	Disconnect => {
	    desconecta(&conexion.nombre().unwrap()).await;
	    return Err(Desconectado); 
	},
    }
    Ok(None)
}

/**
 * Mete a un usuario a un cuarto, si este fue invitado.
 *
 * `rn` - Un String con el nombre del cuarto.
 * <br>
 * `nom` - Un String con el nombre del usuario.
 */
async fn join_room(rn: &String, nom: &String) -> String {
    match CUARTOS.write().await.get_mut(rn) {
	None => {
	    return response_extra("JOIN_ROOM", "NO_SUCH_ROOM", rn);
	},
	Some(room) => {
	    if !room.es_invitado(nom) {
		return response_extra("JOIN_ROOM", "NOT_INVITED", rn);
	    } else {
		room.se_unio(nom.clone());
	    }
	},
    }
    todos_menos_cuarto(joined_room(rn, nom), rn, nom).await;
    response_extra("JOIN_ROOM", "SUCCESS", &rn)
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
 * Envía un mensaje privado a un usuario seleccionado.
 *
 * # Argumentos
 *
 * `des` - El nombre del usuario al que se quiere mandar el mensaje.
 * <br>
 * `msg` - El mensaje que se desea enviar al usuario.
 * <br>
 * `ori` - El nombre del usuario que envía el mensaje.
 */
async fn mensaje_privado(des: String, msg: String, ori: &String) -> Option<String> {
    match CLIENTES.read().await.get(&des) {
	None => return Some(response_extra("TEXT", "NO_SUCH_USER", &des)),
	Some(sender) => {
	    if let Err(_) = sender.send(text_from(ori, msg)).await {
		return Some(response_extra("TEXT", "NO_SUCH_USER", &des));
	    }
	}
    }
    None
}

/**
 * Crea un nuevo cuarto y mete al usuario que lo creó a sí mismo.
 *
 * # Argumentos
 *
 * `rn` - El nombre del cuarto que se quiere crear.
 * <br>
 * `nom` - El nombre del usuario que quiere crear el cuarto.
 */
async fn crea_cuarto(rn: String, nom: &String) -> String {
    if let Some(_) = CUARTOS.read().await.get(&rn) {
	return response_extra("NEW_ROOM", "ROOM_ALREADY_EXISTS", &rn);
    }
    let mut nuevo_cuarto = Cuarto::new();
    nuevo_cuarto.invita(nom.clone());
    CUARTOS.write().await.insert(rn.clone(), nuevo_cuarto);
    join_room(&rn, nom).await;
    response_extra("NEW_ROOM", "SUCCESS", &rn)
}

/**
 * Invita a una lista de usuarios a unirse a un cuarto.
 *
 * # Argumentos
 * 
 * `us` - El `Vec` que contiene los nombres de las personas a invitar.
 * <br>
 * `rn` - El nombre del cuarto para el que son las invitaciones.
 * <br>
 * `nom` - El nombre del usuario que realiza la invitación.
 * <br>
 * `d` - La dirección IP del usuario que realiza la invitación.
 */
async fn invitaciones(us: Vec<String>, rn: String, nom: &String, d: &SocketAddr)
		      ->  Result<Option<String>, ErrorServidor> {
    match CUARTOS.write().await.get_mut(&rn) {
	None => return Ok(Some(response_extra("INVITE", "NO_SUCH_ROOM", &rn))),
	Some(room) => {
	    if !room.es_miembro(&nom) { return Ok(None); }
	    for user in us {
		if user.chars().count() > 8 {
		    return Err(NombreInvalido{ direccion: *d, nombre: Some(nom.clone()) });
		}
		if &user == nom ||
		   room.es_invitado(&user) ||
		   room.es_miembro(&user) { continue; }
		
		match CLIENTES.read().await.get(&user) {
		    None => {
			return Ok(Some(response_extra("INVITE", "NO_SUCH_USER", &user)));
		    },
		    Some(sender) => {
			if let Err(_) = sender.send(invitation(nom, &rn)).await { continue; }
			room.invita(user);
		    }
		}
	    }
	},
    }
    Ok(None)
}

/**
 * Obtiene una lista de los usuarios del cuarto solicitado.
 *
 * # Argumentos
 *
 * `rn` - El nombre del cuarto del que se requiere la lista de miembros.
 * <br>
 * `nom` - El nombre de la persona que está pidiendo la lista.
 */
async fn usuarios_cuarto(rn: String, nom: &String) -> String {
    match CUARTOS.read().await.get(&rn) {
	None => return response_extra("ROOM_USERS", "NO_SUCH_ROOM", &rn),
	Some(room) => {
	    if !room.es_miembro(nom) {
		return response_extra("ROOM_USERS", "NOT_JOINED", &rn);
	    }
	    let mut mapa_miembros = HashMap::new();
	    for user in room.miembros() {
		mapa_miembros.insert(user.clone(),
				     USUARIOS.read().await.get(user).unwrap().clone());
	    }
	    return room_user_list(&rn, mapa_miembros);
	},
    }
}

/**
 * Envía un mensaje a todos los integrantes de un cuarto.
 *
 * # Argumentos
 *
 * `rn` - El nombre del cuarto al que se quiere enviar el mensaje.
 * <br>
 * `msg` - El mensaje que se quiere enviar al cuarto.
 * <br>
 * `nom` - El nombre del usuario que quiere mandar el mensaje.
 */
async fn mensaje_cuarto(rn: String, msg: String, nom: &String) -> Option<String> {
    match CUARTOS.write().await.get_mut(&rn) {
	None => return Some(response_extra("ROOM_TEXT", "NO_SUCH_ROOM", &rn)),
	Some(room) => {
	    if !room.es_miembro(nom) {
		return Some(response_extra("ROOM_TEXT", "NOT_JOINED", &rn));
	    }
	},
    }
    todos_menos_cuarto(room_text_from(&rn, nom, msg), &rn, nom).await;
    None
}

/**
 * Permite a un usuario abandonar un cuarto.
 *
 * # Argumentos
 *
 * `rn` - El nombre del cuarto que el usuario desea abandonar.
 * <br>
 * `nom` - El nombre del usuario que desea abandonar el cuarto.
 */
async fn abandonar_cuarto(rn: String, nom: &String) -> Option<String> {
    let mut cuarto_vacio: bool = false;
    match CUARTOS.write().await.get_mut(&rn) {
	None => {
	    return Some(response_extra("LEAVE_ROOM", "NO_SUCH_ROOM", &rn));
	},
	Some(room) => {
	    if !room.es_miembro(nom) {
		return Some(response_extra("LEAVE_ROOM", "NOT_JOINED", &rn));
	    }
	    room.salio(nom);
	    if room.miembros().is_empty() {
		cuarto_vacio = true;
	    }
	},
    }
    if cuarto_vacio {
	CUARTOS.write().await.remove(&rn);
	return None;
    }
    todos_menos_cuarto(left_room(&rn, nom) , &rn, nom).await;
    None
}

/**
 * Desconecta al cliente del servidor y le avisa a todos los usuarios de esto.
 *
 * # Argumentos
 *
 * `nom` - Un String que contiene el nombre del cliente a desconectar.
 */
async fn desconecta(nom: &String) {
    CLIENTES.write().await.remove(nom);
    USUARIOS.write().await.remove(nom);
    
    todos_menos(disconnected(nom), nom).await;

    let mut cuartos = CUARTOS.write().await;
    let mut miembrode: Vec<String> = Vec::new();
    for (rn, cuarto) in cuartos.iter_mut() {
	if cuarto.es_miembro(nom) {
	    cuarto.salio(nom);
	    miembrode.push(rn.clone());
	}
    }
    drop(cuartos);
    for rn in miembrode {
	todos_menos_cuarto(left_room(&rn, nom), &rn, nom).await;
    }
}

/**
 * Envia a todos excepto al cliente indicado un mensaje.
 *
 * # Argumentos
 *
 * `msg` - El mensaje que se desea mandar a todos los clientes.
 * <br>
 * `nom` - El nombre del cliente al que no se le desea mandar.
 */
async fn todos_menos(msg: String, nom: &String) {
    let clientes = CLIENTES.read().await;
    for (nombre, sender) in clientes.iter() {
	if nombre == nom { continue; }
	if let Err(_) = sender.send(msg.clone()).await {
	    continue;
	}
    }
}

/**
 * Envia a todos los miembros de un cuarto excepto al cliente indicado un mensaje.
 *
 * # Argumentos
 *
 * `msg` - El mensaje que se desea mandar a todos los clientes.
 * <br>
 * `rn` - El nombre del cuarto del que obtener los miembros.
 * <br>
 * `nom` - El nombre del cliente al que no se le desea mandar.
 */
async fn todos_menos_cuarto(msg: String, rn: &String, nom: &String) {
    let clientes = CLIENTES.read().await;
    let cuartos = CUARTOS.read().await;
    let roomito = cuartos.get(rn).unwrap();
    for user in roomito.miembros().iter() {
	if user == nom { continue; }
	if let Err(_) = clientes.get(user).unwrap().send(msg.clone()).await {
	    continue;
	}
    }
}

pub mod bitacora;
pub mod util;
