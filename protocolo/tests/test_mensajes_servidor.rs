use protocolo::mensajes_servidor::*;
use protocolo::EstadoUsuario::*;
use std::collections::HashMap;

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

const OPERACIONES: [&str; 9] = ["IDENTIFY", "TEXT", "NEW_ROOM", "INVITE",
				"JOIN_ROOM", "ROOM_USERS", "ROOM_TEXT",
				"LEAVE_ROOM", "INVALID"];

const RESULTADOS: [&str; 9] = ["SUCCESS", "USER_ALREADY_EXISTS", "NO_SUCH_USER",
			       "ROOM_ALREADY_EXISTS", "NO_SUCH_ROOM",
			       "NOT_INVITED", "NOT_JOINED", "NOT_IDENTIFIED",
			       "INVALID"];

#[test]
fn test_response() {
    for o in OPERACIONES {
	for r in RESULTADOS {
	    let p = format!("{{\"type\":\"RESPONSE\",\"operation\":\"{}\",\"result\":\"{}\"}}\n\0", o, r);
	    assert_eq!(p, response(o, r));
	}
    }
}

#[test]
fn test_response_extra() {
    for u in NOMBRES {
	for o in OPERACIONES {
	    for r in RESULTADOS {
		let p =
		    format!("{{\"type\":\"RESPONSE\",\"operation\":\"{}\",\"result\":\"{}\",\"extra\":\"{}\"}}\n\0",
			    o, r, u);
		assert_eq!(p, response_extra(o, r, &u.to_string()));
	    }
	}
    }
    for c in CUARTOS {
	for o in OPERACIONES {
	    for r in RESULTADOS {
		let p =
		    format!("{{\"type\":\"RESPONSE\",\"operation\":\"{}\",\"result\":\"{}\",\"extra\":\"{}\"}}\n\0",
			    o, r, c);
		assert_eq!(p, response_extra(o, r, &c.to_string()));
	    }
	}
    }
}

#[test]
fn test_new_user() {
    for u in NOMBRES {
	let p = format!("{{\"type\":\"NEW_USER\",\"username\":\"{}\"}}\n\0", u);
	assert_eq!(p, new_user(&u.to_string()));
    }
}

#[test]
fn test_new_status() {
    for u in NOMBRES {
	assert_eq!(format!("{{\"type\":\"NEW_STATUS\",\"username\":\"{}\",\"status\":\"{}\"}}\n\0", u, "ACTIVE"),
		   new_status(&u.to_string(), &Active));
	assert_eq!(format!("{{\"type\":\"NEW_STATUS\",\"username\":\"{}\",\"status\":\"{}\"}}\n\0", u, "AWAY"),
		   new_status(&u.to_string(), &Away));
	assert_eq!(format!("{{\"type\":\"NEW_STATUS\",\"username\":\"{}\",\"status\":\"{}\"}}\n\0", u, "BUSY"),
		   new_status(&u.to_string(), &Busy));
    }
}

#[test]
fn test_user_list() {
    for u in NOMBRES {
	let unoac = HashMap::from([(u.to_string(), Active)]);
	let p1ac = format!("{{\"type\":\"USER_LIST\",\"users\":{{\"{}\":\"{}\"}}}}\n\0", u, "ACTIVE");
	assert_eq!(p1ac, user_list(&unoac));
    }
    for u in NOMBRES {
	let unoaw = HashMap::from([(u.to_string(), Away)]);
	let p1aw = format!("{{\"type\":\"USER_LIST\",\"users\":{{\"{}\":\"{}\"}}}}\n\0", u, "AWAY");
	assert_eq!(p1aw, user_list(&unoaw));
    }
    for u in NOMBRES {
	let unob = HashMap::from([(u.to_string(), Busy)]);
	let p1b = format!("{{\"type\":\"USER_LIST\",\"users\":{{\"{}\":\"{}\"}}}}\n\0", u, "BUSY");
	assert_eq!(p1b, user_list(&unob));
    }
    for a in NOMBRES {
	for b in NOMBRES {
	    for c in NOMBRES {
		if a == b || b == c || a == c { continue; }
		let tres = HashMap::from([(a.to_string(), Active),
					  (b.to_string(), Busy),
					  (c.to_string(), Away)]);
		let p3_1 =
		    format!("{{\"type\":\"USER_LIST\",\"users\":{{\"{}\":\"{}\",\"{}\":\"{}\",\"{}\":\"{}\"}}}}\n\0",
			    a, "ACTIVE", b, "BUSY", c, "AWAY");
		let p3_2 =
		    format!("{{\"type\":\"USER_LIST\",\"users\":{{\"{}\":\"{}\",\"{}\":\"{}\",\"{}\":\"{}\"}}}}\n\0",
			    a, "ACTIVE", c, "AWAY", b, "BUSY");
		let p3_3 =
		    format!("{{\"type\":\"USER_LIST\",\"users\":{{\"{}\":\"{}\",\"{}\":\"{}\",\"{}\":\"{}\"}}}}\n\0",
			    b, "BUSY", c, "AWAY", a, "ACTIVE");
		let p3_4 =
		    format!("{{\"type\":\"USER_LIST\",\"users\":{{\"{}\":\"{}\",\"{}\":\"{}\",\"{}\":\"{}\"}}}}\n\0",
			    b, "BUSY", a, "ACTIVE", c, "AWAY");
		let p3_5 =
		    format!("{{\"type\":\"USER_LIST\",\"users\":{{\"{}\":\"{}\",\"{}\":\"{}\",\"{}\":\"{}\"}}}}\n\0",
			    c, "AWAY", a, "ACTIVE", b, "BUSY");
		let p3_6 =
		    format!("{{\"type\":\"USER_LIST\",\"users\":{{\"{}\":\"{}\",\"{}\":\"{}\",\"{}\":\"{}\"}}}}\n\0",
			    c, "AWAY", b, "BUSY", a, "ACTIVE");
		let res = user_list(&tres);
		if res != p3_1 && res != p3_2 && res != p3_3 && res != p3_4 && res != p3_5 && res != p3_6 {
		    panic!("Fue diferente a todas.");
		}
	    }
	}
    }
}

