use protocolo::mensajes_cliente::*;
use protocolo::EstadoUsuario::*;

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
fn test_identify() {
    for u in NOMBRES {
	let p = format!("{{\"type\":\"IDENTIFY\",\"username\":\"{}\"}}\n\0", u);
	assert_eq!(p, identify(u.to_string()));
    }
}

#[test]
fn test_status() {
    assert_eq!(format!("{{\"type\":\"STATUS\",\"status\":\"{}\"}}\n\0", "ACTIVE"), status(Active));
    assert_eq!(format!("{{\"type\":\"STATUS\",\"status\":\"{}\"}}\n\0", "AWAY"), status(Away));
    assert_eq!(format!("{{\"type\":\"STATUS\",\"status\":\"{}\"}}\n\0", "BUSY"), status(Busy));
}

#[test]
fn test_users() {
    let p = "{\"type\":\"USERS\"}\n\0";
    assert_eq!(p, users());
}

#[test]
fn test_text() {
    for u in NOMBRES {
	for m in MENSAJES {
	    let p = format!("{{\"type\":\"TEXT\",\"username\":\"{}\",\"text\":\"{}\"}}\n\0", u, m);
	    assert_eq!(p, text(u.to_string(), m.to_string()));
	}
    }
}

#[test]
fn test_public_text() {
    for m in MENSAJES {
	let p = format!("{{\"type\":\"PUBLIC_TEXT\",\"text\":\"{}\"}}\n\0", m);
	assert_eq!(p, public_text(m.to_string()));
    }
}

#[test]
fn test_new_room() {
    for c in CUARTOS {
	let p = format!("{{\"type\":\"NEW_ROOM\",\"roomname\":\"{}\"}}\n\0", c);
	assert_eq!(p, new_room(c.to_string()));
    }
}

#[test]
fn test_invite() {
    for r in CUARTOS {
	for u in NOMBRES {
	    let uno = vec![u.to_string()];
	    let p1 = format!("{{\"type\":\"INVITE\",\"roomname\":\"{}\",\"usernames\":[\"{}\"]}}\n\0", r, u);
	    assert_eq!(p1, invite(r.to_string(), uno));
	}
	for a in NOMBRES {
	    for b in NOMBRES {
		for c in NOMBRES {
		    let tres = vec![a.to_string(), b.to_string(), c.to_string()];
		    let p3 =
			format!("{{\"type\":\"INVITE\",\"roomname\":\"{}\",\"usernames\":[\"{}\",\"{}\",\"{}\"]}}\n\0",
				r, a, b, c);
		    assert_eq!(p3, invite(r.to_string(), tres));

		}
	    }
	}
    }
}

#[test]
fn test_join_room() {
    for c in CUARTOS {
	let p = format!("{{\"type\":\"JOIN_ROOM\",\"roomname\":\"{}\"}}\n\0", c);
	assert_eq!(p, join_room(c.to_string()));
    }
}

#[test]
fn test_room_users() {
    for c in CUARTOS {
	let p = format!("{{\"type\":\"ROOM_USERS\",\"roomname\":\"{}\"}}\n\0", c);
	assert_eq!(p, room_users(c.to_string()));
    }
}

#[test]
fn test_room_text() {
    for c in CUARTOS {
	for m in MENSAJES {
	    let p = format!("{{\"type\":\"ROOM_TEXT\",\"roomname\":\"{}\",\"text\":\"{}\"}}\n\0", c, m);
	    assert_eq!(p, room_text(c.to_string(), m.to_string()));
	}
    }
}

#[test]
fn test_leave_room() {
    for c in CUARTOS {
	let p = format!("{{\"type\":\"LEAVE_ROOM\",\"roomname\":\"{}\"}}\n\0", c);
	assert_eq!(p, leave_room(c.to_string()));
    }
}

#[test]
fn test_disconnect() {
    let p = "{\"type\":\"DISCONNECT\"}\n\0";
    assert_eq!(p, disconnect());
}
