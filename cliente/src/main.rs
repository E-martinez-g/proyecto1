use tokio::io::{self, AsyncBufReadExt, AsyncWriteExt, AsyncReadExt, BufReader, Lines, Stdin};
use tokio::net::TcpStream;
use tokio::select;

use protocolo::{*, Operacion::*, Resultado::*, ServerType, ServerType::*, mensajes_cliente::*};

use colored::Colorize;

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
    print!("\r[Sys] ¿Cuál es tu nombre?: ");
    let nombre;
    loop {
	match identificacion(&mut conexion, &mut entrada_estandar).await {
	    Err(e) => {
		let mut esfatal = false;
		if !matches!(e, NombreVacio | NombreOcupado) {
		    esfatal = true;
		}
		util::error(e);
		if esfatal { return; }
	    },
	    Ok(n) => {
		nombre = n;
		println!("[Sys] ¡Hola, {}!", &nombre.bold());
		break;
	    }
	}
    }
    loop {
	tokio::select!({
	    
	});
    }
}

async fn identificacion(conexion: &mut TcpStream,
			lineas: &mut Lines<BufReader<Stdin>>)
			-> Result<ServerType, util::ErrorCliente> {
    let mut line = match lineas.next_line().await {
	Err(e) => return Err(EntradaEstandar{ error: Some(e) }),
	Ok(None) => return Err(EntradaEstandar{ error: None }),
	Ok(Some(l)) => l.trim().to_string(),
    };
    if line.is_empty() {
	return Err(NombreVacio)
    }
    if let Err(e) = conexion.write(&identify(&line).as_bytes()).await {
	return Err(Envio{ error: e });
    }
    let mut buffer = [0u8; 1024];
    let n = match conexion.read(&mut buffer).await {
	Ok(a) => a,
	Err(e) => return Err(Recepcion{ error: e }),
    };
    match parsea_mensaje_servidor(String::from_utf8_lossy(&buffer[..n])
				  .to_string()) {
	Ok(n @ Response { operation: Identify, result: b, extra: Some(c) }) =>
	    return n,
	_ => return Err(Invalido),
    }
}

pub mod util;
