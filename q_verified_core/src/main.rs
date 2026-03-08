use crate::simulation::Simulacion;
use crate::network::mensaje_red::MensajeRed;
use crate::domain::alert::Alerta;
use crate::domain::identity::Identidad;

mod domain;
mod network;
mod simulation;

fn main() {
    let mut red = Simulacion::nueva(5);

    red.conectar_todos();

    println!("Red creada con {} nodos", red.nodos.len());

    let identidad = Identidad::nueva();

    let alerta = Alerta::nueva(
        "Nodo0".to_string(),
        19.4326,
        -99.1332,
        1234567890,
    );

    let alerta_firmada = crate::domain::alert::AlertaFirmada::firmar(alerta, identidad.get_signing_key());
    
    let mut mensaje = MensajeRed::new(alerta_firmada, 5);

    println!("Enviando alerta desde Nodo0");

    red.propagar_alerta(
        &mut mensaje,
        0,
        "Nodo0",
    );

    println!("Simulación terminada");
}