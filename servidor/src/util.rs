use std::net::SocketAddr;

use std::collections::HashSet;

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
 * <br>
 * `ts` - La conexión con el cliente.
 * <br>
 * `nom` - Posiblemente, el nombre con el que se identificó el usuario.
 * <br>
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
 * <br>
 * `ts` - La conexión con el cliente.
 * <br>
 * `nom` - Posiblemente, el nombre con el que se identificó el usuario.
 */
pub async fn recibe(d: &SocketAddr, ts: &mut TcpStream, nom: Option<&String>, buffer: &mut [u8;512])
		    -> Result<Option<String>, ErrorServidor> {
    buffer.fill(0u8);
    let n = match ts.read(buffer).await {
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
    recibido(&rec, d, nom);

    Ok(Some(rec))
}

/**
 * Estructura para los cuartos.
 *
 * # Campos
 *
 * `sender` - El `Sender` del cuarto para enviar mensajes a todos sus miembros.
 * <br>
 * `invitados` - Un `Vec` que contiene los nombres de todos los usuarios que han sido
 *                invitados al cuarto pero no se han unido.
 * <br>
 * `miembros` - Un `Vec` que contiene los nombres de todos los usuarios que se han
 *              unido al cuarto.
 */
pub struct Cuarto {
    invitados: HashSet<String>,
    miembros: HashSet<String>,
}

impl Cuarto {

    /**
     * Crea una instancia de un cuarto.
     */ 
    pub fn new() -> Self {
	Cuarto { invitados: HashSet::new(), miembros: HashSet::new() }
    }

    /**
     * Regresa una referencia al conjunto de miembros del cuarto.
     */
    pub fn miembros(&self) -> &HashSet<String> {
	&self.miembros
    }

    /**
     * Verifica si un usuario es miembro del cuarto.
     *
     * # Argumentos
     *
     * `nom` - El nombre del usuario que se quiere verificar si se encuentra
     *         en el grupo.
     */
    pub fn es_miembro(&self, nom: &String) -> bool {
	self.miembros.contains(nom)
    }

    /**
     * Verifica si un usuario ha sido invitado al cuarto.
     *
     * # Argumentos
     *
     * `nom` - El nombre del usuario que se quiere verificar si ha sido invitado.
     */
    pub fn es_invitado(&self, nom: &String) -> bool {
	self.invitados.contains(nom)
    }

    /**
     * Agrega al conjunto de usuarios invitados a un usuario.
     *
     * # Argumentos
     *
     * `nom` - El nombre del usuario que ha sido invitado.
     */
    pub fn invita(&mut self, nom: String) {
	self.invitados.insert(nom);
    }

    /**
     * Mueve a un usuario del conjunto de usuarios invitados al de miembros del
     * cuarto.
     *
     * # Argumentos
     *
     * `nom` - El nombre del usuario que entró al cuarto.
     */
    pub fn se_unio(&mut self, nom: String) {
	self.invitados.remove(&nom);
	self.miembros.insert(nom);
    }

    /**
     * Retira del conjunto de miembros del cuarto a un usuario.
     *
     * # Argumentos
     *
     * `nom` - El nombre del usuario que salió del cuarto.
     */
    pub fn salio(&mut self, nom: &String) {
	self.miembros.remove(nom);
    }
}
