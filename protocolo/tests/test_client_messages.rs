use protocolo::mensajes_cliente::*;

#[test]
fn test_disconnect() {
    let i = "{\"type\":\"DISCONNECT\"}";
    assert_eq!(i, protocolo::mensajes_cliente::disconnect())
}
