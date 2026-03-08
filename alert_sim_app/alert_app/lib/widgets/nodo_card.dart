import 'package:flutter/material.dart';
import '../models/nodo.dart';

class NodoCard extends StatelessWidget {
  final Nodo nodo;
  final double size;

  const NodoCard({super.key, required this.nodo, this.size = 30});

  @override
  Widget build(BuildContext context) {
    return Positioned(
      left: nodo.position.dx - size / 2,
      top: nodo.position.dy - size / 2,
      child: AnimatedContainer(
        duration: const Duration(milliseconds: 300),
        width: size,
        height: size,
        decoration: BoxDecoration(
          shape: BoxShape.circle,
          gradient: nodo.alerta
              ? const LinearGradient(
                  colors: [Colors.yellow, Colors.pinkAccent, Colors.purple])
              : const LinearGradient(
                  colors: [Color(0xFF9C4D8C), Color(0xFF7A2C6D)]),
          boxShadow: nodo.alerta
              ? [BoxShadow(color: Colors.yellowAccent, blurRadius: 15, spreadRadius: 2)]
              : [],
        ),
        alignment: Alignment.center,
        child: Text(
          nodo.id,
          style: const TextStyle(color: Colors.white, fontSize: 12),
        ),
      ),
    );
  }
}