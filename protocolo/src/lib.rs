use serde::Serialize;

/**
 * Enumeración para el estado de los usuarios.
 */
#[derive(Serialize)]
pub enum EstadoUsuario {
    ACTIVE,
    AWAY,
    BUSY,
}


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
    pub fn identify(usr: &String) -> String {
	json!({
	    "type": "IDENTIFY",
	    "username": usr
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
    pub fn status(sta: &super::EstadoUsuario) -> String {
	json!({
	    "type": "STATUS",
	    "status": sta
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

pub mod mensajes_servidor {
    
    use serde_json::json;
    use std::collections::HashMap;

    /**
     * Crea un mensaje del tipo "RESPONSE".
     *
     * # Argumentos
     *
     * `opr` - Un String que contiene el nombre de
     *         la operación a la que se le está
     *         dando respuesta.
     * `res` - Un String con el resultado de la
     *         operación.
     */
    pub fn response(opr: String, res: String) -> String {
	json!({
	    "type": "RESPONSE",
	    "operation": opr,
	    "result": res
	}).to_string()
    }

    /**
     * Crea un mensaje del tipo "RESPONSE" con una
     * cláusula extra.
     *
     * # Argumentos
     *
     * `opr` - Un String que contiene el nombre de
     *         la operación a la que se le está
     *         dando respuesta.
     * `res` - Un String con el resultado de la
     *         operación.
     * `ext` - Un String con el mensaje extra.
     */
    pub fn response_extra(opr: String, res: String,
			  ext: String) -> String {
	json!({
	    "type": "RESPONSE",
	    "operation": opr,
	    "result": res,
	    "extra": ext
	}).to_string()
    }

    /**
     * Crea un mensaje del tipo "NEW_USER".
     *
     * # Argumentos
     *
     * `usr` - Un String con el nombre del nuevo
     *         usuario en el servidor
     */
    pub fn new_user(usr: &String) -> String {
	json!({
	    "type": "NEW_USER",
	    "username": usr
	}).to_string()
    }

    /**
     * Crea un mensaje del tipo "NEW_STATUS".
     *
     * # Argumentos
     *
     * `usr` - Un String que contiene el nombre
     *         del usuario que cambió su estado.
     * `sta` - Una instancia de cliente::Estado
     *         que representa el nuevo estado que
     *         el usuario desea utilizar.
     */
    pub fn new_status(usr: &String, sta: &super::EstadoUsuario) -> String {
	json!({
	    "type": "NEW_STATUS",
	    "username": usr,
	    "status": sta
	}).to_string()
    }

    /**
     * Crea un mensaje del tipo "USER_LIST".
     *
     * # Argumentos
     *
     * `usrs` - Un HashMap de llaves que son el
     *          nombre de un usuario y valores que
     *          son su respectivo estado.
     */
    pub fn user_list(usrs: &HashMap<String, String>) -> String {
	json!({
	    "type": "USER_LIST",
	    "users": usrs
	}).to_string()
    }

    /**
     * Crea un mensaje del tipo "TEXT_FROM".
     *
     * # Argumentos
     *
     * `usr` - Un String que contiene el nombre
     *         del usuario que envía el mensaje.
     * `msg` - Un String que contiene el mensaje.
     */
    pub fn text_from(usr: &String, msg: String) -> String {
	json!({
	    "type": "TEXT_FROM",
	    "username": usr,
	    "text": msg
	}).to_string()
    }

    /**
     * Crea un mensaje del tipo "PUBLIC_TEXT_FROM".
     *
     * # Argumentos
     *
     * `usr` - Un String que contiene el nombre
     *         del usuario que envía el mensaje.
     * `msg` - Un String que contiene el mensaje.
     */
    pub fn public_text_from(usr: &String, msg: String) -> String {
	json!({
	    "type": "PUBLIC_TEXT_FROM",
	    "username": usr,
	    "text": msg
	}).to_string()
    }

    /**
     * Crea un mensaje del tipo "JOINED_ROOM".
     *
     * # Argumentos
     *
     * `room` - Un String con el nombre del cuarto
     *          al que se unió el nuevo usuario.
     * `usr` - Un String con el nombre del usuario
     *         que se unió.
     */
    pub fn joined_room(room: &String, usr: &String) -> String {
	json!({
	    "type": "JOINED_ROOM",
	    "roomname": room,
	    "username": usr
	}).to_string()
    }

    /**
     * Crea un mensaje del tipo "ROOM_USER_LIST".
     *
     * # Argumentos
     *
     * `room` - Un String con el nombre del cuarto
     *          cuya lista de usuarios se está
     *          enviando.
     * `usrs` - Un HashMap de llaves que son el
     *          nombre de un miembro y valores que
     *          son su respectivo estado.
     */
    pub fn room_user_list(room: &String,
			  usrs: &HashMap<String, String>) -> String {
	json!({
	    "type": "ROOM_USER_LIST",
	    "roomname": room,
	    "users": usrs
	}).to_string()
    }

    /**
     * Crea un mensaje del tipo "ROOM_TEXT_FROM"
     *
     * # Argumentos
     *
     * `room` - Un String con el nombre del cuarto
     *          al que se envió el mensaje.
     * `usr` - Un String con el nombre del usuario
     *         que envió el mensaje.
     * `msg` - Un String con el mensaje que fue
     *         enviado.
     */
    pub fn room_text_from(room: &String, usr: &String,
			  msg: String) -> String {
	json!({
	    "type": "ROOM_TEXT_FROM",
	    "roomname": room,
	    "username": usr,
	    "text": msg
	}).to_string()
    }

    /**
     * Crea un mensaje del tipo "LEFT_ROOM"
     *
     * # Argumentos
     *
     * `room` - Un String con el nombre del cuarto
     *          que el usuario abandonó.
     * `usr` - Un String con el nombre del usuario
     *         que abandonó el cuarto.
     */
    pub fn left_room(room: &String, usr: &String) -> String {
	json!({
	    "type": "LEFT_ROOM",
	    "roomname": room,
	    "username": usr
	}).to_string()
    }

    /**
     * Crea un mensaje del tipo "DISCONNECTED".
     *
     * # Argumentos
     *
     * `usr` - Un String que contiene el nombre
     *         del usuario que se desconectó.
     */
    pub fn disconnected(usr: &String) -> String {
	json!({
	    "type": "DISCONNECTED",
	    "username": usr
	}).to_string()
    }
}
