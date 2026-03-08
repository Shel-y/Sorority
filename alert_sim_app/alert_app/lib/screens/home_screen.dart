import 'dart:math';
import 'dart:ui';
//import 'dart:convert';
import 'package:flutter/material.dart';
//import 'package:ffi/ffi.dart';
import 'package:geolocator/geolocator.dart';
import 'dart:io'; 

// Asegúrate de que estas importaciones coincidan con tu proyecto
// import '../models/nodo.dart';
// import '../widgets/nodo_card.dart';
// import '../native/rust_bridge.dart';

class Nodo3D {
  final String id;
  final double x, y, z;
  bool alerta;
  Nodo3D({required this.id, required this.x, required this.y, required this.z, this.alerta = false});
}

class HomeScreen extends StatefulWidget {
  const HomeScreen({super.key});

  @override
  State<HomeScreen> createState() => _HomeScreenState();
}

class _HomeScreenState extends State<HomeScreen> with TickerProviderStateMixin {
  final int cantidadNodos = 30;
  List<Nodo3D> nodos3D = [];
  
  late AnimationController _rotationController;
  late AnimationController _pulseController;
  late AnimationController _lineController;

  bool _isPressed = false;
  
  int? _origenActual;
  int? _destinoActual;

  // late Pointer<Void> _sim; // Descomenta esto cuando conectes Rust

  @override
  void initState() {
    super.initState();
    // _sim = crearSimulacion(cantidadNodos); // Descomenta esto para Rust

    _rotationController = AnimationController(
      vsync: this,
      duration: const Duration(seconds: 25),
    )..repeat();

    _pulseController = AnimationController(
      vsync: this,
      duration: const Duration(seconds: 1),
    )..repeat(reverse: true);

    _lineController = AnimationController(
      vsync: this,
      duration: const Duration(milliseconds: 500),
    );

    _generarEsfera();
  }

  @override
  void dispose() {
    _rotationController.dispose();
    _pulseController.dispose();
    _lineController.dispose();
    // liberarSimulacion(_sim); // Descomenta esto para Rust
    super.dispose();
  }

  void _generarEsfera() {
    const double radius = 140.0;
    nodos3D.clear();
    for (int i = 0; i < cantidadNodos; i++) {
      double phi = acos(1 - 2 * (i + 0.5) / cantidadNodos);
      double theta = pi * (1 + sqrt(5)) * (i + 0.5);

      double x = radius * sin(phi) * cos(theta);
      double y = radius * sin(phi) * sin(theta);
      double z = radius * cos(phi);

      nodos3D.add(Nodo3D(id: 'N$i', x: x, y: y, z: z));
    }
  }

  Future<void> _enviarAlerta() async {
    if (_isPressed) return;
    setState(() => _isPressed = true);

    ScaffoldMessenger.of(context).clearSnackBars();
    ScaffoldMessenger.of(context).showSnackBar(
      SnackBar(
        content: const Text('Obteniendo ubicación y asegurando red...', textAlign: TextAlign.center),
        backgroundColor: Colors.white.withOpacity(0.2),
        behavior: SnackBarBehavior.floating,
        shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(20)),
        margin: const EdgeInsets.only(bottom: 120, left: 40, right: 40),
        duration: const Duration(seconds: 2),
      ),
    );

