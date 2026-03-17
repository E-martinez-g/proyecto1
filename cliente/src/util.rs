use std::io::Error;
use std::collections::HashMap;
use std::hash::{Hash, Hasher, DefaultHasher};
use colored::*;
use protocolo::{ServerType, ServerType::*, EstadoUsuario, EstadoUsuario::*};
use crate::util::ErrorCliente::*;

/**
 * Enumeración para los errores que pueden ocurrir en el cliente.
 */
pub enum ErrorCliente {
    EntradaEstandar { error: Option<Error> },
    Conexion{ error: Error, direccion: String },
    NombreVacio,
    NombreOcupado,
    NombreCuartoVacio,
    NombreCuartoOcupado,
    Envio{ error: Error },
    Recepcion{ error: Error },
    Invalido,
}

/**
 * Imprime de forma legible el error ocurrido.
 *
 * # Argumentos
 *
 * `err` - El error que se quiere imprimir.
 */
pub fn error(err: ErrorCliente) {
    print!("[Sys] ");
    match err {
	EntradaEstandar{ error: None } =>
	    println!("Se cerró la entrada estándar."),
	EntradaEstandar{ error: Some(e) } =>
	    println!("Ocurrió un error en la entrada estándar. {}", e),
	Conexion{ error: e, direccion: d } =>
	    println!("No se pudo conectar a un servidor en {}. {}",
		     d.to_string().bold(), e),
	NombreVacio =>
	    println!("No se puede utilizar un nombre vacío."),
	NombreOcupado =>
	    println!("El nombre elegido ya está en uso."),
	NombreCuartoVacio =>
	    println!("Los cuartos no pueden tener un nombre vacío."),
	NombreCuartoOcupado =>
	    println!("Ya existe un cuarto con ese nombre."),
	Envio{ error: e } =>
	    println!("Ocurrió un error al enviar datos al servidor. {}",
		     e),
	Recepcion{ error: e } =>
	    println!("Ocurrió un error el recibir datos del servidor. {}",
		     e),
	Invalido =>
	    println!("El servidor envió un mensaje inválido."),
    }
}

/**
 * Imprime un mensaje del servidor.
 *
 * # Argumentos
 *
 * `st` - El tipo del mensaje del servidor.
 */
pub fn sistema(st: ServerType) {
    let msg = match st {
	r @ Response{..} => respuesta(r),
	
	NewUser{ username: u } =>
	    format!("* {} se unió a la conversación. *",
		    colorea(u)).dimmed().to_string(),
	
	NewStatus{ username: u, status:  Active } =>
	    format!("* {} cambió su estado a {} *",
		    colorea(u), "ACTIVO".green()).dimmed().to_string(),
	
	NewStatus{ username: u, status: Away } =>
	    format!("* {} cambió su estado a {} *",
		    colorea(u), "AUSENTE".yellow()).dimmed().to_string(),
	
	NewStatus{ username: u, status: Busy } =>
	    format!("* {} cambió su estado a {} *",
		    colorea(u), "OCUPADO".red()).dimmed().to_string(),

	UserList{ users: us } => lista(us),

	TextFrom { username: u, text: t } =>
	    format!("[{} (MD)] {}", colorea(u), t),
	
	PublicTextFrom{ username: u, text: t } =>
	    format!("[{}] {}", colorea(u), t),
	
	Invitation{ username: u, roomname: r } =>
	    format!("* {} te invitó al cuarto {}. *",
		    colorea(u), colorea(r)).dimmed().to_string(),
	
	JoinedRoom{ roomname: r, username: u } =>
	    format!("* {} se unió al cuarto {}. *",
		    colorea(u), colorea(r)).dimmed().to_string(),
	
	RoomUserList{ roomname: r, users: us } => lista_cuarto(r, us),

	RoomTextFrom{ roomname: r, username: u, text: t } =>
	    format!("[{} @ {}] {}", colorea(u), colorea(r), t),

	LeftRoom{ roomname: r, username: u } =>
	    format!("* {} abandonó el cuarto {}. *",
		    colorea(u), colorea(r)).dimmed().to_string(),

	Disconnected{ username: u } =>
	    format!("* {} abandonó la conversación. *",
		    colorea(u)).dimmed().to_string(),
    };
    println!("{}", msg);
}

/**
 * Colorea la cadena que se le entrega dispersándola.
 *
 * # Argumentos
 *
 * `nom` - El String que se desea colorear, se llama asíporque se espera sea el
 *         nombre de un cuarto o un usuario.
 */
fn colorea(nom: String) -> String {
    let colores = [(220, 80, 80), (220, 140, 60), (200, 200, 60),
		   (100, 200, 100), (60, 180, 180), (80, 140, 220),
		   (140, 80, 220), (220, 80, 180), (80, 220, 160),
		   (220, 120, 120), (120, 180, 240), (200, 120, 220),
		   (240, 160, 80), (80, 200, 220), (160, 220, 80),
		   (220, 160, 100), (180, 100, 60), (100, 160, 140),];

    let mut dispersor = DefaultHasher::new();
    nom.hash(&mut dispersor);
    let dis = dispersor.finish() as usize;
    
    let (r, g, b) = colores[dis % colores.len()];
    nom.color(colored::Color::TrueColor{ r, g, b }).to_string()
}

/**
 * Regresa la representación en cadena de la lista de usuarios
 *
 * # Argumentos
 *
 * `us` - El `HashMap` con la lista de usuarios.
 */
fn lista(us: HashMap<String, EstadoUsuario>) -> String {
    let mut usuarios = String::new();
    for (nombre, estado) in us.iter() {
	usuarios += &format!("• {}: ", nombre);
	match estado {
	    Active => usuarios += &"ACTIVO".green().to_string(),
	    Away => usuarios += &"AUSENTE".yellow().to_string(),
	    Busy => usuarios += &"OCUPADO".red().to_string(),
	}
	usuarios += "\n";
    }
    usuarios
}


/**
 * Regresa la representación en cadena de la lista de usuarios
 *
 * # Argumentos
 *
 * `r` - El nombre del cuarto del que es la lista.
 * <br>
 * `us` - El `HashMap` con la lista de usuarios.
 */
fn lista_cuarto(r: String, us: HashMap<String, EstadoUsuario>) -> String {
    format!("Miembros de {}:\n", r) + &lista(us)
}

/**
 * Se encarga de manejar las respuestas que envía el servidor.
 *
 * # Argumentos
 *
 * `r` - La respuesta enviada por el servidor.
 */
pub fn respuesta(r: ServerType) -> String {
    format!("algo {}", "bien")
}
