import 'dart:ffi';
import 'dart:convert';
import 'package:ffi/ffi.dart';
import 'rust_bridge.dart';

class Simulacion {
  final Pointer<Void> _sim;

  Simulacion(int cantidad) : _sim = crearSimulacion(cantidad);

  void liberar() {
    liberarSimulacion(_sim);
  }

  List<List<int>> enviarAlerta(int nodoInicial) {
    // Obtenemos el puntero de Rust
    final ptrStr = propagarAlertaJson(_sim, nodoInicial);
    if (ptrStr == nullptr) return [];

    // Lo convertimos a String de Dart
    final jsonStr = ptrStr.toDartString();
    
    // ¡CRUCIAL! Liberamos la memoria en Rust para evitar memory leaks
    liberarString(ptrStr);

    // Decodificamos el JSON
    final List<dynamic> decoded = jsonDecode(jsonStr);
    
    // Lo mapeamos al formato exacto que necesita tu animación en HomeScreen
    return decoded.map((e) => List<int>.from(e)).toList();
  }
}