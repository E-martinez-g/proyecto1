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

	TextFrom{ username: u, text: t } =>
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
	    format!("No se pudo enviar el mensaje porque no existe el usuario {}.", n),
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
	Response{ operation: RoomText, result: NoSuchRoom, extra: Some(n) } =>
	    format!("No se pudo mandar el mensaje porque {} no existe.", n),
	Response{ operation: RoomText, result: NotJoined, extra: Some(n) } =>
	    format!("No se pudo mandar el mensaje porque no estás en {}.", colorea(n)),
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

#[cfg(test)]
mod tests {

    use super::*;
    use strip_ansi_escapes::strip_str;
    
    const NOMBRES: [&str; 28] = ["Alicia", "Beto", "Carlos", "David", "Emilio",
				 "Felipe", "Gabriel", "Helena", "Isabel", "Juan",
				 "Karen", "Luana", "Lizandro", "Miguel", "Nicholas",
				 "Ñoño", "Odette", "Paulina", "Queta", "Renata",
				 "Sara", "Tamara", "Ulises", "Victoria", "Wanda",
				 "Ximena", "Yair", "Zair"];

    const CUARTOS: [&str; 14] = ["Sala 1", "Cuartito", "Diversión", "Seriedad",
				 "TCG", "Gente Normal", "Cuarto", "Problemas",
				 "Estudio", "Ayuda", "Juegos", "Arte", "Música",
				 "Secreto"];

    const MENSAJES: [&str; 7] = ["¿Cómo estás?", "¿Qué haces?", "¡Buenos días!",
				 "Necesito ayuda.", "Estoy preocupado.", "¡Hola!",
				 "Voy a reprobar."];
    
    #[test]
    fn test_sistema_new_user() {
	for u in NOMBRES {
	    let p = format!("* {} se unió a la conversación. *\n", u);
	    let nu = NewUser{ username: u.to_string() };
	    assert_eq!(p, strip_str(sistema(nu)));
	}
    }

    #[test]
    fn test_sistema_new_status() {
	for u in NOMBRES {
	    let pac = format!("* {} cambió su estado a {} *\n", u, "Activo");
	    let paw = format!("* {} cambió su estado a {} *\n", u, "Ausente");
	    let pb = format!("* {} cambió su estado a {} *\n", u, "Ocupado");
	    let nsac = NewStatus{ username: u.to_string(), status: Active };
	    let nsaw = NewStatus{ username: u.to_string(), status: Away };
	    let nsb = NewStatus{ username: u.to_string(), status: Busy };
	    assert_eq!(pac, strip_str(sistema(nsac)));
	    assert_eq!(paw, strip_str(sistema(nsaw)));
	    assert_eq!(pb, strip_str(sistema(nsb)));
	}
    }

    #[test]
    fn test_sistema_user_list() {
	for u in NOMBRES {
	    let pac = format!("• {}: Activo\n", u);
	    let paw = format!("• {}: Ausente\n", u);
	    let pb = format!("• {}: Ocupado\n", u);
	    let unoac = HashMap::from([(u.to_string(), Active)]);
	    let unoaw = HashMap::from([(u.to_string(), Away)]);
	    let unob = HashMap::from([(u.to_string(), Busy)]);
	    assert_eq!(pac, strip_str(sistema(UserList{ users: unoac })));
	    assert_eq!(paw, strip_str(sistema(UserList{ users: unoaw })));
	    assert_eq!(pb, strip_str(sistema(UserList{ users: unob })));
	}
	for a in NOMBRES {
	    for b in NOMBRES {
		for c in NOMBRES {
		    if a == b || b == c || a == c { continue; }
		    let p3_1 = format!("• {}: Activo\n• {}: Activo\n• {}: Activo\n", a, b, c);
		    let p3_2 = format!("• {}: Activo\n• {}: Activo\n• {}: Activo\n", a, c, b);
		    let p3_3 = format!("• {}: Activo\n• {}: Activo\n• {}: Activo\n", b, a, c);
		    let p3_4 = format!("• {}: Activo\n• {}: Activo\n• {}: Activo\n", b, c, a);
		    let p3_5 = format!("• {}: Activo\n• {}: Activo\n• {}: Activo\n", c, a, b);
		    let p3_6 = format!("• {}: Activo\n• {}: Activo\n• {}: Activo\n", c, b, a);
		    let tres = HashMap::from([(a.to_string(), Active),
					      (b.to_string(), Active),
					      (c.to_string(), Active)]);
		    let res = strip_str(sistema(UserList{ users: tres }));
		    if res != p3_1 && res != p3_2 && res != p3_3 && res != p3_4 && res != p3_5 && res != p3_6 {
			panic!("No fue ninguna.")
		    }
		}
	    }
	}
    }

