use protocolo::{ServerType, ServerType::*, EstadoUsuario, EstadoUsuario::*, Operacion::*, Resultado::*, mensajes_cliente::*};
use std::io::Error;
use std::collections::HashMap;
use std::hash::{Hash, Hasher, DefaultHasher};
use colored::*;
use crate::util::ErrorCliente::*;

/**
 * Enumeración para los errores que pueden ocurrir en el cliente.
  */
pub enum ErrorCliente {
    NombreVacio,
    NombreMuyLargo,
    NombreCuartoVacio,
    NombreCuartoMuyLargo,
    EstadoInvalido,
    UsuarioFaltante,
    CuartoFaltante, 
    MensajeFaltante,
    Invalido,
    Envio{ error: Error },
    Recepcion{ error: Error },
    Conexion{ error: Error, direccion: String },
    EntradaEstandar { error: Option<Error> },
}

/**
 * Da una cadena que explica de forma legible el error ocurrido.
 *
 * # Argumentos
 *
 * `err` - El error que se quiere imprimir.
 */
pub fn error(err: ErrorCliente) -> String {
    let mut s = String::from("[Sys] ");
    match err {
	NombreVacio =>
	    s = "No se puede utilizar un nombre vacío.".dimmed().to_string(),
	NombreMuyLargo =>
	    s = "El nombre elegido es muy largo. (MAX 8 CHARS)".dimmed().to_string(),
	NombreCuartoVacio =>
	    s = "Los cuartos no pueden tener un nombre vacío.".dimmed().to_string(),
	NombreCuartoMuyLargo =>
	    s = "El nombre elegido es muy largo. (MAX 16 CHARS)".dimmed().to_string(),
	EstadoInvalido =>
	    s = "El valor ingresado no corresponde a un estado válido.".dimmed().to_string(),
	UsuarioFaltante =>
	    s = "No se ingresó el nombre de un usuario.".dimmed().to_string(),
	CuartoFaltante =>
	    s = "No se ingresó el nombre de un cuarto.".dimmed().to_string(),
	MensajeFaltante =>
	    s = "No se ingresó un mensaje.".dimmed().to_string(),
	Invalido =>
	    s += "El servidor envió un mensaje inválido.",
	Envio{ error: e } =>
	    s += &format!("Ocurrió un error al enviar datos al servidor. {}", e),
	Recepcion{ error: e } =>
	    s += &format!("Ocurrió un error el recibir datos del servidor. {}", e),
	Conexion{ error: e, direccion: d } =>
	    s += &format!("No se pudo conectar a un servidor en {}. {}",
			  d.to_string().bold(), e),
	EntradaEstandar{ error: None } =>
	    s += "Se cerró la entrada estándar.",
	EntradaEstandar{ error: Some(e) } =>
	    s += &format!("Ocurrió un error en la entrada estándar. {}", e),
    }
    s
}

/**
 * Da una cadena que representa de forma legible un mensaje del servidor.
 *
 * # Argumentos
 *
 * `st` - El tipo del mensaje del servidor.
 */
