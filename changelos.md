# Registro de Cambios (Changelog)

Todos los cambios notables en este proyecto seran documentados en este archivo.

## [0.0.1] - 2026-11-11

### Anadido
- Desarrollado y programado por Pirulug (https://pirulug.com | https://github.com/pirulug).
- Version inicial v0.0.1 de QR Generator en Rust.
- Interfaz grafica de escritorio nativa desarrollada con egui y eframe optimizada para fondo blanco limpio.
- Soporte para linea de comandos (CLI) rapida para generacion de archivos PNG desde la terminal.
- 5 pestanas de control: Entrada, Colores y tamano, Logo en QR, Texto en QR y Verificacion: OK.
- Motor de generacion de codigos QR con soporte para correccion de error Baja (L), Media (M), Cuartil (Q) y Alta (H).
- Biblioteca de plantillas integradas con ventanas de configuracion:
  - Enlaces web (URL).
  - Texto libre y notas.
  - Redes Wi-Fi (WLAN) con WPA, WEP o abiertas.
  - Tarjetas de contacto personales y profesionales (vCard).
  - Tweets para X (Twitter).
  - Eventos de calendario.
  - Enlaces de pago PayPal.Me.
  - Transferencias bancarias europeas IBAN / BIC (EPC-QR).
  - Claves de autenticacion en dos pasos TOTP 2FA.
- Integracion completa del icono oficial `img/icon.ico`:
  - Recurso nativo incrustado en el archivo binario ejecutable de Windows (`qr_generator.exe`).
  - Icono de ventana y barra de tareas en Windows mediante eframe / winit.
  - Icono integrado en la barra de herramientas y ventana Acerca de.
  - Plantilla seleccionable de logotipo en la pestana "Logo en QR" y exportacion vectorial SVG.
- Personalizacion de color de fondo y frente con vista previa circular interactiva y codigo hexadecimal.
- Generador de paletas armoniosas aleatorias y funcion de intercambio rapido de colores.
- Modulo de incrustacion de logotipo central con badge protector para asegurar la legibilidad del escaneo:
  - Plantillas predefinidas: Icono de la aplicacion (icon.ico), Corazon con pulso, Estrella, Escudo, Simbolo Wi-Fi, X (Twitter), Avatar de contacto.
  - Cargador de imagenes locales personalizadas (PNG, JPG, WebP).
  - Controles de tamano y D-Pad direccional para posicionamiento fino.
- Renderizado de texto inferior rasterizado con ab_glyph utilizando fuentes nativas del sistema (Segoe UI y Arial), con soporte para negrita, cursiva, subrayado, tamano de fuente, color y auto-escalado horizontal.
- Modulo de verificacion con decodificacion y comprobacion diagnostica del codigo QR.
- Exportacion a formatos: PNG, JPEG, SVG vectorial, PostScript EPS, URI de datos Base64 y fragmentos Markdown.