    #[test]
    fn test_sistema_text_from() {
	for u in NOMBRES {
	    for m in MENSAJES {
		let p = format!("[{} (MD)] {}\n", u, m);
		let tf = TextFrom{ username: u.to_string(), text: m.to_string()};
		assert_eq!(p, strip_str(sistema(tf)));
	    }
	}
    }

    #[test]
    fn test_sistema_public_text_from() {
	for u in NOMBRES {
	    for m in MENSAJES {
		let p = format!("[{}] {}\n", u, m);
		let ptf = PublicTextFrom{ username: u.to_string(), text: m.to_string()};
		assert_eq!(p, strip_str(sistema(ptf)));
	    }
	}
    }

    #[test]
    fn test_sistema_invitation() {
	for u in NOMBRES {
	    for c in CUARTOS {
		let p = format!("* {} te invitó al cuarto {}. *\n", u, c);
		let i = Invitation{ username: u.to_string(), roomname: c.to_string() };
		assert_eq!(p, strip_str(sistema(i)));
	    }
	}
    }
    #[test]
    fn test_sistema_joined_room() {
	for c in CUARTOS {
	    for u in NOMBRES {
		let p = format!("* {} se unió al cuarto {}. *\n", u, c);
		let jr = JoinedRoom{ roomname: c.to_string(), username: u.to_string() };
		assert_eq!(p, strip_str(sistema(jr)));
	    }
	}
    }

    #[test]
    fn test_sistema_room_user_list() {
	for r in CUARTOS {
	    for u in NOMBRES {
		let pac = format!("Miembros de {}:\n• {}: Activo\n", r, u);
		let paw = format!("Miembros de {}:\n• {}: Ausente\n", r, u);
		let pb = format!("Miembros de {}:\n• {}: Ocupado\n", r, u);
		let unoac = HashMap::from([(u.to_string(), Active)]);
		let unoaw = HashMap::from([(u.to_string(), Away)]);
		let unob = HashMap::from([(u.to_string(), Busy)]);
		assert_eq!(pac, strip_str(sistema(RoomUserList{ roomname: r.to_string(), users: unoac })));
		assert_eq!(paw, strip_str(sistema(RoomUserList{ roomname: r.to_string(), users: unoaw })));
		assert_eq!(pb, strip_str(sistema(RoomUserList{ roomname: r.to_string(), users: unob })));
	    }
	}
    }

    #[test]
    fn test_sistema_room_text_from() {
	for c in CUARTOS {
	    for u in NOMBRES {
		for m in MENSAJES {
		    let p = format!("[{} @ {}] {}\n", u, c, m); // youtube.com/watch?v=399Ez7WHK5s
		    let rtf = RoomTextFrom{ roomname: c.to_string(),
					    username: u.to_string(),
					    text: m.to_string() };
		    assert_eq!(p, strip_str(sistema(rtf)));
		}
	    }
	}
    }

    #[test]
    fn test_sistema_left_room() {
	for c in CUARTOS {
	    for u in NOMBRES {
		let p = format!("* {} abandonó el cuarto {}. *\n", u, c);
		let lr = LeftRoom{ roomname: c.to_string(), username: u.to_string() };
		assert_eq!(p, strip_str(sistema(lr)));
	    }
	}
    }

    #[test]
    fn test_sistema_disconnected() {
	for u in NOMBRES {
	    let p = format!("* {} abandonó la conversación. *\n", u);
	    let d = Disconnected{ username: u.to_string() };
	    assert_eq!(p, strip_str(sistema(d)));
	}
    }

