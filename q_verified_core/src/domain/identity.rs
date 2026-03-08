use ed25519_dalek::{SigningKey, VerifyingKey, Signature, Signer};
use rand::rngs::OsRng;
use sha2::{Sha256, Digest};

use crate::domain::alert::{Alerta, AlertaFirmada};
#[allow(dead_code)]
pub struct Identidad {
    signing_key: SigningKey,
    pub verifying_key: VerifyingKey,
    #[allow(dead_code)]
    pub fingerprint: String,
}
#[allow(dead_code)]
impl Identidad {

    pub fn nueva() -> Self {
        let mut csprng = OsRng;
        let signing_key = SigningKey::generate(&mut csprng);
        let verifying_key = signing_key.verifying_key();

        let fingerprint = Self::generar_fingerprint(&verifying_key);

        Self {
            signing_key,
            verifying_key,
            fingerprint,
        }
    }

    fn generar_fingerprint(verifying_key: &VerifyingKey) -> String {
        let mut hasher = Sha256::new();
        hasher.update(verifying_key.to_bytes());
        let result = hasher.finalize();
        hex::encode(result)
    }

    pub fn firmar(&self, mensaje: &[u8]) -> Signature {
        self.signing_key.sign(mensaje)
    }

    pub fn firmar_alerta(&self, alerta: Alerta) -> AlertaFirmada {
        AlertaFirmada::firmar(alerta, &self.signing_key)
    }

    pub fn get_signing_key(&self) -> &SigningKey {
        &self.signing_key
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn firma_y_verificacion_funcionan() {
        let identidad = Identidad::nueva();
        let mensaje = b"alerta critica";

        let firma = identidad.firmar(mensaje);

        let es_valida = Identidad::verificar(
            &identidad.verifying_key,
            mensaje,
            &firma
        );

        assert!(es_valida);
    }
}