    try {
      // Coordenadas por defecto (CDMX)
      double lat = 19.4326;
      double lon = -99.1332;

      // Solo intentamos pedir GPS real si estamos en un celular (Android o iOS)
      if (Platform.isAndroid || Platform.isIOS) {
        LocationPermission permission = await Geolocator.checkPermission();
        if (permission == LocationPermission.denied) {
          permission = await Geolocator.requestPermission();
        }
        
        if (permission == LocationPermission.whileInUse || permission == LocationPermission.always) {
          try {
            Position position = await Geolocator.getCurrentPosition(
              desiredAccuracy: LocationAccuracy.high,
              timeLimit: const Duration(seconds: 5), 
            );
            lat = position.latitude;
            lon = position.longitude;
          } catch (e) {
            print("Tiempo de espera agotado para el GPS. Usando ubicación por defecto.");
          }
        } else {
          print("Permisos denegados en el celular. Usando ubicación por defecto.");
        }
      } else {
        // Si estamos en Linux, Mac o Windows (tu Lap), nos saltamos el chequeo para que no se trabe
        print("💻 Ejecutando en computadora. Saltando GPS de hardware para evitar bloqueos.");
      }

      print("📍 Coordenadas listas para Rust: Lat: $lat, Lon: $lon");

      // Animación visual de la propagación
      List<List<int>> pasosSimulados = [[0, 3], [3, 7], [7, 12], [12, 18], [18, 25]];

      for (var paso in pasosSimulados) {
        int origen = paso[0].clamp(0, nodos3D.length - 1);
        int destino = paso[1].clamp(0, nodos3D.length - 1);

        setState(() {
          _origenActual = origen;
          _destinoActual = destino;
          nodos3D[origen].alerta = true;
          nodos3D[destino].alerta = true;
        });

        _lineController.forward(from: 0.0);
        await Future.delayed(const Duration(milliseconds: 600));

        setState(() {
          nodos3D[origen].alerta = false;
          nodos3D[destino].alerta = false;
          _origenActual = null;
          _destinoActual = null;
        });
      }

      if (mounted) {
        ScaffoldMessenger.of(context).showSnackBar(
          SnackBar(
            content: const Text('Alerta distribuida en la red. Ayuda en camino.', textAlign: TextAlign.center),
            backgroundColor: Colors.redAccent.withOpacity(0.9),
            behavior: SnackBarBehavior.floating,
            shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(20)),
            margin: const EdgeInsets.only(bottom: 120, left: 40, right: 40),
          ),
        );
      }
    } catch (e) {
      print("Error procesando la alerta: $e");
    } finally {
      if (mounted) setState(() => _isPressed = false);
    }
  }

  @override
  Widget build(BuildContext context) {
    // Renombrado a screenSize para evitar conflictos con propiedades internas
    final Size screenSize = MediaQuery.of(context).size;

    return Scaffold(
      backgroundColor: const Color(0xFF0A0A10),
      body: Stack(
        children: [
          // Luz de fondo reactiva
          Center(
            child: AnimatedContainer(
              duration: const Duration(milliseconds: 500),
              width: _isPressed ? 350 : 300,
              height: _isPressed ? 350 : 300,
              decoration: BoxDecoration(
                shape: BoxShape.circle,
                gradient: RadialGradient(
                  colors: [
                    _isPressed 
                        ? const Color(0xFFE53935).withOpacity(0.25)
                        : const Color(0xFFE91E63).withOpacity(0.10),
                    Colors.transparent,
                  ],
                ),
              ),
            ),
          ),

          const Positioned(
            top: 70,
            left: 0,
            right: 0,
            child: Center(
              child: Text(
                'RED DE CONFIANZA',
                style: TextStyle(color: Colors.white70, fontSize: 14, letterSpacing: 5.0, fontWeight: FontWeight.w400),
              ),
            ),
          ),

          // Esfera 3D y Líneas de Propagación
          AnimatedBuilder(
            animation: Listenable.merge([_rotationController, _lineController]),
            builder: (context, child) {
              double angle = _rotationController.value * 2 * pi;

              List<Offset> proyecciones2D = [];
              List<double> escalas = [];

              for (var nodo in nodos3D) {
                double rotX = nodo.x * cos(angle) - nodo.z * sin(angle);
                double rotZ = nodo.z * cos(angle) + nodo.x * sin(angle);
                
                double scale = (rotZ + 200) / 400;
                scale = scale.clamp(0.3, 1.5);
                
                proyecciones2D.add(Offset(
                  screenSize.width / 2 + rotX, 
                  screenSize.height / 2 + nodo.y - 50
                ));
                escalas.add(scale);
              }

              return Stack(
                children: [
                  CustomPaint(
                    size: screenSize,
                    painter: RedPainter(
                      proyecciones: proyecciones2D,
                      origen: _origenActual,
                      destino: _destinoActual,
                      progresoLinea: _lineController.value,
                    ),
                  ),
                  ...nodos3D.asMap().entries.map((entry) {
                    int idx = entry.key;
                    Nodo3D nodo3d = entry.value;
                    double scale = escalas[idx];
                    double opacity = scale.clamp(0.1, 1.0);

                    return Positioned(
                      left: proyecciones2D[idx].dx - (15 * scale),
                      top: proyecciones2D[idx].dy - (15 * scale),
                      child: Opacity(
                        opacity: opacity,
                        child: Transform.scale(
                          scale: scale,
                          child: Container(
                            width: 30,
                            height: 30,
                            decoration: BoxDecoration(
                              shape: BoxShape.circle,
                              color: nodo3d.alerta ? Colors.redAccent : Colors.white.withOpacity(0.5),
                              boxShadow: nodo3d.alerta
                                  ? [const BoxShadow(color: Colors.red, blurRadius: 15, spreadRadius: 3)]
                                  : [],
                            ),
                          ),
                        ),
                      ),
                    );
                  }).toList(),
                ],
              );
            },
          ),

          // Barra Glassmorphism y Botón Pulse
          Align(
            alignment: Alignment.bottomCenter,
            child: ClipRRect(
              child: BackdropFilter(
                filter: ImageFilter.blur(sigmaX: 10, sigmaY: 10),
                child: Container(
                  height: 150,
                  width: double.infinity,
                  decoration: BoxDecoration(
                    color: Colors.white.withOpacity(0.03),
                    border: Border(top: BorderSide(color: Colors.white.withOpacity(0.05))),
                  ),
                  child: Center(
                    child: Padding(
                      padding: const EdgeInsets.only(bottom: 20),
                      child: GestureDetector(
                        onTap: _enviarAlerta,
                        child: AnimatedBuilder(
                          animation: _pulseController,
                          builder: (context, child) {
                            double pulseScale = _isPressed ? 0.9 : 1.0 + (_pulseController.value * 0.05);
                            return Transform.scale(
                              scale: pulseScale,
                              child: Container(
                                width: 85,
                                height: 85,
                                decoration: BoxDecoration(
                                  shape: BoxShape.circle,
                                  gradient: const LinearGradient(
                                    colors: [Color(0xFFE53935), Color(0xFFB71C1C)],
                                    begin: Alignment.topLeft,
                                    end: Alignment.bottomRight,
                                  ),
                                  boxShadow: [
                                    BoxShadow(
                                      color: Colors.redAccent.withOpacity(0.4 + (_pulseController.value * 0.2)),
                                      blurRadius: 20 + (_pulseController.value * 10),
                                      spreadRadius: 2 + (_pulseController.value * 5),
                                    )
                                  ],
                                ),
                                child: const Icon(Icons.sos_rounded, color: Colors.white, size: 45),
                              ),
                            );
                          },
                        ),
                      ),
                    ),
                  ),
                ),
              ),
            ),
          ),
        ], // Aquí cerramos los children del Stack principal
      ), // Aquí cerramos el Stack principal
    ); // Aquí cerramos el Scaffold
  }
}

