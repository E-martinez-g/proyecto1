use protocolo::{*, ServerType::*, ClientType::*, EstadoUsuario::*, Operacion::*, Resultado::*, mensajes_cliente::*, mensajes_servidor::*};
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

const OPERACIONES: [Operacion; 9] = [Identify, Text, NewRoom, Invite, JoinRoom,
				     RoomUsers, RoomText, LeaveRoom,
				     Operacion::Invalid];

const RESULTADOS: [Resultado; 9] = [Success, UserAlreadyExists, NoSuchUser,
				    RoomAlreadyExists, NoSuchRoom, NotInvited,
				    NotJoined, NotIdentified, Resultado::Invalid];

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

#[test]
fn test_parsea_mensaje_servidor_new_user() {
    for u in NOMBRES {
	let nu = match parsea_mensaje_servidor(new_user(&u.to_string())) {
	    Err(_) => panic!("No pudo parsear."),
	    Ok(None) => panic!("No parseo nada."),
	    Ok(Some(st)) => st,
	};
	assert_eq!(NewUser{ username: u.to_string() }, nu);
    }
}

#[test]
fn test_parsea_mensaje_servidor_new_status() {
    for u in NOMBRES {
	let nsac = match parsea_mensaje_servidor(new_status(&u.to_string(), &Active)) {
	    Err(_) => panic!("No pudo parsear."),
	    Ok(None) => panic!("No parseo nada."),
	    Ok(Some(st)) => st,
	};
	let nsaw = match parsea_mensaje_servidor(new_status(&u.to_string(), &Away)) {
	    Err(_) => panic!("No pudo parsear."),
	    Ok(None) => panic!("No parseo nada."),
	    Ok(Some(st)) => st,
	};
	let nsb = match parsea_mensaje_servidor(new_status(&u.to_string(), &Busy)) {
	    Err(_) => panic!("No pudo parsear."),
	    Ok(None) => panic!("No parseo nada."),
	    Ok(Some(st)) => st,
	};
	assert_eq!(NewStatus{ username: u.to_string(), status: Active }, nsac);
	assert_eq!(NewStatus{ username: u.to_string(), status: Away }, nsaw);
	assert_eq!(NewStatus{ username: u.to_string(), status: Busy }, nsb);
    }
}

#[test]
fn test_parsea_mensaje_servidor_user_list() {
    for a in NOMBRES {
	for b in NOMBRES {
	    for c in NOMBRES {
		if a == b || b == c || a == c { continue; }
		let m = HashMap::from([(a.to_string(), Active),
				       (b.to_string(), Busy),
				       (c.to_string(), Away)]);
		let ul = match parsea_mensaje_servidor(user_list(&m)) {
		    Err(_) => panic!("No pudo parsear."),
		    Ok(None) => panic!("No parseo nada."),
		    Ok(Some(st)) => st,
		};
		assert_eq!(UserList{ users: m }, ul);
	    }
	}
    }
}

#[test]
fn test_parsea_mensaje_servidor_text_from() {
    for u in NOMBRES {
	for m in MENSAJES {
	    let tf = match parsea_mensaje_servidor(text_from(&u.to_string(), m.to_string())) {
		Err(_) => panic!("No pudo parsear."),
		Ok(None) => panic!("No parseo nada."),
		Ok(Some(st)) => st,
	    };
	    assert_eq!(TextFrom{ username: u.to_string(), text: m.to_string() }, tf);
	}
    }
}

#[test]
fn test_parsea_mensaje_servidor_public_text_from() {
    for u in NOMBRES {
	for m in MENSAJES {
	    let ptf = match parsea_mensaje_servidor(public_text_from(&u.to_string(), m.to_string())) {
		Err(_) => panic!("No pudo parsear."),
		Ok(None) => panic!("No parseo nada."),
		Ok(Some(st)) => st,
	    };
	    assert_eq!(PublicTextFrom{ username: u.to_string(), text: m.to_string() }, ptf);
	}
    }
}

