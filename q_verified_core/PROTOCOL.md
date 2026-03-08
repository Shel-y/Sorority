Red Federada de Apoyo Comunitario

Protocol v0.1 (MVP)

1. Propósito

Este proyecto define un protocolo open source para una red federada de apoyo comunitario orientada a mujeres en situación de riesgo.

El objetivo principal es:

Reducir el tiempo y fricción para activar ayuda en una emergencia crítica, priorizando velocidad sobre complejidad.

El sistema no pretende reemplazar servicios de emergencia oficiales, sino facilitar la activación rápida de redes de apoyo verificables.

2. Principios de Diseño

Velocidad en emergencia es prioridad.

La ubicación exacta es un privilegio, no un derecho.

La identidad es criptográfica y portable.

La confianza es relacional, no global.

Federación en lugar de centralización.

Privacidad por diseño, no por promesa.

Gobernanza comunitaria con revisión humana.

3. Alcance del MVP

El MVP implementa únicamente:

Identidad criptográfica local.

Relaciones de confianza firmadas entre usuarias.

Envío de alerta crítica.

Validación de firma en servidor.

Reenvío federado entre servidores comunitarios.

Diferenciación de visibilidad (exacta vs aproximada).

No incluye:

Sistema automático de reputación.

Geolocalización automática por proximidad.

Niveles múltiples de alerta.

Integración con autoridades.

Blockchain o sistemas distribuidos complejos.

4. Modelo de Identidad

Cada usuaria:

Genera un par de claves criptográficas (ej. Ed25519).

La clave privada nunca abandona el dispositivo.

La clave pública funciona como identificador global.

El ID del nodo deriva de la clave pública (fingerprint).

La identidad no depende del servidor.

5. Modelo de Confianza

La confianza es relacional.

Una relación de confianza consiste en:

Identificador del nodo destino.

Nivel de confianza contextual.

Firma de validación.

Timestamp.

La confianza no es un estado global asignado por el servidor.

6. Modelo de Alerta Crítica

Una alerta crítica incluye:

ID criptográfico del remitente.

Coordenadas.

Timestamp.

ID único de evento.

Firma digital.

Flujo:

La usuaria firma la alerta localmente.

Se envía al servidor comunitario.

El servidor valida la firma.

Se reenvía a:

Red de confianza directa (ubicación exacta).

Red ampliada validada (zona aproximada).

Si los contactos pertenecen a otros servidores, se utiliza federación HTTP.

7. Visibilidad Escalonada

Exacta: Solo red de confianza directa.

Aproximada: Red ampliada validada.

Nunca se expone ubicación exacta a nodos no confiables.

8. Federación

Cada comunidad puede operar su propio servidor.

Los servidores se comunican mediante HTTP firmado.

No existe servidor central obligatorio.

Las identidades pueden migrarse entre servidores.

9. Modelo de Amenaza 

El sistema considera:

Infiltración de nodos.

Alertas falsas.

Servidores maliciosos.

Rastreo por patrón de uso.

Mitigación basada en:

Firmas criptográficas.

Validación previa para acceso ampliado.

Revisión humana para revocaciones.

Minimización de datos almacenados.

10. No-Objetivos

Este protocolo no pretende:

Garantizar presencia física de apoyo.

Sustituir autoridades oficiales.

Ser completamente anónimo sin fricción.

Resolver todos los riesgos sociales mediante tecnología.

Estado del Proyecto

Versión: v0.1
Estado: Diseño inicial de MVP federado.