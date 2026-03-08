import 'dart:ffi';
import 'dart:io';
import 'package:ffi/ffi.dart';

final DynamicLibrary rustLib = Platform.isLinux
    ? DynamicLibrary.open('./libq_verified_core.so')
    : throw UnsupportedError('Plataforma no soportada');

typedef CrearSimulacionC = Pointer<Void> Function(Uint32 cantidad);
typedef CrearSimulacionDart = Pointer<Void> Function(int cantidad);
final CrearSimulacionDart crearSimulacion =
    rustLib.lookupFunction<CrearSimulacionC, CrearSimulacionDart>('crear_simulacion');

typedef LiberarSimulacionC = Void Function(Pointer<Void> sim);
typedef LiberarSimulacionDart = void Function(Pointer<Void> sim);
final LiberarSimulacionDart liberarSimulacion =
    rustLib.lookupFunction<LiberarSimulacionC, LiberarSimulacionDart>('liberar_simulacion');

typedef PropagarAlertaC = Pointer<Uint32> Function(Pointer<Void> sim, Uint32 nodoInicial);
typedef PropagarAlertaDart = Pointer<Uint32> Function(Pointer<Void> sim, int nodoInicial);
final PropagarAlertaDart propagarAlerta =
    rustLib.lookupFunction<PropagarAlertaC, PropagarAlertaDart>('propagar_alerta_para_ui');


typedef PropagarAlertaJsonC = Pointer<Utf8> Function(Pointer<Void> sim, Uint32 nodoInicial);
typedef PropagarAlertaJsonDart = Pointer<Utf8> Function(Pointer<Void> sim, int nodoInicial);
final PropagarAlertaJsonDart propagarAlertaJson =
    rustLib.lookupFunction<PropagarAlertaJsonC, PropagarAlertaJsonDart>('propagar_alerta_ui_json');

typedef LiberarStringC = Void Function(Pointer<Utf8> s);
typedef LiberarStringDart = void Function(Pointer<Utf8> s);
final LiberarStringDart liberarString =
    rustLib.lookupFunction<LiberarStringC, LiberarStringDart>('liberar_string');