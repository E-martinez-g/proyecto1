use protocolo::{*, ServerType::*, ClientType::*, EstadoUsuario::*, Operacion::*, Resultado::*, mensajes_cliente::*, mensajes_servidor::*};

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
fn test_parsea_mensaje_cliente_identify() {
    for u in NOMBRES {
	let i = match parsea_mensaje_cliente(identify(u.to_string())) {
	    Err(_) => panic!("No pudo parsear."),
	    Ok(None) => panic!("No parseo nada."),
	    Ok(Some(ct)) => ct,
	};
	assert_eq!(ClientType::Identify{ username: u.to_string() }, i);
    }
}

#[test]
fn test_parsea_mensaje_cliente_status() {
    let sac = match parsea_mensaje_cliente(status(Active)) {
	Err(_) => panic!("No pudo parsear."),
	Ok(None) => panic!("No parseo nada."),
	Ok(Some(ct)) => ct,
    };
    let saw = match parsea_mensaje_cliente(status(Away)) {
	Err(_) => panic!("No pudo parsear."),
	Ok(None) => panic!("No parseo nada."),
	Ok(Some(ct)) => ct,
    };
    let sb = match parsea_mensaje_cliente(status(Busy)) {
	Err(_) => panic!("No pudo parsear."),
	Ok(None) => panic!("No parseo nada."),
	Ok(Some(ct)) => ct,
    };
    assert_eq!(Status{ status: Active }, sac);
    assert_eq!(Status{ status: Away }, saw);
    assert_eq!(Status{ status: Busy }, sb);
}

#[test]
fn test_parsea_mensaje_cliente_users() {
    let u = match parsea_mensaje_cliente(users()) {
	Err(_) => panic!("No pudo parsear."),
	Ok(None) => panic!("No parseo nada."),
	Ok(Some(ct)) => ct,
    };
    assert_eq!(Users, u);
}

#[test]
fn test_parsea_mensaje_cliente_text() {
    for u in NOMBRES {
	for m in MENSAJES {
	    let t = match parsea_mensaje_cliente(text(u.to_string(), m.to_string())) {
		Err(_) => panic!("No pudo parsear."),
		Ok(None) => panic!("No parseo nada."),
		Ok(Some(ct)) => ct,
	    };
	    assert_eq!(ClientType::Text{ username: u.to_string(), text: m.to_string() }, t);
	}
    }
}

#[test]
fn test_parsea_mensaje_cliente_public_text() {
    for m in MENSAJES {
	let pt = match parsea_mensaje_cliente(public_text(m.to_string())) {
	    Err(_) => panic!("No pudo parsear."),
	    Ok(None) => panic!("No parseo nada."),
	    Ok(Some(ct)) => ct,
	};
	assert_eq!(PublicText{ text: m.to_string() }, pt);
    }
}

#[test]
fn test_parsea_mensaje_cliente_new_room() {
    for c in CUARTOS {
	let nr = match parsea_mensaje_cliente(new_room(c.to_string())) {
	    Err(_) => panic!("No pudo parsear."),
	    Ok(None) => panic!("No parseo nada."),
	    Ok(Some(ct)) => ct,
	};
	assert_eq!(ClientType::NewRoom{ roomname: c.to_string() }, nr);
    }
}

#[test]
fn test_parsea_mensaje_cliente_invite() {
    for c in CUARTOS {
	for a in NOMBRES {
	    for b in NOMBRES {
		let v = vec![a.to_string(), b.to_string()];
		let nr = match parsea_mensaje_cliente(invite(c.to_string(), v.clone())) {
		    Err(_) => panic!("No pudo parsear."),
		    Ok(None) => panic!("No parseo nada."),
		    Ok(Some(ct)) => ct,
		};
		assert_eq!(ClientType::Invite{ roomname: c.to_string(), usernames: v }, nr);
	    }
	}
    }
}

#[test]
fn test_parsea_mensaje_cliente_join_room() {
    for c in CUARTOS {
	let jr = match parsea_mensaje_cliente(join_room(c.to_string())) {
	    Err(_) => panic!("No pudo parsear."),
	    Ok(None) => panic!("No parseo nada."),
	    Ok(Some(ct)) => ct,
	};
	assert_eq!(ClientType::JoinRoom{ roomname: c.to_string() }, jr);
    }
}

#[test]
fn test_parsea_mensaje_cliente_room_users() {
    for c in CUARTOS {
	let ru = match parsea_mensaje_cliente(room_users(c.to_string())) {
	    Err(_) => panic!("No pudo parsear."),
	    Ok(None) => panic!("No parseo nada."),
	    Ok(Some(ct)) => ct,
	};
	assert_eq!(ClientType::RoomUsers{ roomname: c.to_string() }, ru);
    }
}

#[test]
fn test_parsea_mensaje_cliente_room_text() {
    for c in CUARTOS {
	for m in MENSAJES {
	    let rt = match parsea_mensaje_cliente(room_text(c.to_string(), m.to_string())) {
		Err(_) => panic!("No pudo parsear."),
		Ok(None) => panic!("No parseo nada."),
		Ok(Some(ct)) => ct,
	    };
	    assert_eq!(ClientType::RoomText{ roomname: c.to_string(), text: m.to_string() }, rt);
	}
    }
}

#[test]
fn test_parsea_mensaje_cliente_leave_room() {
    for c in CUARTOS {
	let lr = match parsea_mensaje_cliente(leave_room(c.to_string())) {
	    Err(_) => panic!("No pudo parsear."),
	    Ok(None) => panic!("No parseo nada."),
	    Ok(Some(ct)) => ct,
	};
	assert_eq!(ClientType::LeaveRoom{ roomname: c.to_string() }, lr);
    }
}

#[test]
fn test_parsea_mensaje_cliente_disconnect() {
    let d = match parsea_mensaje_cliente(disconnect()) {
	Err(_) => panic!("No pudo parsear."),
	Ok(None) => panic!("No parseo nada."),
	Ok(Some(ct)) => ct,
    };
    assert_eq!(Disconnect, d);
}
