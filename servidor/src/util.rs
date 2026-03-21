use std::collections::HashSet;

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
