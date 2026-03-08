import 'package:flutter/material.dart';

class Nodo {
  final String id;
  final Offset position;
  bool alerta;

  Nodo({required this.id, required this.position, this.alerta = false});
}