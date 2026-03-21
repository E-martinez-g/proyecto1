# CHAT SERVER Y CLIENTE
## Utilizando el crate [Tokio](https://tokio.rs/) de Rust 

## Estructura:

- `servidor`: Recibe, maneja y envía mensajes para permitir a los clientes comunicarse entre sí.
- `cliente`: Interfaz de la línea de comandos para facilitar la comunicación con el servidor y otros clientes.
- `protocolo`: Serialización y deserialización de los JSON para comunicarse con el servidor.

## Comandos con Cargo:

### Compilar:

```bash
# Compilar el servidor:
cargo build --bin servidor

# Compilar el cliente:
cargo build --bin cliente

# Compilar ambos:
cargo build
```

### Correr los tests:

```bash
# Tests del cliente:
cargo test --package cliente

# Tests del protocolo:
cargo test --package protocolo

# Ambos: 
cargo test
```

### Ejecución:

```bash
# Ejecutar el servidor en 0.0.0.0:42069:
cargo run --bin servidor
# Ejecutar el servidor en otra dirección:
cargo run --bin servidor <dirección IP> <puerto>

# Ejecutar el cliente en 0.0.0.0:42069:
cargo run --bin cliente
# Ejecutar el cliente en otra dirección:
cargo run --bin cliente <dirección IP> <puerto>
```

## Bitácora del servidor:

La bitácora imprime un mensaje cada vez que recibe o envía un mensaje, o cada vez que ocurre un error.

### Mensajes recibidos:

```bash
# Se imprimen en la salida estándar del servidor.

# Si el usuario no se ha identificado:
** MENSAJE RECIBIDO DE <dirección IP del cliente>: <mensaje recibido>
# Si el usuario ya se identificó:
** MENSAJE RECIBIDO DE <dirección IP del cliente> (<nombre del cliente>): <mensaje recibido>
```

### Mensajes enviados:

```bash
# Se imprimen en la salida estándar.

# Si el usuario no se ha identificado:
** MENSAJE ENVIADO A <direccion IP del cliente>: <mensaje enviado>
# Si el usuario ya se identificó:
** MENSAJE ENVIADO A <direccion IP del cliente> (<nombre del cliente>): <mensaje enviado>
```

### Errores ocurridos:

```bash
# Se imprimen en la salida estándar de errores.

# Si el error es de entrada o salida:
** ERROR:
      <descripción del error>. <Descripción del error de io>
# En cualquier otro caso:
** ERROR:
      <descripción del error>
```

## Comandos del cliente:

- `/invite <cuarto> <usuario> <usuario> ...`: Invita a los usuarios elegidos al cuarto.
  
- `/status <1|Activo|2|Ausente|3|Ocupado>`: Cambia el estado propio al elegido.
  
- `/roommsg <cuarto> <mensaje>`: Envía el mensaje a todos los usuarios en el cuarto elegido.
  
- `/msg <usuario> <mensaje>`: Envía el mensaje de tal manera en que solo el usuario elegido lo pueda ver.
  
- `/roomusers <cuarto>`: Obtiene la lista de miembros del cuarto elegido.
  
- `/leave <cuarto>`: Abandona el cuarto elegido.
  
- `/room <cuarto>`: Crea un cuarto con el nombre elegido.
  
- `/join <cuarto>`: Mete al usuario al cuarto elegido.
  
- `/disconnect`: Desconecta al cliente del servidor.
  
- `/users`: Obtiene la lista de usuarios conectados al servidor.
  
- `/help`: Imprime un mensaje de ayuda con los comandos disponibles.
