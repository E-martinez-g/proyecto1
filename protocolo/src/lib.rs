use serde::{Serialize, Deserialize};

use std::option::Option;
use std::collections::HashMap;

/**
 * Enumeración para el estado de los usuarios.
 */
#[derive(Serialize, Deserialize)]
pub enum EstadoUsuario {
    ACTIVE,
    AWAY,
    BUSY,
}

/**
 * Enumeración para el "type" de los mensajes que
 * envía el cliente.
 */
#[derive(Deserialize)]
#[serde(tag = "type")]
pub enum ClientType {
    
    #[serde(rename = "IDENTIFY")]
    Identify { username: String },
    
    #[serde(rename = "STATUS")]
    Status { status: EstadoUsuario },
    
    #[serde(rename = "USERS")]
    Users,
    
    #[serde(rename = "TEXT")]
    Text { username: String, text: String },
    
    #[serde(rename = "PUBLIC_TEXT")]
    PublicText { text: String },
    
    #[serde(rename = "NEW_ROOM")]
    NewRoom { roomname: String },
    
    #[serde(rename = "INVITE")]
    Invite { roomname: String, usernames: Vec<String> },
    
    #[serde(rename = "JOIN_ROOM")]
    JoinRoom { roomname: String },
    
    #[serde(rename = "ROOM_USERS")]
    RoomUsers { roomname: String },
    
    #[serde(rename = "ROOM_TEXT")]
    RoomText { roomname: String, text: String },
    
    #[serde(rename = "LEAVE_ROOM")]
    LeaveRoom { roomname: String },
    
    #[serde(rename = "DISCONNECT")]
    Disconnect,
}

/**
 * Enumeración para el "type" de los mensajes que
 * envía el servidor.
 */
#[derive(Deserialize)]
#[serde(tag = "type")]
pub enum ServerType {
    
    #[serde(rename = "RESPONSE")]
    Response { operation: String, result: String,
	       extra: Option<String> },
    
    #[serde(rename = "NEW_USER")]
    NewUser { username: String },
    
    #[serde(rename = "NEW_STATUS")]
    NewStatus { username: String, status: EstadoUsuario },
    
    #[serde(rename = "USER_LIST")]
    UserList { users: HashMap<String, String> },
    
    #[serde(rename = "TEXT_FROM")]
    TextFrom { username: String, text: String },
    
    #[serde(rename = "PUBLIC_TEXT_FROM")]
    PublicTextFrom { username: String, text: String },
    
    #[serde(rename = "JOINED_ROOM")]
    JoinedRoom { roomname: String, username: String },
    
    #[serde(rename = "ROOM_USER_LIST")]
    RoomUserList { roomname: String,
		   users: HashMap<String, String> },
    
    #[serde(rename = "ROOM_TEXT_FROM")]
    RoomTextFrom { roomname: String, username: String,
		   text: String },
    
    #[serde(rename = "LEFT_ROOM")]
    LeftRoom { roomname: String, username: String },
    
    #[serde(rename = "DISCONNECTED")]
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
