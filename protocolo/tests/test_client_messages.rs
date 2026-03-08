use protocolo::mensajes_cliente::*;

#[test]
fn test_disconnect() {
    let i = "{\"type\":\"DISCONNECT\"}\n";
    assert_eq!(i, protocolo::mensajes_cliente::disconnect())
}
