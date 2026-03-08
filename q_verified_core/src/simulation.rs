use crate::domain::node::NodoVerificador;
use crate::network::mensaje_red::MensajeRed;
use crate::network::gossip::MotorGossip;

pub struct Simulacion {
    pub nodos: Vec<NodoVerificador>,
}

impl Simulacion {

    pub fn nueva(cantidad: usize) -> Self {

        let mut nodos = Vec::new();

        for _ in 0..cantidad {
            nodos.push(NodoVerificador::new());
        }

        Self { nodos }
    }

    pub fn conectar_todos(&mut self) {

        let total = self.nodos.len();

        for i in 0..total {

            let mut peers = Vec::new();

            for j in 0..total {
                if i != j {
                    peers.push(format!("Nodo{}", j));
                }
            }

            self.nodos[i].peers = peers;
        }
    }

    pub fn propagar_alerta(
        &mut self,
        mensaje: &mut MensajeRed,
        nodo_inicial: usize,
        id_remitente: &str,
    ) {

        let mut cola = vec![nodo_inicial];

        while let Some(idx) = cola.pop() {

            let nodo = &mut self.nodos[idx];

            let peers = MotorGossip::propagar(
                nodo,
                mensaje,
                id_remitente,
            );

            for peer in peers {

                if let Some(num) = peer.strip_prefix("Nodo") {
                    if let Ok(i) = num.parse::<usize>() {
                        cola.push(i);
                    }
                }

            }

        }

    }

    pub fn propagar_alerta_para_ui(&self, nodo_inicial: usize) -> Vec<(usize, usize)> {
        let mut pasos = Vec::new();

        for i in nodo_inicial..self.nodos.len() - 1 {
            pasos.push((i, i + 1));
        }

        pasos
    }
}