    #[test]
    fn test_sistema_response() {
	for u in NOMBRES {
	    let rids = Response{ operation: Identify, result: Success, extra: Some(u.to_string()) };
	    let ridu = Response{ operation: Identify, result: UserAlreadyExists, extra: Some(u.to_string()) };
	    let rtn = Response{ operation: Text, result: NoSuchUser, extra: Some(u.to_string()) };
	    let rinu = Response{ operation: Invite, result: NoSuchUser, extra: Some(u.to_string()) };
	    let pids = format!("* Te identificaste como {}. *\n", u);
	    let pidu = format!("Ya existe un usuario con el nombre {}.\n", u);
	    let ptn = format!("No se pudo enviar el mensaje porque no existe el usuario {}.\n", u);
	    let pinu = format!("No se pudo invitar porque no existe un usuario con el nombre {}.\n", u);
	    assert_eq!(pids, strip_str(sistema(rids)));
	    assert_eq!(pidu, strip_str(sistema(ridu)));
	    assert_eq!(ptn, strip_str(sistema(rtn)));
	    assert_eq!(pinu, strip_str(sistema(rinu)));
	}
	for c in CUARTOS {
	    let rns  = Response{ operation: NewRoom, result: Success, extra: Some(c.to_string()) };
	    let rne  = Response{ operation: NewRoom, result: RoomAlreadyExists, extra: Some(c.to_string()) };
	    let rinr = Response{ operation: Invite, result: NoSuchRoom, extra: Some(c.to_string()) };
	    let rjs  = Response{ operation: JoinRoom, result: Success, extra: Some(c.to_string()) };
	    let rjnr = Response{ operation: JoinRoom, result: NoSuchRoom, extra: Some(c.to_string()) };
	    let rjni = Response{ operation: JoinRoom, result: NotInvited, extra: Some(c.to_string()) };
	    let runr = Response{ operation: RoomUsers, result: NoSuchRoom, extra: Some(c.to_string()) };
	    let runj = Response{ operation: RoomUsers, result: NotJoined, extra: Some(c.to_string()) };
	    let rtnr = Response{ operation: RoomText, result: NoSuchRoom, extra: Some(c.to_string()) };
	    let rtnj = Response{ operation: RoomText, result: NotJoined, extra: Some(c.to_string()) };
	    let rlnr = Response{ operation: LeaveRoom, result: NoSuchRoom, extra: Some(c.to_string()) };
	    let rlnj = Response{ operation: LeaveRoom, result: NotJoined, extra: Some(c.to_string()) };
	    let pns  = format!("* Se creó con éxito el cuarto {}. *\n", c);
	    let pne  = format!("Ya existe un cuarto llamado {}.\n", c);
	    let pinr = format!("No se pudo invitar porque no existe un cuarto llamado {}.\n", c);
	    let pjs  = format!("* Te uniste al cuarto {}. *\n", c);
	    let pjnr = format!("No pudiste unirte porque no existe un cuarto llamado {}.\n", c);
	    let pjni = format!("No pudiste unirte porque no fuiste invitado a {}\n", c);
	    let punr = format!("No se pudo obtener la lista de miembros porque {} no existe.\n", c);
	    let punj = format!("No se pudo obtener la lista de miembros porque no estás en {}.\n", c);
	    let ptnr = format!("No se pudo mandar el mensaje porque {} no existe.\n", c);
	    let ptnj = format!("No se pudo mandar el mensaje porque no estás en {}.\n", c);
	    let plnr = format!("No pudiste abandonar el cuarto {} porque no existe\n", c);
	    let plnj = format!("No pudiste abandonar el cuarto {} porque no eres miembro.\n", c);
	    assert_eq!(pns,  strip_str(sistema(rns)));
	    assert_eq!(pne,  strip_str(sistema(rne)));
	    assert_eq!(pinr, strip_str(sistema(rinr)));
	    assert_eq!(pjs,  strip_str(sistema(rjs)));
	    assert_eq!(pjnr, strip_str(sistema(rjnr)));
	    assert_eq!(pjni, strip_str(sistema(rjni)));
	    assert_eq!(punr, strip_str(sistema(runr)));
	    assert_eq!(punj, strip_str(sistema(runj)));
	    assert_eq!(ptnr, strip_str(sistema(rtnr)));
	    assert_eq!(ptnj, strip_str(sistema(rtnj)));
	    assert_eq!(plnr, strip_str(sistema(rlnr)));
	    assert_eq!(plnj, strip_str(sistema(rlnj)));
	}
    }
}
