use serde_json::json;

/**
 * Crea un mensaje del tipo "IDENTIFY".
 *
 * # Argumentos
 *
 * `usr` - Un `String` con el nombre de usuario
 *         que desea usar el cliente.
 */
pub fn identify(usr: &String) -> String {
    json!({
	"type": "IDENTIFY",
	"username": usr
    }).to_string() + "\n"
}

/**
 * Crea un mensaje del tipo "STATUS".
 *
 * # Argumentos
 *
 * `sta` - Una instancia de `cliente::Estado`
 *         que representa el nuevo estado que
 *         el cliente desea utilizar.
 */
pub fn status(sta: &super::EstadoUsuario) -> String {
    json!({
	"type": "STATUS",
	"status": sta
    }).to_string() + "\n"
}

/**
 * Crea un mensaje del tipo "USERS".
 */
pub fn users() -> String {
    json!({
	"type": "USERS"
    }).to_string() + "\n"
}

/**
 * Crea un mensaje del tipo "TEXT".
 *
 * # Argumentos
 *
 * `usr` - Un `String` que contiene el usuario
 *         al que se desea mandar el mensaje.
 * <br>
 * `msg` - Un `String` que contiene el mensaje.
 */
pub fn text(usr: String, msg: String) -> String {
    json!({
	"type": "TEXT",
	"username": usr,
	"text": msg
    }).to_string() + "\n"
}

/**
 * Crea un mensaje del tipo "PUBLIC_TEXT".
 *
 * # Argumentos
 *
 * `msg` - Un `String` que contiene el mensaje.
 */
pub fn public_text(msg: String) -> String {
    json!({
	"type": "PUBLIC_TEXT",
	"text": msg 
    }).to_string() + "\n"
}

/**
 * Crea un mensaje del tipo "NEW_ROOM".
 *
 * # Argumentos
 *
 * `room` - Un `String` que contiene el nombre
 *          del nuevo cuarto a crear.
 */
pub fn new_room(room: &String) -> String {
    json!({
	"type": "NEW_ROOM",
	"roomname": room
    }).to_string() + "\n"
}

/**
 * Crea un mensaje del tipo "INVITE".
 *
 * # Argumentos
 *
 * `room` - Un `String` que contiene el nombre
 *          del cuarto al que se invitará.
 * <br>
 * `usrs` - Un `Vec<String>` con los usuarios
 *          a invitar.
 */
pub fn invite(room: &String, usrs: Vec<String>) -> String {
    json!({
	"type": "INVITE",
	"roomname": room,
	"usernames": usrs
    }).to_string() + "\n"
}

/**
 * Crea un mensaje del tipo "JOIN_ROOM".
 *
 * # Argumentos
 *
 * `room` - Un `String` que contiene el nombre
 *          del cuarto al que se busca unirse.
 */
pub fn join_room(room: &String) -> String {
    json!({
	"type": "JOIN_ROOM",
	"roomname": room
    }).to_string() + "\n"
}

/**
 * Crea un mensaje del tipo "ROOM_USERS".
 *
 * # Argumentos
 *
 * `room` - Un `String` que contiene el nombre
 *          del cuarto del que se quiere
 *          obtener el nombre de sus miembros.
 */
pub fn room_users(room: &String) -> String {
    json!({
	"type": "ROOM_USERS",
	"roomname": room
    }).to_string() + "\n"
}

/**
 * Crea un mensaje del tipo "ROOM_TEXT".
 *
 * # Argumentos
 *
 * `room` - Un `String` que contiene el nombre
 *          del cuarto al que se quiere enviar
 *          el mensaje.
 * <br>
 * `msg` - Un `String` que contiene el mensaje
 *         que se desea enviar.
 */
pub fn room_text(room: &String, msg: String) -> String {
    json!({
	"type": "ROOM_TEXT",
	"roomname": room,
	"text": msg
    }).to_string() + "\n"
}

/**
 * Crea un mensaje del tipo "LEAVE_ROOM".
 *
 * # Argumentos
 *
 * `room` - Un `String` que contiene el nombre
 *          del cuarto que se desea abandonar.
 */
pub fn leave_room(room: &String) -> String {
    json!({
	"type": "LEAVE_ROOM",
	"roomname": room
    }).to_string() + "\n"
}

/**
 * Crea un mensaje del tipo "DISCONNECT".
 */
pub fn disconnect() -> String {
    json!({
	"type": "DISCONNECT"
    }).to_string() + "\n"
}
