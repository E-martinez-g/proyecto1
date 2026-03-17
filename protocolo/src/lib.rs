use std::env::args;
use std::net::{IpAddr, Ipv4Addr};

use serde::{Serialize, Deserialize};

use std::option::Option;
use std::collections::HashMap;

/**
 * Provee la dirección del servidor a partir de los argumentos del programa.
 * El puerto por omisión es 42069.
 * La dirección IP por omisión es 127.0.0.1.
 */
pub fn server_address() -> String {
    let ip = args().nth(1).unwrap_or_default()
	     .parse::<IpAddr>().unwrap_or(IpAddr::V4(Ipv4Addr::new(0,0,0,0)));
    
    let mut port = args().nth(2).unwrap_or_default()
	           .parse::<u16>().unwrap_or_default();
    if port < 1024 { port = 42069; }
    
    format!("{}:{}", ip, port)
}

/**
 * Enumeración para el estado de los usuarios.
 */
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum EstadoUsuario {
    Active,
    Away,
    Busy,
}

/**
 * Enumeración para los tipos de operaciones a las que se les puede responder.
 */
#[derive(Deserialize, PartialEq)]
#[serde(tag = "operation", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Operacion {
    Identify,
    Text,
    NewRoom,
    Invite,
    JoinRoom,
    RoomUsers,
    RoomText,
    LeaveRoom,
    Invalid,
}

/**
 * Enumeración para los resultados posibles de las operaciones a las que se les
 * puede responder.
 */
#[derive(Deserialize, PartialEq)]
#[serde(tag = "result", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Resultado {
    Success,
    UserAlreadyExists,
    NoSuchUser,
    RoomAlreadyExists,
    NoSuchRoom,
    NotInvited,
    NotJoined,
    NotIdentified,
    Invalid,
}

/**
 * Enumeración para el "type" de los mensajes que
 * envía el cliente.
 */
#[derive(Deserialize)]
#[serde(tag = "type", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ClientType {
    
    Identify { username: String },
    
    Status { status: EstadoUsuario },
    
    Users,
    
    Text { username: String, text: String },
    
    PublicText { text: String },
    
    NewRoom { roomname: String },
    
    Invite { roomname: String, usernames: Vec<String> },
    
    JoinRoom { roomname: String },
    
    RoomUsers { roomname: String },
    
    RoomText { roomname: String, text: String },
    
    LeaveRoom { roomname: String },
    
    Disconnect,
}

/**
 * Enumeración para el "type" de los mensajes que
 * envía el servidor.
 */
#[derive(Deserialize)]
#[serde(tag = "type", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ServerType {
    
    Response { operation: Operacion, result: Resultado,
	       extra: Option<String> },
    
    NewUser { username: String },
    
    NewStatus { username: String, status: EstadoUsuario },
    
    UserList { users: HashMap<String, EstadoUsuario> },
    
    TextFrom { username: String, text: String },
    
    PublicTextFrom { username: String, text: String },

    Invitation{ username: String, roomname: String },
    
    JoinedRoom { roomname: String, username: String },
    
    RoomUserList { roomname: String,
		   users: HashMap<String, EstadoUsuario> },
    
    RoomTextFrom { roomname: String, username: String,
		   text: String },
    
    LeftRoom { roomname: String, username: String },
    
    Disconnected { username: String },
}

/**
 * Obtiene la instancia de `ClientType` que
 * corresponde a la línea recibida.
 *
 * # Argumentos
 *
 * `ser` - Un String que contiene el JSON
 *         seriado a buscar.
 */
pub fn parsea_mensaje_cliente(ser: String)
			      -> Result<ClientType, serde_json::Error> {
    serde_json::from_str(&ser)
}

/**
 * Obtiene la instancia de `ServerType` que
 * corresponde a la línea recibida.
 *
 * # Argumentos
 *
 * `ser` - Un String que contiene el JSON
 *         seriado a buscar.
 */
pub fn parsea_mensaje_servidor(ser: String)
			       -> Result<ServerType, serde_json::Error> {
    serde_json::from_str(&ser)
}

/**
 * Módulo para funciones que crean de forma 
 * sencilla los mensajes enviados por los clientes
 * en formato JSON.
 */
pub mod mensajes_cliente;

/**
 * Módulo para funciones que crean de forma
 * sencilla los mensajes enviados por el servidor
 * servidor en formato JSON.
 */
pub mod mensajes_servidor;
