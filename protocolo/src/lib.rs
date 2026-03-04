/**
 * Enumeración para el "type" de los mensajes que
 * envía el cliente.
 */
pub enum ClientType {
    Identify,
    Status,
    Users,
    Text,
    PublicText,
    NewRoom,
    Invite,
    JoinRoom,
    RoomUsers,
    RoomText,
    LeaveRoom,
    Disconnect,
}

/**
 * Enumeración para el "type" de los mensajes que
 * envía el servidor.
 */
pub enum ServerType {
    Response,
    ResponseExtra,
    NewUser,
    NewStatus,
    UserList,
    TextFrom,
    PublicTextFrom,
    JoinedRoom,
    RoomUserList,
    RoomTextFrom,
    LeftRoom,
    Disconnected,
}

/**
 * Módulo para funciones que crean de forma 
 * sencilla los mensajes enviados por los clientes
 * en formato JSON.
 */
pub mod mensajes_cliente {

    use serde_json::json;

    /**
     * Crea un mensaje del tipo "IDENTIFY".
     *
     * # Argumentos
     *
     * `usr` - Un String con el nombre de usuario
     *         que desea usar el cliente.
     */
    pub fn identify(s: &String) -> String {
	json!({
	    "type": "IDENTIFY",
	    "username": s
	}).to_string()
    }

    /**
     * Crea un mensaje del tipo "STATUS".
     *
     * # Argumentos
     *
     * `sta` - Una instancia de cliente::Estado que
     *         representa el nuevo estado que el
     *         cliente desea utilizar.
     */
    pub fn status(e: &String) -> String {
	json!({
	    "type": "STATUS",
	    "status": e
	}).to_string()
    }

    /**
     * Crea un mensaje del tipo "USERS".
     */
    pub fn users() -> String {
	json!({
	    "type": "USERS"
	}).to_string()
    }
    
    /**
     * Crea un mensaje del tipo "TEXT".
     *
     * # Argumentos
     *
     * `usr` - Un String que contiene el usuario
     *         al que se desea mandar el mensaje.
     * `msg` - Un String que contiene el mensaje.
     */
    pub fn text(usr: String, msg: String) -> String {
	json!({
	    "type": "TEXT",
	    "username": usr,
	    "text": msg
	}).to_string()
    }
    
    /**
     * Crea un mensaje del tipo "PUBLIC_TEXT".
     *
     * # Argumentos
     *
     * `msg` - Un String que contiene el mensaje.
     */
    pub fn public_text(msg: String) -> String {
	json!({
	    "type": "PUBLIC_TEXT",
	    "text": msg 
	}).to_string()
    }
    
    /**
     * Crea un mensaje del tipo "NEW_ROOM".
     *
     * # Argumentos
     *
     * `room` - Un String que contiene el nombre
     *          del nuevo cuarto a crear.
     */
    pub fn new_room(room: &String) -> String {
	json!({
	    "type": "NEW_ROOM",
	    "roomname": room
	}).to_string()
    }
    
    /**
     * Crea un mensaje del tipo "INVITE".
     *
     * # Argumentos
     *
     * `room` - Un String que contiene el nombre
     *          del cuarto al que se invitará.
     * `usrs` - Un Vec<String> con los usuarios a
     *          invitar.
     */
    pub fn invite(room: &String, usrs: Vec<String>) -> String {
	json!({
	    "type": "INVITE",
	    "roomname": room,
	    "usernames": usrs
	}).to_string()
    }
    
    /**
     * Crea un mensaje del tipo "JOIN_ROOM".
     *
     * # Argumentos
     *
     * `room` - Un String que contiene el nombre
     *          del cuarto al que se busca unirse.
     */
    pub fn join_room(room: &String) -> String {
	json!({
	    "type": "JOIN_ROOM",
	    "roomname": room
	}).to_string()
    }
    
    /**
     * Crea un mensaje del tipo "ROOM_USERS".
     *
     * # Argumentos
     *
     * `room` - Un String que contiene el nombre
     *          del cuarto del que se quiere
     *          obtener el nombre de sus miembros.
     */
    pub fn room_users(room: &String) -> String {
	json!({
	    "type": "ROOM_USERS",
	    "roomname": room
	}).to_string()
    }
    
    /**
     * Crea un mensaje del tipo "ROOM_TEXT".
     *
     * # Argumentos
     *
     * `room` - Un String que contiene el nombre
     *          del cuarto al que se quiere enviar
     *          el mensaje.
     * `msg` - Un String que contiene el mensaje
     *         que se desea enviar.
     */
    pub fn room_text(room: &String, msg: String) -> String {
	json!({
	    "type": "ROOM_TEXT",
	    "roomname": room,
	    "text": msg
	}).to_string()
    }
    
    /**
     * Crea un mensaje del tipo "LEAVE_ROOM".
     *
     * # Argumentos
     *
     * `room` - Un String que contiene el nombre
     *          del cuarto que se desea abandonar.
     */
    pub fn leave_room(room: &String) -> String {
	json!({
	    "type": "LEAVE_ROOM",
	    "roomname": room
	}).to_string()
    }
    
    /**
     * Crea un mensaje del tipo "DISCONNECT".
     */
    pub fn disconnect() -> String {
	json!({
	    "type": "DISCONNECT"
	}).to_string()
    }
}
