use std::net::SocketAddr;
use std::option::Option;
use colored::Colorize;
use std::io::Error;

/**
 * Registra en la bitácora un mensaje recibido.
 *
 * # Argumentos
 *
 * `mensaje` - El mensaje recibido.
 * <br>
 * `direccion` - La dirección IP desde la que se recibió el mensaje.
 * <br>
 * `nombre` - Posiblemente, el nombre con el que se identificó el cliente en la
 *            dirección IP.
 */
pub fn recibido(mensaje: &String, direccion: &SocketAddr, nombre: Option<String>) {
    print!("** MENSAJE {} DE {}", "RECIBIDO".green(), direccion.to_string().bold());
    if let Some (n) = nombre { print!(" ({})", n.magenta()); }
    println!(": {}", mensaje);
}

/**
 * Registra en la bitácora un mensaje enviado.
 *
 * # Argumentos
 *
 * `mensaje` - El mensaje enviado.
 * <br>
 * `direccion` - La dirección IP a la que se envió el mensaje.
 * <br>
 * `nombre` - Posiblemente, el nombre con el que se identificó el cliente en la
 *            dirección IP.
 */
pub fn enviado(mensaje: &String, direccion: &SocketAddr, nombre: Option<String>) {
    print!("** MENSAJE {} A {}", "ENVIADO".green(), direccion.to_string().bold());
    if let Some (n) = nombre { print!(" ({})", n.magenta()); }
    println!(": {}", mensaje);
}

/**
 * Registra en la bitácora la ocurrencia de un error.
 *
 * # Argumentos
 *
 * `error` - El error ocurrido.
 */
pub fn error(e: ErrorServidor) {
    eprint!("** {}: \n\t", "ERROR".red());
    match e {
	ErrorServidor::Creacion { error: e, direccion: d } =>
	    eprintln!("No se pudo crear el servidor en {}: {}",
		      d.to_string().bold(), e),
	ErrorServidor::Aceptacion { error: e } =>
	    eprintln!("Ocurrió un error al intentar aceptar una conexión: {}",
		      e),
	ErrorServidor::Reidentify { direccion: d, nombre: n } =>
	    eprintln!("El cliente en {}, identificado como {} intentó volver a identificarse.",
		      d.to_string().bold(), n.magenta()),
	ErrorServidor::Recepcion { error: e, direccion: d, nombre: Some(n) } =>
	    eprintln!("Ocurrió un error al recibir un mensaje de {} ({}): {}",
		      d.to_string().bold(), n.magenta(), e),
	ErrorServidor::Recepcion { error: e, direccion: d, nombre: None } =>
	    eprintln!("Ocurrió un error al recibir un mensaje de {}: {}",
		      d.to_string().bold(), e),
	ErrorServidor::Envio { error: e, direccion: d, nombre: Some(n) } =>
	    eprintln!("Ocurrió un error al enviar un mensaje a {} ({}): {}",
		      d.to_string().bold(), n.magenta(), e),
	ErrorServidor::Envio { error: e, direccion: d, nombre: None } =>
	    eprintln!("Ocurrió un error al enviar un mensaje a {}: {}",
		      d.to_string().bold(), e),
	ErrorServidor::NombreInvalido { direccion: d, nombre: Some(n) } =>
	    eprintln!("El nombre solicitado por {} ({}) es muy largo.",
		      d.to_string().bold(), n.magenta()),
	ErrorServidor::NombreInvalido { direccion: d, nombre: None } =>
	    eprintln!("El nombre solicitado por {} es muy largo.",
		      d),
	ErrorServidor::Invalido { direccion: d, nombre: Some(n) } =>
	    eprintln!("El mensaje enviado por {} ({}) fue inválido.",
		      d.to_string().bold(), n.magenta()),
	ErrorServidor::Invalido { direccion: d, nombre: None } =>
	    eprintln!("El mensaje enviado por {} fue inválido.",
		      d),
	_ => {},
    }
    eprint!("\n");
}

/**
 * Enumeración para los errores que pueden ocurrir
 */
pub enum ErrorServidor{
    Creacion { error: Error, direccion: String },
    Aceptacion { error: Error },
    Recepcion { error: Error, direccion: SocketAddr, nombre: Option<String> },
    Envio {error: Error, direccion: SocketAddr, nombre: Option<String> },
    Reidentify { direccion: SocketAddr, nombre: String },
    Invalido { direccion: SocketAddr, nombre: Option<String> },
    NombreInvalido { direccion: SocketAddr, nombre: Option<String> },
    Desconectado,
}