#[test]
fn test_parsea_mensaje_servidor_invitation() {
    for u in NOMBRES {
	for c in CUARTOS {
	    let i = match parsea_mensaje_servidor(invitation(&u.to_string(), &c.to_string())) {
		Err(_) => panic!("No pudo parsear."),
		Ok(None) => panic!("No parseo nada."),
		Ok(Some(st)) => st,
	    };
	    assert_eq!(Invitation{ username: u.to_string(), roomname: c.to_string() }, i);
	}
    }
}

#[test]
fn test_parsea_mensaje_servidor_joined_room() {
    for c in CUARTOS {
	for u in NOMBRES {
	    let jr = match parsea_mensaje_servidor(joined_room(&c.to_string(), &u.to_string())) {
		Err(_) => panic!("No pudo parsear."),
		Ok(None) => panic!("No parseo nada."),
		Ok(Some(st)) => st,
	    };
	    assert_eq!(JoinedRoom{ roomname: c.to_string(), username: u.to_string() }, jr);
	}
    }
}

#[test]
fn test_parsea_mensaje_servidor_room_user_list() {
    for r in CUARTOS {
	for a in NOMBRES {
	    for b in NOMBRES {
		if a == b { continue; }
		let m = HashMap::from([(a.to_string(), Active),
				       (b.to_string(), Busy)]);
		let rul = match parsea_mensaje_servidor(room_user_list(&r.to_string(), m.clone())) {
		    Err(_) => panic!("No pudo parsear."),
		    Ok(None) => panic!("No parseo nada."),
		    Ok(Some(st)) => st,
		};
		assert_eq!(RoomUserList{ roomname: r.to_string(), users: m }, rul);
	    }
	}
    }
}

#[test]
fn test_parsea_mensaje_servidor_room_text_from() {
    for c in CUARTOS {
	for u in NOMBRES {
	    for m in MENSAJES {
		let rtf = match parsea_mensaje_servidor(room_text_from(&c.to_string(), &u.to_string(), m.to_string())) {
		    Err(_) => panic!("No pudo parsear."),
		    Ok(None) => panic!("No parseo nada."),
		    Ok(Some(st)) => st,
		};
		assert_eq!(RoomTextFrom{ roomname: c.to_string(), username: u.to_string(), text: m.to_string() }, rtf);
	    }
	}
    }
}

#[test]
fn test_parsea_mensaje_servidor_left_room() {
    for c in CUARTOS {
	for u in NOMBRES {
	    let lr = match parsea_mensaje_servidor(left_room(&c.to_string(), &u.to_string())) {
		Err(_) => panic!("No pudo parsear."),
		Ok(None) => panic!("No parseo nada."),
		Ok(Some(st)) => st,
	    };
	    assert_eq!(LeftRoom{ roomname: c.to_string(), username: u.to_string() }, lr);
	}
    }
}

#[test]
fn test_parsea_mensaje_servidor_disconnected() {
    for u in NOMBRES {
	let d = match parsea_mensaje_servidor(disconnected(&u.to_string())) {
	    Err(_) => panic!("No pudo parsear."),
	    Ok(None) => panic!("No parseo nada."),
	    Ok(Some(st)) => st,
	};
	assert_eq!(Disconnected{ username: u.to_string() }, d);
    }
}

