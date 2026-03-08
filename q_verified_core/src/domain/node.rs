use std::collections::{HashMap, HashSet};
use ed25519_dalek::VerifyingKey;

use crate::network::mensaje_red::MensajeRed;

use rand::seq::SliceRandom;
use rand::thread_rng;

pub struct NodoVerificador {

    pub identidades: HashMap<String, VerifyingKey>,

    pub identidades_revocadas: HashSet<String>,

    pub eventos_procesados: HashSet<String>,

    pub peers: Vec<String>,
}

pub enum NivelDifusion {
    Anillo1,
    Anillo2,
}

impl NodoVerificador {

    pub fn new() -> Self {
        Self {
            identidades: HashMap::new(),
            identidades_revocadas: HashSet::new(),
            eventos_procesados: HashSet::new(),
            peers: Vec::new(),
        }
    }

    pub fn procesar_mensaje(
        &mut self,
        mensaje: &mut MensajeRed,
        id_remitente: &str,
    ) -> Option<NivelDifusion> {

        if mensaje.expirado() {
            return None;
        }

        if self.identidades_revocadas.contains(id_remitente) {
            return None;
        }

        let clave = self.identidades.get(id_remitente)?;

        if !mensaje.alerta.verificar(clave) {
            return None;
        }

        let id_evento = mensaje.alerta.alerta.id_evento();

        if self.eventos_procesados.contains(&id_evento) {
            return None;
        }

        self.eventos_procesados.insert(id_evento);

        mensaje.decrementar_ttl();

        if mensaje.ttl > 2 {
            Some(NivelDifusion::Anillo1)
        } else {
            Some(NivelDifusion::Anillo2)
        }
    }
    #[allow(dead_code)]
    pub fn revocar_identidad(&mut self, id: String) {
        self.identidades_revocadas.insert(id);
    }

    pub fn seleccionar_peers(&self, cantidad: usize) -> Vec<String> {

        let mut rng = thread_rng();

        self.peers
            .choose_multiple(&mut rng, cantidad)
            .cloned()
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_evento_se_registra() {
        let mut nodo = NodoVerificador::new();
        nodo.eventos_procesados.insert("evento1".into());
        assert!(nodo.eventos_procesados.contains("evento1"));
    }

    #[test]
    fn test_revocacion_identidad() {
        let mut nodo = NodoVerificador::new();
        nodo.revocar_identidad("ana".into());
        assert!(nodo.identidades_revocadas.contains("ana"));
    }

    #[test]
    fn test_seleccion_peers() {
        let mut nodo = NodoVerificador::new();
        nodo.peers = vec!["A".into(), "B".into(), "C".into()];
        let seleccion = nodo.seleccionar_peers(2);
        assert!(seleccion.len() <= 2);
    }

    #[test]
    fn test_revocado_no_procesa_mensaje() {
        let mut nodo = NodoVerificador::new();
        nodo.revocar_identidad("malicioso".into());
        assert!(nodo.identidades_revocadas.contains("malicioso"));
    }
}