use std::net::SocketAddr;
use std::option::Option;
use std::io::Error;

/**
 * Registra en la bitácora un mensaje recibido.
 *
 * # Argumentos
 *
 * `mensaje` - El mensaje recibido.
 * `direccion` - La dirección IP desde la que se recibió el mensaje.
 * `nombre` - Posiblemente, el nombre con el que se identificó el cliente en la
 *            dirección IP.
 */
pub fn recibido(mensaje: &String, direccion: &SocketAddr, nombre: Option<&String>) {
    match nombre {
	None => println!("** MENSAJE RECIBIDO DE {}: {}", direccion, mensaje),
	Some(n) => println!("** MENSAJE RECIBIDO DE {} ({}): {}", direccion, n, mensaje),
    }
}

/**
 * Registra en la bitácora un mensaje enviado.
 *
 * # Argumentos
 *
 * `mensaje` - El mensaje enviado.
 * `direccion` - La dirección IP a la que se envió el mensaje.
 * `nombre` - Posiblemente, el nombre con el que se identificó el cliente en la
 *            dirección IP.
 */
pub fn enviado(mensaje: &String, direccion: &SocketAddr, nombre: Option<&String>) {
    match nombre {
	None => println!("** MENSAJE ENVIADO A {}: {}", direccion, mensaje),
	Some(n) => println!("** MENSAJE ENVIADO A {} ({}): {}", direccion, n, mensaje),
    }
}

/**
 * Registra en la bitácora la ocurrencia de un error.
 *
 * # Argumentos
 *
 * `error` - El error ocurrido.
 * `direccion` - Posiblemente, La dirección IP del cliente con el que ocurriò el error.
 * `nombre` - Posiblemente, el nombre con el que se identificó el cliente en la
 *            dirección IP.
 */
pub fn error(e: ErrorServidor) {
    eprint!("** ERROR: \n\t");
    match e {
	ErrorServidor::Creacion { error: e, direccion: d } =>
	    eprintln!("No se pudo crear el servidor en {}: {}", d, e),
	ErrorServidor::Aceptacion { error: e } =>
	    eprintln!("Ocurrió un error al intentar aceptar una conexión: {}", e),
	ErrorServidor::Reidentify { direccion: d, nombre: n } =>
	    eprintln!("El cliente en {}, identificado como {} intentó volver a identificarse.",
		      d, n),
	ErrorServidor::Recepcion { error: e, direccion: d, nombre: Some(n) } =>
	    eprintln!("Ocurrió un error al recibir un mensaje de {} ({}): {}", d, n, e),
	ErrorServidor::Recepcion { error: e, direccion: d, nombre: None } =>
	    eprintln!("Ocurrió un error al recibir un mensaje de {}: {}", d, e),
	ErrorServidor::Envio { error: e, direccion: d, nombre: Some(n) } =>
	    eprintln!("Ocurrió un error al enviar un mensaje a {} ({}): {}", d, n, e),
	ErrorServidor::Envio { error: e, direccion: d, nombre: None } =>
	    eprintln!("Ocurrió un error al enviar un mensaje a {}: {}", d, e),
	ErrorServidor::Invalido { direccion: d, nombre: Some(n) } =>
	    eprintln!("El mensaje enviado por {} ({}) fue inválido.", d, n),
	ErrorServidor::Invalido { direccion: d, nombre: None } =>
	    eprintln!("El mensaje enviado por {} fue inválido.", d),
    }
    eprint!("\n");
}

/**
 * Enumeración para los errores que pueden ocurrir
 */
pub enum ErrorServidor{
    Creacion { error:Error, direccion: String },
    Aceptacion { error: Error },
    Recepcion { error: Error, direccion: SocketAddr, nombre: Option<String> },
    Envio {error: Error, direccion: SocketAddr, nombre: Option<String> },
    Reidentify { direccion: SocketAddr, nombre: String },
    Invalido { direccion: SocketAddr, nombre: Option<String> },
}
