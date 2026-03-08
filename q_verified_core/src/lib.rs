#[allow(unused_imports)]
use std::ffi::{CString, CStr};
use std::os::raw::{c_char, c_uint};
use std::ffi::c_void;

pub mod domain;
pub mod network;
pub mod simulation;

use simulation::Simulacion;
use network::mensaje_red::MensajeRed;
use domain::alert::{Alerta, AlertaFirmada};
use domain::identity::Identidad;

#[no_mangle]
pub extern "C" fn crear_simulacion(cantidad: u32) -> *mut Simulacion {
    let sim = Simulacion::nueva(cantidad as usize);
    Box::into_raw(Box::new(sim))
}

#[no_mangle]
pub extern "C" fn liberar_simulacion(sim: *mut Simulacion) {
    if !sim.is_null() {
        unsafe { let _ = Box::from_raw(sim); }
    }
}

#[no_mangle]
pub extern "C" fn conectar_todos(sim: *mut Simulacion) {
    if sim.is_null() { return; }
    let sim = unsafe { &mut *sim };
    sim.conectar_todos();
}

#[no_mangle]
pub extern "C" fn crear_alerta(
    remitente: *const c_char,
    lat: f64,
    lon: f64,
    timestamp: u64,
) -> *mut MensajeRed {
    let c_str = unsafe { CStr::from_ptr(remitente) };
    let remitente_str = c_str.to_str().unwrap_or("unknown");

    let alerta = Alerta::nueva(remitente_str.to_string(), lat, lon, timestamp);
    let identidad = Identidad::nueva();
    let alerta_firmada = AlertaFirmada::firmar(alerta, identidad.get_signing_key());
    let mensaje = MensajeRed::new(alerta_firmada, 5);

    Box::into_raw(Box::new(mensaje))
}

#[no_mangle]
pub extern "C" fn liberar_mensaje(msg: *mut MensajeRed) {
    if !msg.is_null() {
        unsafe { let _ = Box::from_raw(msg); }
    }
}

#[no_mangle]
pub extern "C" fn propagar_alerta(
    sim: *mut Simulacion,
    msg: *mut MensajeRed,
    nodo_inicial: u32,
    remitente: *const c_char,
) {
    if sim.is_null() || msg.is_null() { return; }

    let sim = unsafe { &mut *sim };
    let msg = unsafe { &mut *msg };

    let c_str = unsafe { CStr::from_ptr(remitente) };
    let remitente_str = c_str.to_str().unwrap_or("unknown");

    sim.propagar_alerta(msg, nodo_inicial as usize, remitente_str);
}


// ---------------------------------------------------------

#[no_mangle]
pub extern "C" fn propagar_alerta_ui_json(sim: *mut Simulacion, nodo_inicial: c_uint) -> *mut c_char {
    if sim.is_null() {
        return std::ptr::null_mut();
    }
    
    let sim = unsafe { &mut *sim };
    
    let pasos = sim.propagar_alerta_para_ui(nodo_inicial as usize);
    
    let json_string = serde_json::to_string(&pasos).unwrap_or_else(|_| "[]".to_string());
    
    let c_string = CString::new(json_string).expect("Fallo al crear CString");
    
    c_string.into_raw()
}

#[no_mangle]
pub extern "C" fn liberar_string(s: *mut c_char) {
    if !s.is_null() {
        unsafe { let _ = CString::from_raw(s); }
    }
}