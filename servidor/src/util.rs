use std::net::SocketAddr;

use super::bitacora::ErrorServidor::*;
use super::bitacora::*;

use tokio::net::TcpStream;
use tokio::io::{AsyncWriteExt, AsyncReadExt};

/**
 * Envía un mensaje a un cliente y lo registra en la bitácora.
 *
 * # Argumentos
 *
 * `d` - La dirección IP del cliente al que se enviará el mensaje.
 * `ts` - La conexión con el cliente.
 * `nom` - Posiblemente, el nombre con el que se identificó el usuario.
 * `msg` - Un String con el mensaje a enviar al cliente
 */
pub async fn envia(d: &SocketAddr, ts: &mut TcpStream, nom: Option<&String>, msg: String)
		   -> Result<(), ErrorServidor> {
    enviado(&msg, d, nom);
    if let Err(e) = ts.write(msg.as_bytes()).await {
	if let Some(n) = nom {
	    return Err(Envio { error: e, direccion: *d, nombre: Some(n.clone()) });
	}
	return Err(Envio { error: e, direccion: *d, nombre: None });
    }
    Ok(())
}

/**
 * Recibe un mensaje de un cliente y lo registra en la bitácora.
 *
 * # Argumentos
 *
 * `d` - La dirección IP del cliente que envió el mensaje.
 * `ts` - La conexión con el cliente.
 * `nom` - Posiblemente, el nombre con el que se identificó el usuario.
 */
pub async fn recibe(d: &SocketAddr, ts: &mut TcpStream, nom: Option<&String>)
		    -> Result<Option<String>, ErrorServidor> {
    let mut buffer = [0u8; 512];

    let n = match ts.read(&mut buffer).await {
	Ok(0) => return Ok(None),
	Ok(a) => a,
	Err(e) => {
	    if let Some(n) = nom {
		return Err(Recepcion {error: e, direccion: *d, nombre: Some(n.clone())});
	    }
	    return Err(Recepcion {error: e, direccion: *d, nombre: None})
	}
    };

    let rec = String::from_utf8_lossy(&buffer[..n]).to_string();
    recibido(&rec, d, None);

    Ok(Some(rec))
}
