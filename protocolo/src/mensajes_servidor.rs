use serde_json::json;
use std::collections::HashMap;

/**
 * Crea un mensaje del tipo "RESPONSE".
 *
 * # Argumentos
 *
 * `opr` - Un `String` que contiene el nombre
 *         de la operación a la que se le está
 *         dando respuesta.
 * <br>
 * `res` - Un `String` con el resultado de la
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
 * `opr` - Un `String` que contiene el nombre
 *         de la operación a la que se le está
 *         dando respuesta.
 * <br>
 * `res` - Un `String` con el resultado de la
 *         operación.
 * <br>
 * `ext` - Un `String` con el mensaje extra.
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
 * `usr` - Un `String` con el nombre del nuevo
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
 * `usr` - Un `String` que contiene el nombre
 *         del usuario que cambió su estado.
 * <br>
 * `sta` - Una instancia de `cliente::Estado`
 *         que representa el nuevo estado del
 *         usuario.
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
 * `usrs` - Un `HashMap<String, String>` de
 *          llaves que son el nombre de un 
 *          usuario y valores que son su estado
 *          correspondiente.
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
 * `usr` - Un `String` que contiene el nombre
 *         del usuario que envía el mensaje.
 * <br>
 * `msg` - Un `String` que contiene el mensaje.
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
 * `usr` - Un `String` que contiene el nombre
 *         del usuario que envía el mensaje.
 * <br>
 * `msg` - Un `String` que contiene el mensaje.
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
 * `room` - Un `String` con el nombre del
 *          cuarto al que se unió el usuario.
 * <br>
 * `usr` - Un `String` con el nombre del
 *         usuario que se unió.
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
 * `room` - Un `String` con el nombre del
 *          cuarto cuya lista de usuarios se
 *          está enviando.
 * <br>
 * `usrs` - Un `HashMap<String, String>` de
 *          llaves que son el nombre de un 
 *          usuario y valores que son su
 *          estado correspondiente.
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
 * `room` - Un `String` con el nombre del
 *          cuarto al que se envió el mensaje.
 * <br>
 * `usr` - Un `String` con el nombre del
 *         usuario que envió el mensaje.
 * <br>
 * `msg` - Un `String` con el mensaje que fue
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
 * `room` - Un `String` con el nombre del
 *          cuarto que el usuario abandonó.
 * <br>
 * `usr` - Un `String` con el nombre del
 *         usuario que abandonó el cuarto.
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
 * `usr` - Un `String` que contiene el nombre
 *         del usuario que se desconectó.
 */
pub fn disconnected(usr: &String) -> String {
    json!({
	"type": "DISCONNECTED",
	"username": usr
    }).to_string()
}