pub fn sistema(st: ServerType) -> String {
    let msg = match st {
	r @ Response{..} => respuesta(r),
	
	NewUser{ username: u } =>
	    format!("* {} se unió a la conversación. *\n",
		    colorea(u)).dimmed().to_string(),
	
	NewStatus{ username: u, status:  Active } =>
	    format!("* {} cambió su estado a {} *\n",
		    colorea(u), "Activo".green()).dimmed().to_string(),
	
	NewStatus{ username: u, status: Away } =>
	    format!("* {} cambió su estado a {} *\n",
		    colorea(u), "Ausente".yellow()).dimmed().to_string(),
	
	NewStatus{ username: u, status: Busy } =>
	    format!("* {} cambió su estado a {} *\n",
		    colorea(u), "Ocupado".red()).dimmed().to_string(),

	UserList{ users: us } => lista(us),

	TextFrom { username: u, text: t } =>
	    format!("[{} (MD)] {}\n", colorea(u), t),
	
	PublicTextFrom{ username: u, text: t } =>
	    format!("[{}] {}\n", colorea(u), t),
	
	Invitation{ username: u, roomname: r } =>
	    format!("* {} te invitó al cuarto {}. *\n",
		    colorea(u), colorea(r)).dimmed().to_string(),
	
	JoinedRoom{ roomname: r, username: u } =>
	    format!("* {} se unió al cuarto {}. *\n",
		    colorea(u), colorea(r)).dimmed().to_string(),
	
	RoomUserList{ roomname: r, users: us } => lista_cuarto(r, us),

	RoomTextFrom{ roomname: r, username: u, text: t } =>
	    format!("[{} @ {}] {}\n", colorea(u), colorea(r), t),

	LeftRoom{ roomname: r, username: u } =>
	    format!("* {} abandonó el cuarto {}. *\n",
		    colorea(u), colorea(r)).dimmed().to_string(),

	Disconnected{ username: u } =>
	    format!("* {} abandonó la conversación. *\n",
		    colorea(u)).dimmed().to_string(),
    };
    msg
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
	    Active => usuarios += &"Activo".green().to_string(),
	    Away => usuarios += &"Ausente".yellow().to_string(),
	    Busy => usuarios += &"Ocupado".red().to_string(),
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
fn respuesta(r: ServerType) -> String {
    let s = match r {
	Response{ operation: Identify, result: Success, extra: Some(n) } =>
	    format!("* Te identificaste como {}. *", colorea(n)),
	Response{ operation: Identify, result: UserAlreadyExists, extra: Some(n) } =>
	    format!("Ya existe un usuario con el nombre {}.", colorea(n)),
	Response{ operation: Text, result: NoSuchUser, extra: Some(n) } =>
	    format!("No existe un usuario con el nombre {}.", n),
	Response{ operation: NewRoom, result: Success, extra: Some(n) } =>
	    format!("* Se creó con éxito el cuarto {}. *", colorea(n)),
	Response{ operation: NewRoom, result: RoomAlreadyExists, extra: Some(n) } =>
	    format!("Ya existe un cuarto llamado {}.", colorea(n)),
	Response{ operation: Invite, result: NoSuchRoom, extra: Some(n) } =>
	    format!("No se pudo invitar porque no existe un cuarto llamado {}.", n),
	Response{ operation: Invite, result: NoSuchUser, extra: Some(n) } =>
	    format!("No se pudo invitar porque no existe un usuario con el nombre {}.", n),
	Response{ operation: JoinRoom, result: Success, extra: Some(n) } =>
	    format!("* Te uniste al cuarto {}. *", colorea(n)),
	Response{ operation: JoinRoom, result: NoSuchRoom, extra: Some(n) } =>
	    format!("No pudiste unirte porque no existe un cuarto llamado {}.", n),
	Response{ operation: JoinRoom, result: NotInvited, extra: Some(n) } =>
	    format!("No pudiste unirte porque no fuiste invitado a {}", colorea(n)),
	Response{ operation: RoomUsers, result: NoSuchRoom, extra: Some(n) } =>
	    format!("No se pudo obtener la lista de miembros porque {} no existe.", n),
	Response{ operation: RoomUsers, result: NotJoined, extra: Some(n) } =>
	    format!("No se pudo obtener la lista de miembros porque no estás en {}.", colorea(n)),
	Response{ operation: LeaveRoom, result: NoSuchRoom, extra: Some(n) } =>
	    format!("No pudiste abandonar el cuarto {} porque no existe", n),
	Response{ operation: LeaveRoom, result: NotJoined, extra: Some(n) } =>
	    format!("No pudiste abandonar el cuarto {} porque no eres miembro.", colorea(n)),
	_ => "Esto no debería suceder".to_string(),
    };
    s.dimmed().to_string() + "\n"
}

/**
 * Maneja lo que sea que el usuario escribió en la consola.
 *
 * # Argumentos
 *
 * `entrada` - La cadena que se recibió de la entrada estándar.
 */
pub fn maneja_stdin(entrada: String) -> Result<Option<String>, ErrorCliente> {
    let limpia = entrada.trim();
    let mut iterador = limpia.splitn(2, ' ');
    let primera = iterador.next();
    match primera.unwrap() {
	"/status" => return nuevo_estado(iterador.next()),
	"/msg" => return mensaje_privado(iterador.next()),
	"/users" => return Ok(Some(users())),
	"/room" => return crea_cuarto(iterador.next()),
	"/join" => return unirse(iterador.next()),
	"/roomusers" => return usuarios_cuarto(iterador.next()),
	"/invite" => return invita(iterador.next()),
	"/roommsg" => return mensaje_cuarto(iterador.next()),
	"/leave" => return abandona(iterador.next()),
	"/disconnect" => return Ok(Some(disconnect())),
	"/help" => {
	    ayuda();
	    return Ok(None);},
	"" => return Ok(None),
	_ => {
	    print!("\x1B[1A\x1B[2K\r[{}] {}\n", colorea("TÚ".to_string()), limpia);
	    return Ok(Some(public_text(limpia.to_string())));
	},
    }
}

/**
 * Regresa el mensaje para cambiar el estado.
 *
 * # Argumentos
 *
 * `estado` - Un `Option<&str>` que puede contener el nuevo estado.
 */
fn nuevo_estado(estado: Option<&str>) -> Result<Option<String>, ErrorCliente> {
    match estado {
	None => return Err(EstadoInvalido),
	Some(s) => {
	    match s {
		"1" | "Activo" => return Ok(Some(status(Active))),
		"2" | "Ausente" => return Ok(Some(status(Away))),
		"3" | "Ocupado" => return Ok(Some(status(Busy))),
		_ => return Err(EstadoInvalido),
	    }
	},
    }
}

/**
 * Regresa el mensaje para enviar un mensaje privado al usuario elegido.
 * 
 * # Argumentos
 *
 * `s` - Un `Option<&str>` que puede contener el nombre del usuario al que enviar
 *       el mensaje y el mensaje a enviar.
 */
fn mensaje_privado(s: Option<&str>) -> Result<Option<String>, ErrorCliente> {
    let resto = match s {
	None => return Err(UsuarioFaltante),
	Some(a) => a,
    };
    let mut iterator = resto.splitn(2, ' ');
    let nom = iterator.next().unwrap();
    if nom == "" { return Err(UsuarioFaltante); }
    if nom.chars().count() > 8 { return Err(NombreMuyLargo); }
    match iterator.next() {
	None => return Err(MensajeFaltante),
	Some(msg) => {
	    match msg.trim() {
		"" => return Err(MensajeFaltante),

		_ => return Ok(Some(text(nom.to_string(), msg.to_string()))),
	    }
	},
    }
}

/**
 * Regresa el mensaje para crear un cuarto con el nombre elegido.
 *
 * # Argumentos
 *
 * `cuarto` - Un `Option<&str>` que posiblemente contiene el nombre del cuarto que se
 *            quiere crear.
 */
fn crea_cuarto(cuarto: Option<&str>) -> Result<Option<String>, ErrorCliente> {
    let nom = match cuarto {
	None => return Err(CuartoFaltante),
	Some(a) => a.trim(),
    };
    if nom == "" { return Err(CuartoFaltante); }
    if nom.chars().count() > 16 { return Err(NombreCuartoMuyLargo); }
    Ok(Some(new_room(nom.to_string())))
}

/**
 * Regresa el mensaje para unirse al cuarto elegido.
 *
 * # Argumentos
 *
 * `cuarto` - Un `Option<&str>` que posiblemente contiene el nombre del cuarto al que
 *            el usuario se quiere unir.
 */
fn unirse(cuarto: Option<&str>) -> Result<Option<String>, ErrorCliente> {
    let nom = match cuarto {
	None => return Err(CuartoFaltante),
	Some(a) => a.trim(),
    };
    if nom == "" { return Err(CuartoFaltante); }
    if nom.chars().count() > 16 { return Err(NombreCuartoMuyLargo); }
    Ok(Some(join_room(nom.to_string())))
}

/**
 * Regresa el mansaje para obtener la lista de usuarios del cuarto elegido.
 *
 * # Argumentos
 *
 * `cuarto` - Un `Option<&str>` que posiblemente contiene el nombre del cuarto cuya
 *            lista de miembros se quiere obtener.
 */
fn usuarios_cuarto(cuarto: Option<&str>) -> Result<Option<String>, ErrorCliente> {
    let nom = match cuarto {
	None => return Err(CuartoFaltante),
	Some(a) => a.trim(),
    };
    if nom == "" { return Err(CuartoFaltante); }
    if nom.chars().count() > 16 { return Err(NombreCuartoMuyLargo); }
    Ok(Some(room_users(nom.to_string())))
}

/**
 * Regresa el mensaje para invitar personas al cuarto elegido.
 *
 * # Argumentos
 *
 * `s` - Un `Option<&str>` que posiblemente contiene el nombre del cuarto al que se
 *       quiere invitar a los usuarios y los nombres de dichos usuarios.
 */
fn invita(s: Option<&str>) -> Result<Option<String>, ErrorCliente> {
    let resto = match s {
	None => return Err(CuartoFaltante),
	Some(a) => a.trim(),
    };
    let mut iterator = resto.split_whitespace();
    let cuarto = iterator.next().unwrap();
    if cuarto == "" { return Err(CuartoFaltante); }
    if cuarto.chars().count() > 16 { return Err(NombreCuartoMuyLargo); }
    let mut invitados: Vec<String> = Vec::new();
    loop {
	match iterator.next() {
	    None => break,
	    Some(a) => {
		match a {
		    "" => continue,
		    _ => invitados.push(a.to_string()),
		}
	    },
	}
    }
    if invitados.is_empty() { return Err(UsuarioFaltante); }
    Ok(Some(invite(cuarto.to_string(), invitados)))
}

/**
 * Regresa el mensaje para enviar un mensaje al cuarto elegido
 *
 * # Argumentos
 *
 * `s` - Un `Option<&str>` que posiblemente contiene el nombre del cuarto al que se
 *       quiere mandar el mensaje y el mensaje que se quiere enviar.
 */
fn mensaje_cuarto(s: Option<&str>) -> Result<Option<String>, ErrorCliente> {
    let resto = match s {
	None => return Err(CuartoFaltante),
	Some(a) => a.trim(),
    };
    let mut iterator = resto.splitn(2, ' ');
    let cuarto = iterator.next().unwrap();
    if cuarto == "" { return Err(CuartoFaltante); }
    if cuarto.chars().count() > 16 { return Err(NombreCuartoMuyLargo); }
    match iterator.next() {
	None => return Err(MensajeFaltante),
	Some(a) => {
	    match a.trim() {
		"" => return Err(MensajeFaltante),
		_ => return Ok(Some(room_text(cuarto.to_string(), a.trim().to_string()))),
	    }
	}
    }
}

fn abandona(cuarto: Option<&str>) -> Result<Option<String>, ErrorCliente> {
    let nom = match cuarto {
	None => return Err(CuartoFaltante),
	Some(a) => a.trim(),
    };
    if nom == "" { return Err(CuartoFaltante); }
    if nom.chars().count() > 16 { return Err(NombreCuartoMuyLargo); }
    Ok(Some(leave_room(nom.to_string())))
}

pub fn ayuda() {
    let help = format!(
	"{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n",
	"/status <estado>                          Cambia tu estado.",
	"    estados: Activo (1), Ausente (2), Ocupado(3)",
	"/msg <usuario> <mensaje>                  Envía un mensaje directo.",
	"/users                                    Lista de usuarios conectados.",
	"/room <cuarto>                            Crea el cuarto dado.",
	"/join <cuarto>                            Se une al cuarto dado.",
	"/roomusers <cuarto>                       Lista de usuarios del cuarto.",
	"/invite <cuarto> <usuario> <usuario> ...  Invita al cuarto a los usuarios.",
	"/roommsg <cuarto> <mensaje>               Envía un mensaje al cuarto.",
	"/leave <cuarto>                           Abandona el cuarto.",
	"/disconnect                               Desconecta del servidor",
	"/help                                     Imprime este mensaje"
    ).dimmed();
    println!("{}", help);
}
