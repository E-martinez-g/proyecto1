use bitacora::ErrorServidor::*;
use bitacora::*;

use tokio::net::TcpStream;

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
    bitacora::enviado(&msg, d, nom);
    if let Err(e) = ts.write(msg.as_bytes()).await {
	return Envio { error: e, direccion: d, nombre: nom };
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
pub async fn recibe(d: &SocketAddress, ts: &mut TcpStream, nom: Option<&String>)
		    -> Result<Option<String>, ErrorServidor> {
    let mut buffer = [0u8; 512];

    let n = match ts.read(&mut buffer).await {
	Ok(0) => return Ok(None),
	Ok(a) => a,
	Err(e) => return Err(Recepcion {error: e, direccion: *d, nombre: nom}),
    };

    let rec = String::from_utf8_lossy(&buffer[..n]).to_string();
    bitacora::recibido(&rec, d, None);

    Ok(Some(rec))
}