#[test]
fn test_text_from() {
    for u in NOMBRES {
	for m in MENSAJES {
	    let p = format!("{{\"type\":\"TEXT_FROM\",\"username\":\"{}\",\"text\":\"{}\"}}\n\0", u, m);
	    assert_eq!(p, text_from(&u.to_string(), m.to_string()));
	}
    }
}

#[test]
fn test_public_text_from() {
    for u in NOMBRES {
	for m in MENSAJES {
	    let p = format!("{{\"type\":\"PUBLIC_TEXT_FROM\",\"username\":\"{}\",\"text\":\"{}\"}}\n\0", u, m);
	    assert_eq!(p, public_text_from(&u.to_string(), m.to_string()));
	}
    }
}

#[test]
fn test_invitation() {
    for u in NOMBRES{
	for c in CUARTOS {
	    let p = format!("{{\"type\":\"INVITATION\",\"username\":\"{}\",\"roomname\":\"{}\"}}\n\0", u, c);
	    assert_eq!(p, invitation(&u.to_string(), &c.to_string()));
	}
    }
}

#[test]
fn test_joined_room() {
    for u in NOMBRES {
	for c in CUARTOS {
	    let p = format!("{{\"type\":\"JOINED_ROOM\",\"roomname\":\"{}\",\"username\":\"{}\"}}\n\0", c, u);
	    assert_eq!(p, joined_room(&c.to_string(), &u.to_string()));
	}
    }
}

#[test]
fn test_room_user_list() {
    for r in CUARTOS {
	for a in NOMBRES {
	    for b in NOMBRES {
		for c in NOMBRES {
		    if a == b || b == c || a == c { continue; }
		    let tres = HashMap::from([(a.to_string(), Active),
					      (b.to_string(), Busy),
					      (c.to_string(), Away)]);
		    let p3_1 =
			format!("{{\"type\":\"ROOM_USER_LIST\",\"roomname\":\"{}\",\"users\":{{\"{}\":\"{}\",\"{}\":\"{}\",\"{}\":\"{}\"}}}}\n\0",
				r, a, "ACTIVE", b, "BUSY", c, "AWAY");
		    let p3_2 =
			format!("{{\"type\":\"ROOM_USER_LIST\",\"roomname\":\"{}\",\"users\":{{\"{}\":\"{}\",\"{}\":\"{}\",\"{}\":\"{}\"}}}}\n\0",
				r, a, "ACTIVE", c, "AWAY", b, "BUSY");
		    let p3_3 =
			format!("{{\"type\":\"ROOM_USER_LIST\",\"roomname\":\"{}\",\"users\":{{\"{}\":\"{}\",\"{}\":\"{}\",\"{}\":\"{}\"}}}}\n\0",
				r, b, "BUSY", c, "AWAY", a, "ACTIVE");
		    let p3_4 =
			format!("{{\"type\":\"ROOM_USER_LIST\",\"roomname\":\"{}\",\"users\":{{\"{}\":\"{}\",\"{}\":\"{}\",\"{}\":\"{}\"}}}}\n\0",
				r, b, "BUSY", a, "ACTIVE", c, "AWAY");
		    let p3_5 =
			format!("{{\"type\":\"ROOM_USER_LIST\",\"roomname\":\"{}\",\"users\":{{\"{}\":\"{}\",\"{}\":\"{}\",\"{}\":\"{}\"}}}}\n\0",
				r, c, "AWAY", a, "ACTIVE", b, "BUSY");
		    let p3_6 =
			format!("{{\"type\":\"ROOM_USER_LIST\",\"roomname\":\"{}\",\"users\":{{\"{}\":\"{}\",\"{}\":\"{}\",\"{}\":\"{}\"}}}}\n\0",
				r, c, "AWAY", b, "BUSY", a, "ACTIVE");
		    let res = room_user_list(&r.to_string(), tres);
		    if res != p3_1 && res != p3_2 && res != p3_3 && res != p3_4 && res != p3_5 && res != p3_6 {
			panic!("Fue diferente a todas.");
		    }
		}
	    }
	}
    }
}

#[test]
fn test_room_text_from() {
    for c in CUARTOS {
	for u in NOMBRES {
	    for m in MENSAJES {
		let p = format!("{{\"type\":\"ROOM_TEXT_FROM\",\"roomname\":\"{}\",\"username\":\"{}\",\"text\":\"{}\"}}\n\0", c, u, m); //xd
		assert_eq!(p, room_text_from(&c.to_string(), &u.to_string(), m.to_string()));
	    }
	}
    }
}

#[test]
fn test_left_room() {
    for c in CUARTOS {
	for u in NOMBRES {
	    let p = format!("{{\"type\":\"LEFT_ROOM\",\"roomname\":\"{}\",\"username\":\"{}\"}}\n\0", c, u);
	    assert_eq!(p, left_room(&c.to_string(), &u.to_string()));
	}
    }
}

#[test]
fn test_disconnected() {
    for u in NOMBRES {
	let p = format!("{{\"type\":\"DISCONNECTED\",\"username\":\"{}\"}}\n\0", u);
	assert_eq!(p, disconnected(&u.to_string()));
    }
}
