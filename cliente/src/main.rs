use tokio::io::{self, AsyncBufReadExt, AsyncWriteExt, BufReader, Lines, Stdin};
use tokio::net::TcpStream;
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};
use tokio;

use protocolo::{*, Operacion::*, Resultado::*, ServerType, ServerType::*, mensajes_cliente::*};

use util::ErrorCliente::{self, *};

/**
 * Maneja la comunicación con el usuario y con el servidor.
 *
 * # Campos
 *
 * `usuario` - La entrada estándar para recibir solicitudes del usuario. <br>
 * `buffer` - El `Vec<u8>` para recibir información del servidor. <br>
 * `lector` - Un `BufReader` con la `OwnedReadHalf` para recibir información del
 *            servidor. <br>
 * `escritor` - La `OwnedWriteHalf` para mandar información al servidor.
 */
struct Cliente {
    usuario: Lines<BufReader<Stdin>>,
    buffer: Vec<u8>,
    lector: BufReader<OwnedReadHalf>,
    escritor: OwnedWriteHalf,
}

impl Cliente {

    /**
     * Crea un nuevo cliente a partir de la conexión con el servidor.
     *
     * # Argumentos
     *
     * `conexion` - El `TcpStream` para comunicarse con el servidor.
     */
    fn new(conexion: TcpStream) -> Self {
	let (l, e) = conexion.into_split();
	Cliente {
	    usuario: BufReader::new(io::stdin()).lines(),
	    buffer: vec![0u8;512],
	    lector: BufReader::new(l),
	    escritor: e,
	}
    }

    /**
     * Obtiene lo que el usuario ingrese en la entrada estándar.
     */
    async fn usuario_in(&mut self) -> Result<String, ErrorCliente> {
	match self.usuario.next_line().await {
	    Err(e) => return Err(EntradaEstandar{ error: Some(e) }),
	    Ok(None) => return Err(EntradaEstandar{ error: None }),
	    Ok(Some(l)) => return Ok(l.trim().to_string()),
	}
    }

    /**
     * Obtiene lo que el servidor envíe.
     */
    async fn servidor_in(&mut self) -> Result<Option<String>, ErrorCliente> {
	self.buffer.clear();
	let n = match self.lector.read_until(b'\0', &mut self.buffer).await {
	    Ok(0) => return Ok(None),
	    Ok(a) => a,
	    Err(e) => return Err(Recepcion{ error: e }),
	};
	if n > 512 { return Ok(None); }
	let rec = String::from_utf8_lossy(&self.buffer[..n]);
	Ok(Some(rec.trim_end_matches('\0').to_string()))
    }

    /**
     * Envia un mensaje al servidor.
     *
     * # Argumentos
     *
     * `msg` - Un `String` con el mensaje a enviar.
     */
    async fn servidor_out(&mut self, msg: String) -> Result<(), ErrorCliente> {
	if let Err(e) = self.escritor.write(msg.as_bytes()).await {
	    return Err(Envio{ error: e });
	}
	Ok(())
    }
}

#[tokio::main]
async fn main() {
    let conexion = match TcpStream::connect(&server_address()).await {
	Ok(stream) => stream,
	Err(e) => {
	    println!("{}", util::error(Conexion{ error: e,
						 direccion: server_address() }));
	    return;
	}
    };
    let mut cliente = Cliente::new(conexion);
    println!("[Sys] ¿Cuál es tu nombre?");
    loop {
	match identifica(&mut cliente).await {
	    Err(e) => {
		if !matches!(e, NombreVacio | NombreMuyLargo) {
		    println!("{}", util::error(e));
		    return;
		}
		println!("{}", util::error(e));
	    },
	    Ok(Some(Response{ operation: Identify,
			      result: b,
			      extra: Some(n)})) => {
		if matches!(b, Success) {
		    println!("{}", util::sistema(Response{operation: Identify,
							  result: b,
							  extra: Some(n)}));
		    break;
		}
		println!("{}", util::sistema(Response{operation: Identify,
						      result: b,
						      extra: Some(n)}));
	    }
	    Ok(None) => {},
	    _ => return,
	}
    }
    loop {
	cliente.buffer.clear();
	tokio::select!{
	    linea = cliente.usuario.next_line() => {
		match linea {
		    Err(e) => {
			println!("{}",
				 util::error(EntradaEstandar{ error: Some(e) }));
			break;
		    },
		    Ok(None) => {
			println!("{}",
				 util::error(EntradaEstandar{ error: None }));
			break;
		    },
		    Ok(Some(entrada)) => {
			match util::maneja_stdin(entrada.trim().to_string()) {
			    Err(e) => println!("{}", util::error(e)),
			    Ok(None) => {},
			    Ok(Some(msg)) => {
				if let Err(e) = cliente.servidor_out(msg).await {
				    println!("{}", util::error(e));
				    break;
				}
			    }
			}
		    }
		}
	    }
	    recibido = cliente.lector.read_until(b'\0', &mut cliente.buffer) => {
		match recibido {
		    Ok(0) => {},
		    Err(e) => {
			println!("{}", util::error(Recepcion{ error: e }));
			break;
		    },
		    Ok(n) => {
			if n > 512 { break; }
			let rec = String::from_utf8_lossy(&cliente.buffer[..n]);
			match parsea_mensaje_servidor(rec.to_string()) {
			    Err(_) => {
				println!("{}", util::error(Invalido));
				break;
			    }
			    Ok(None) => {},
			    Ok(Some(st)) => println!("{}", util::sistema(st)),
			}
		    }
		}
	    }
	}
    }
}

async fn identifica(cliente: &mut Cliente) -> Result<Option<ServerType>,
						     ErrorCliente> {
    let nombre: String = match cliente.usuario_in().await {
	Err(e) => return Err(e),
	Ok(l) => l,
    };
    if nombre.is_empty() {
	return Err(NombreVacio);
    }
    if nombre.chars().count() > 8 {
	return Err(NombreMuyLargo);
    }
    if let Err(e) = cliente.servidor_out(identify(nombre)).await {
	return Err(e);
    }
    let recibido = match cliente.servidor_in().await {
	Ok(None) => return Ok(None),
	Ok(Some(rec)) => rec,
	Err(e) => return Err(e),
    };
    match parsea_mensaje_servidor(recibido) {
	Ok(Some(n @ Response { .. })) =>
	    return Ok(Some(n)),
	Ok(None) => Ok(None),
	_ => return Err(Invalido),
    }
}

pub mod util;
