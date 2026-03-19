use tokio::io::{self, AsyncBufReadExt, AsyncWriteExt, AsyncReadExt, BufReader, Lines, Stdin};
use tokio::net::TcpStream;
use tokio;

use protocolo::{*, Operacion::*, Resultado::*, ServerType, ServerType::*, mensajes_cliente::*};

use util::ErrorCliente::*;

#[tokio::main]
async fn main() {
    let mut entrada_estandar = BufReader::new(io::stdin()).lines();
    
    let direccion_servidor = server_address();
    let mut conexion = match TcpStream::connect(&direccion_servidor).await {
	Ok(stream) => stream,
	Err(e) => {
	    util::error(Conexion{ error: e, direccion: direccion_servidor });
	    return;
	}
    };
    println!("[Sys] ¿Cuál es tu nombre?");
    let mut buffer = [0u8;512];
    loop {
	match identificacion(&mut conexion, &mut entrada_estandar, &mut buffer).await {
	    Err(e) => {
		let mut esfatal = false;
		if !matches!(e, NombreVacio) {
		    esfatal = true;
		}
		util::error(e);
		if esfatal { return; }
	    },
	    Ok(Some(Response{ operation: Identify, result: b, extra: Some(n)})) => {
		if matches!(b, Success) {
		    util::sistema(Response{operation: Identify, result: b, extra: Some(n)});
		    break;
		}
		util::sistema(Response{operation: Identify, result: b, extra: Some(n)});
	    }
	    Ok(None) => {},
	    _ => return,
	}
    }
    loop {
	tokio::select!{
	    linea = entrada_estandar.next_line() => {
		match linea {
		    Err(e) => {
			util::error(EntradaEstandar{ error: Some(e) });
			break;
		    }
		    Ok(None) => {
			util::error(EntradaEstandar{ error: None });
			break;
		    }
		    Ok(Some(entrada)) => {
			match util::maneja_stdin(entrada) {
			    Err(e) => util::error(e),
			    Ok(None) => {},
			    Ok(Some(msg)) => {
				if let Err(e) = util::envia(&mut conexion, msg).await {
				    util::error(e);
				    break;
				}
			    }
			}
		    }
		}
	    }
	    recibido = util::recibe(&mut conexion, &mut buffer) => {
		match recibido {
		    Err(e) => {
			util::error(e);
			break;
		    },
		    Ok(None) => break,
		    Ok(Some(msg)) => {
			match parsea_mensaje_servidor(msg) {
			    Err(_) => {
				util::error(Invalido);
				break;
			    },
			    Ok(Some(st)) => util::sistema(st),
			    Ok(None) => {},
			};
		    },
		}
	    }
	}
    }
}

async fn identificacion(conexion: &mut TcpStream, lineas: &mut Lines<BufReader<Stdin>>,
			buffer: &mut [u8;512]) -> Result<Option<ServerType>, util::ErrorCliente> {
    let line = match lineas.next_line().await {
	Err(e) => return Err(EntradaEstandar{ error: Some(e) }),
	Ok(None) => return Err(EntradaEstandar{ error: None }),
	Ok(Some(l)) => l.trim().to_string(),
    };
    if line.is_empty() {
	return Err(NombreVacio);
    }
    if line.chars().count() > 8 {
	return Err(NombreMuyLargo);
    }
    if let Err(e) = conexion.write(&identify(line).as_bytes()).await {
	return Err(Envio{ error: e });
    }
    buffer.fill(0u8);
    let n = match conexion.read(buffer).await {
	Ok(a) => a,
	Err(e) => return Err(Recepcion{ error: e }),
    };
    let a = String::from_utf8_lossy(&buffer[..n]).to_string();
    let m = parsea_mensaje_servidor(a);
    match m {
	Ok(Some(n @ Response { .. })) =>
	    return Ok(Some(n)),
	Ok(None) => Ok(None),
	_ => return Err(Invalido),
    }
}

pub mod util;