#[test]
fn test_parsea_mensaje_servidor_response() {
    for o in OPERACIONES {
	for r in RESULTADOS {
	    let opr = match o {
		Identify => "IDENTIFY",
		Text => "TEXT",
		NewRoom => "NEW_ROOM",
		Invite => "INVITE",
		JoinRoom => "JOIN_ROOM",
		RoomUsers => "ROOM_USERS",
		RoomText => "ROOM_TEXT",
		LeaveRoom => "LEAVE_ROOM",
		Operacion::Invalid => "INVALID",
	    };
	    let res = match r {
		Success => "SUCCESS",
		UserAlreadyExists => "USER_ALREADY_EXISTS",
		NoSuchUser => "NO_SUCH_USER",
		RoomAlreadyExists => "ROOM_ALREADY_EXISTS",
		NoSuchRoom => "NO_SUCH_ROOM",
		NotInvited => "NOT_INVITED",
		NotJoined => "NOT_JOINED",
		NotIdentified => "NOT_IDENTIFIED",
		Resultado::Invalid => "INVALID",
	    };
	    let response = match parsea_mensaje_servidor(response(opr, res)) {
		Err(_) => panic!("No pudo parsear."),
		Ok(None) => panic!("No parseo nada."),
		Ok(Some(st)) => st,
	    };
	    assert_eq!(Response{ operation: o, result: r, extra: None }, response);
	}
    }
}

#[test]
fn test_parsea_mensaje_servidor_response_extra() {
    for u in NOMBRES {
	for o in OPERACIONES {
	    for r in RESULTADOS {
		let opr = match o {
		    Identify => "IDENTIFY",
		    Text => "TEXT",
		    NewRoom => "NEW_ROOM",
		    Invite => "INVITE",
		    JoinRoom => "JOIN_ROOM",
		    RoomUsers => "ROOM_USERS",
		    RoomText => "ROOM_TEXT",
		    LeaveRoom => "LEAVE_ROOM",
		    Operacion::Invalid => "INVALID",
		};
		let res = match r {
		    Success => "SUCCESS",
		    UserAlreadyExists => "USER_ALREADY_EXISTS",
		    NoSuchUser => "NO_SUCH_USER",
		    RoomAlreadyExists => "ROOM_ALREADY_EXISTS",
		    NoSuchRoom => "NO_SUCH_ROOM",
		    NotInvited => "NOT_INVITED",
		    NotJoined => "NOT_JOINED",
		    NotIdentified => "NOT_IDENTIFIED",
		    Resultado::Invalid => "INVALID",
		};
		let response = match parsea_mensaje_servidor(response_extra(opr, res, &u.to_string())) {
		    Err(_) => panic!("No pudo parsear."),
		    Ok(None) => panic!("No parseo nada."),
		    Ok(Some(st)) => st,
		};
		assert_eq!(Response{ operation: o, result: r, extra: Some(u.to_string()) }, response);
	    }
	}
    }
    for c in CUARTOS {
	for o in OPERACIONES {
	    for r in RESULTADOS {
		let opr = match o {
		    Identify => "IDENTIFY",
		    Text => "TEXT",
		    NewRoom => "NEW_ROOM",
		    Invite => "INVITE",
		    JoinRoom => "JOIN_ROOM",
		    RoomUsers => "ROOM_USERS",
		    RoomText => "ROOM_TEXT",
		    LeaveRoom => "LEAVE_ROOM",
		    Operacion::Invalid => "INVALID",
		};
		let res = match r {
		    Success => "SUCCESS",
		    UserAlreadyExists => "USER_ALREADY_EXISTS",
		    NoSuchUser => "NO_SUCH_USER",
		    RoomAlreadyExists => "ROOM_ALREADY_EXISTS",
		    NoSuchRoom => "NO_SUCH_ROOM",
		    NotInvited => "NOT_INVITED",
		    NotJoined => "NOT_JOINED",
		    NotIdentified => "NOT_IDENTIFIED",
		    Resultado::Invalid => "INVALID",
		};
		let response = match parsea_mensaje_servidor(response_extra(opr, res, &c.to_string())) {
		    Err(_) => panic!("No pudo parsear."),
		    Ok(None) => panic!("No parseo nada."),
		    Ok(Some(st)) => st,
		};
		assert_eq!(Response{ operation: o, result: r, extra: Some(c.to_string()) }, response);
	    }
	}
    }
}
