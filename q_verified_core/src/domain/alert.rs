use serde::{Serialize, Deserialize};
use sha2::{Sha256, Digest};
use hex;
use ed25519_dalek::{SigningKey, VerifyingKey, Signature, Signer, Verifier};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Alerta {
    pub remitente_id: String,
    pub latitud: f64,
    pub longitud: f64,
    pub timestamp: u64,
}
#[allow(dead_code)]
pub enum NivelAlerta {
    Critico,
    Alto,
    Medio,
}

impl Alerta {

    pub fn nueva(
        remitente_id: String,
        latitud: f64,
        longitud: f64,
        timestamp: u64,
    ) -> Self {

        Self {
            remitente_id,
            latitud,
            longitud,
            timestamp,
        }

    }

    pub fn to_bytes(&self) -> Vec<u8> {
        serde_json::to_vec(self).expect("Error serializando alerta")
    }

    pub fn hash(&self) -> Vec<u8> {
        let mut hasher = Sha256::new();
        hasher.update(self.to_bytes());
        hasher.finalize().to_vec()
    }

    pub fn id_evento(&self) -> String {
        hex::encode(self.hash())
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AlertaFirmada {
    pub alerta: Alerta,
    pub firma: String,
}

impl AlertaFirmada {

    pub fn firmar(alerta: Alerta, signing_key: &SigningKey) -> Self {
        let hash = alerta.hash();
        let signature: Signature = signing_key.sign(&hash);

        AlertaFirmada {
            alerta,
            firma: hex::encode(signature.to_bytes()),
        }
    }

    pub fn verificar(&self, verifying_key: &VerifyingKey) -> bool {
        let hash = self.alerta.hash();

        let firma_bytes = match hex::decode(&self.firma) {
            Ok(bytes) => bytes,
            Err(_) => return false,
        };

        let signature = match Signature::from_slice(&firma_bytes) {
            Ok(sig) => sig,
            Err(_) => return false,
        };

        verifying_key.verify(&hash, &signature).is_ok()
    }
}
#[allow(dead_code)]
impl NivelAlerta {

    pub fn ttl_inicial(&self) -> u8 {
        match self {
            NivelAlerta::Critico => 6,
            NivelAlerta::Alto => 4,
            NivelAlerta::Medio => 2,
        }
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::rngs::OsRng;
    use ed25519_dalek::SigningKey;

    #[test]
    fn flujo_completo_firma_y_verificacion() {

       
        let mut csprng = OsRng;
        let signing_key = SigningKey::generate(&mut csprng);
        let verifying_key = signing_key.verifying_key();

        
        let alerta = Alerta {
            remitente_id: "user123".to_string(),
            latitud: 19.4326,
            longitud: -99.1332,
            timestamp: 1710000000,
        };

        
        let alerta_firmada = AlertaFirmada::firmar(alerta.clone(), &signing_key);

        
        assert!(alerta_firmada.verificar(&verifying_key));
    }

    #[test]
    fn falla_si_se_modifica_alerta() {

        let mut csprng = OsRng;
        let signing_key = SigningKey::generate(&mut csprng);
        let verifying_key = signing_key.verifying_key();

        let mut alerta = Alerta {
            remitente_id: "user123".to_string(),
            latitud: 19.4326,
            longitud: -99.1332,
            timestamp: 1710000000,
        };

        let alerta_firmada = AlertaFirmada::firmar(alerta.clone(), &signing_key);

        
        alerta.latitud = 20.0;

        let alerta_modificada = AlertaFirmada {
            alerta,
            firma: alerta_firmada.firma.clone(),
        };

        assert!(!alerta_modificada.verificar(&verifying_key));
    }

    #[cfg(test)]
mod tests {
    use super::*;
    use ed25519_dalek::SigningKey;
    use rand::rngs::OsRng;

    #[test]
    fn firma_falla_si_alerta_es_modificada() {
        let signing_key = SigningKey::generate(&mut OsRng);
        let verifying_key = signing_key.verifying_key();

        let alerta = Alerta {
            remitente_id: "entidad_1".to_string(),
            latitud: 19.4326,
            longitud: -99.1332,
            timestamp: 123456789,
        };

        let mut alerta_firmada = AlertaFirmada::firmar(alerta, &signing_key);

        alerta_firmada.alerta.latitud = 0.0;

       
        assert!(!alerta_firmada.verificar(&verifying_key));
    }
}
}