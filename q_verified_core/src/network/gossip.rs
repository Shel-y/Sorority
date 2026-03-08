use crate::domain::node::{NodoVerificador, NivelDifusion};
use crate::network::mensaje_red::MensajeRed;

pub struct MotorGossip;

impl MotorGossip {

    pub fn propagar(
        nodo: &mut NodoVerificador,
        mensaje: &mut MensajeRed,
        id_remitente: &str,
    ) -> Vec<String> {

        let nivel = match nodo.procesar_mensaje(mensaje, id_remitente) {
            Some(n) => n,
            None => return vec![],
        };

        let cantidad_peers = match nivel {
            NivelDifusion::Anillo1 => 3,
            NivelDifusion::Anillo2 => 1,
        };

        nodo.seleccionar_peers(cantidad_peers)
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::domain::node::NodoVerificador;

    #[test]
    fn test_gossip_sin_peers() {

        let mut nodo = NodoVerificador::new();

        let mut mensaje = panic!("usar mock de MensajeRed");

        let peers = MotorGossip::propagar(
            &mut nodo,
            &mut mensaje,
            "nodoA"
        );

        assert!(peers.is_empty());
    }
}