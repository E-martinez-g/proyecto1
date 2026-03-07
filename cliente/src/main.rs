use std::net::TcpStream;
use std::io;
use std::io::{Stdin, BufRead, Read, Write};

use protocolo::*;
use protocolo::ServerType::*;
use protocolo::mensajes_cliente::*;

fn main() {
    let entrada_estandar = io::stdin();
    
    let direccion_servidor = server_address();
    let conexion = match TcpStream::connect(&direccion_servidor) {
	Ok(stream) => stream,
	Err(_) => {
	    eprintln!("No se pudo conectar a un servidor en {}.",
		      direccion_servidor);
	    return;
	}
    };
    println!("Conectado al servidor en {}.", direccion_servidor);
    println!("¿Cuál es tu nombre?");
    loop {
	match identificacion(&conexion, &entrada_estandar) {
	    Err(e) => {
		println!("{}", e);
		if e != "Ese nombre ya está siendo utilizado." {
		    return;
		}
	    },
	    Ok(nombre) => {
		println!("¡Hola, {}!", nombre);
		break;
	    }
	}
    }
}

fn identificacion(mut conexion: &TcpStream, entrada: &Stdin) -> Result<String, String> {
    let mut line = String::new();
    entrada.lock().read_line(&mut line).unwrap();
    let line = line.trim().to_string();
    if line.is_empty() {
	return Err("No se puede usar un nombre vacío.".to_string())
    }
    if let Err(_) = conexion.write(&identify(&line).as_bytes()) {
	return Err("Ocurrió un problema al escribirle al servidor.".to_string());
    }
    let mut buffer = [0u8; 1024];
    let n = match conexion.read(&mut buffer) {
	Ok(a) => a,
	Err(_) => return Err("Ocurrió un problema al recibir un mensaje del servidor.".to_string()),
    };
    match parsea_mensaje_servidor(String::from_utf8_lossy(&buffer[..n])
				  .to_string()) {
	Ok(Response { result: b, extra: Some(c),.. }) => {
	    if b == "USER_ALREADY_EXISTS" {
		return Err("Ese nombre ya está siendo utilizado.".to_string());
	    } else if b == "SUCCESS" {
		return Ok(c);
	    } else {
		return Err("El mensaje recibido del servidor no fue válido".to_string());
	    }
	},
	_ => return Err("El mensaje recibido del servidor no fue válido".to_string()),
    }
}
