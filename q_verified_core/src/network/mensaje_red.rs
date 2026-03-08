use crate::domain::alert::AlertaFirmada;

pub struct MensajeRed {
    pub alerta: AlertaFirmada,
    pub ttl: u8,
}

impl MensajeRed {

    pub fn new(alerta: AlertaFirmada, ttl: u8) -> Self {
        Self { alerta, ttl }
    }

    pub fn decrementar_ttl(&mut self) {
        self.ttl = self.ttl.saturating_sub(1);
    }

    pub fn expirado(&self) -> bool {
        self.ttl == 0
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::alert::{Alerta, AlertaFirmada};

    fn alerta_falsa() -> AlertaFirmada {
        let alerta = Alerta {
            remitente_id: "usuario1".to_string(),
            latitud: 19.43,
            longitud: -99.13,
            timestamp: 1677640000,
        };

        AlertaFirmada {
            alerta,
            firma: "firma_fake".to_string(),
        }
    }

    #[test]
    fn test_ttl_decrementa() {
        let alerta = alerta_falsa();
        let mut mensaje = MensajeRed::new(alerta, 3);

        mensaje.decrementar_ttl();

        assert_eq!(mensaje.ttl, 2);
    }

    #[test]
    fn test_ttl_expira() {
        let alerta = alerta_falsa();
        let mensaje = MensajeRed::new(alerta, 0);

        assert!(mensaje.expirado());
    }
}