class RedPainter extends CustomPainter {
  final List<Offset> proyecciones;
  final int? origen;
  final int? destino;
  final double progresoLinea;

  RedPainter({
    required this.proyecciones,
    required this.origen,
    required this.destino,
    required this.progresoLinea,
  });

  @override
  void paint(Canvas canvas, Size size) {
    if (origen == null || destino == null) return;

    final p1 = proyecciones[origen!];
    final p2 = proyecciones[destino!];

    final pActual = Offset(
      p1.dx + (p2.dx - p1.dx) * progresoLinea,
      p1.dy + (p2.dy - p1.dy) * progresoLinea,
    );

    final paintGlow = Paint()
      ..color = Colors.redAccent.withOpacity(0.4)
      ..strokeWidth = 6.0
      ..strokeCap = StrokeCap.round
      ..maskFilter = const MaskFilter.blur(BlurStyle.normal, 5.0);

    final paintCore = Paint()
      ..color = Colors.white.withOpacity(0.9)
      ..strokeWidth = 2.0
      ..strokeCap = StrokeCap.round;

    canvas.drawLine(p1, pActual, paintGlow);
    canvas.drawLine(p1, pActual, paintCore);
  }

  @override
  bool shouldRepaint(covariant RedPainter oldDelegate) {
    return oldDelegate.progresoLinea != progresoLinea ||
           oldDelegate.origen != origen ||
           oldDelegate.proyecciones != proyecciones;
  }
}