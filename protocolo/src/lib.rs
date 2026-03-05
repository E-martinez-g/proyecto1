use serde::Serialize;

/**
 * Enumeración para el estado de los usuarios.
 */
#[derive(Serialize)]
pub enum EstadoUsuario {
    ACTIVE,
    AWAY,
    BUSY,
}


/**
 * Enumeración para el "type" de los mensajes que
 * envía el cliente.
 */
pub enum ClientType {
    Identify,
    Status,
    Users,
    Text,
    PublicText,
    NewRoom,
    Invite,
    JoinRoom,
    RoomUsers,
    RoomText,
    LeaveRoom,
    Disconnect,
}

/**
 * Enumeración para el "type" de los mensajes que
 * envía el servidor.
 */
pub enum ServerType {
    Response,
    ResponseExtra,
    NewUser,
    NewStatus,
    UserList,
    TextFrom,
    PublicTextFrom,
    JoinedRoom,
    RoomUserList,
    RoomTextFrom,
    LeftRoom,
    Disconnected,
}

/**
 * Módulo para funciones que crean de forma 
 * sencilla los mensajes enviados por los clientes
 * en formato JSON.
 */
pub mod mensajes_cliente;

/**
 * Módulo para funciones que crean de forma
 * sencilla los mensajes enviados por el servidor
 * servidor en formato JSON.
 */
pub mod mensajes_servidor;
