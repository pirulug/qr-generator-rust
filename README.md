# QR Generator v0.0.1 (Windows)

Aplicacion nativa de escritorio en Rust disenada para ser visual y funcionalmente fiel al software profesional de referencia, con diseno optimizado para fondo blanco y completamente en espanol.

## Desarrollador

Programado por Pirulug:
- Web: https://pirulug.com
- GitHub: https://github.com/pirulug

## Icono Oficial de la Aplicacion

El proyecto integra el icono oficial `img/icon.ico`:
- Incrustado en el archivo binario de Windows (`qr_generator.exe`) como recurso nativo de ejecutable.
- Configurado como icono de la ventana principal y de la barra de tareas de Windows.
- Disponible en la barra de herramientas y en la ventana informativa "Acerca de".
- Integrado como plantilla predefinida en la pestana "Logo en QR" y exportacion vectorial SVG.

## Icono Oficial de la Aplicacion

El proyecto integra el icono oficial `img/icon.ico`:
- Incrustado en el archivo binario de Windows (`qr_generator.exe`) como recurso nativo de ejecutable.
- Configurado como icono de la ventana principal y de la barra de tareas de Windows.
- Disponible en la barra de herramientas y en la ventana informativa "Acerca de".
- Integrado como plantilla predefinida en la pestana "Logo en QR" y exportacion vectorial SVG.

## Interfaz en Espanol para Fondo Blanco

Barra de Menu Superior:
- `Archivo`: Nuevo (Ctrl+N), Abrir, Guardar como, Exportar Codigo QR, Configurar pagina, Imprimir (Ctrl+P), Actualizar (F12), Salir (Ctrl+Q).
- `Edicion`: Copiar texto, Limpiar entrada.
- `Plantillas QR`: Texto libre, Enlace (URL), Tweet de X (Twitter), Contacto (vCard), Red WiFi (WLAN), Fecha / Evento, Pago PayPal, Datos bancarios IBAN / BIC, Codigo TOTP (2FA), Programar codigo QR.
- `Ver`: Restablecer vista.
- `Configuracion`: Idioma: Espanol.
- `Ayuda`: Acerca de QR Generator.

Barra de Herramientas:
- `Actualizar`: Regenera la visualizacion del codigo QR en tiempo real.
- `Plantillas v`: Menu desplegable completo con todas las plantillas.
- `Copiar`: Copia el texto o imagen al portapapeles.
- `Verificacion`: Acceso directo a la pestana de verificacion.
- `Exportar v`: Exportacion rapida a PNG, JPEG, SVG vectorial, EPS, Base64 o MarkDown.
- `Herramientas v`: Generador de paletas armoniosas e inversion de colores.
- `Lienzos`: Alternar escalas de resolucion.

Lienzo Blanco Central:
- Marco cuadrado blanco puro con bordes suaves que enmarca el codigo QR con contraste optimo.
- Indicador visual divisorio bajo el lienzo.

## Pestanas de Control

1. `Entrada`:
   - Area multilinea con texto por defecto "Mi primer codigo QR".
   - Contador de caracteres en tiempo real.
   - Casilla de verificacion "Usar 'Texto en QR' como entrada".
   - Botones de accion: `Importar texto`, `Vaciar`.

2. `Colores y tamano`:
   - Configuracion de matriz:
     - Correccion de error: Selector Bajo (L), Medio (M), Cuartil (Q), Alto (H).
     - Borde: Configuracion de zona de silencio (0 a 8 modulos).
     - Factor de escala: Escala de pixeles por modulo (2 a 15).
     - Tamano final: Calculo dinamico de resolucion en pixeles.
   - Color del codigo QR:
     - Botones Color Fondo y Color Frente con muestras circulares de color y codigo hexadecimal.
     - Boton de paleta de colores aleatorios y boton de inversion.

3. `Logo en QR`:
   - Casilla Habilitar logo central.
   - Botones Seleccionar imagen y Quitar imagen.
   - Plantillas integradas: Icono de la aplicacion (icon.ico), Corazon / Pulso, Estrella, Escudo, Simbolo Wi-Fi, X (Twitter) y Avatar de contacto.
   - Control de tamano en pixeles al cuadrado.
   - Casillas Mantener centrado al redimensionar y Comprobar validez.
   - Boton Ajuste automatico (nivel de error Alto y tamano optimo).
   - Posicion: Botones direccionales (^, <, v, >), boton Centrar y ajuste de pixeles por paso.

4. `Texto en QR`:
   - Casilla Habilitar texto y campo con "Texto de ejemplo".
   - Selector de tipografia y selector de formato (Original, minusculas, MAYUSCULAS).
   - Estilos: Negrita, Cursiva, Subrayado.
   - Selector de color de texto con muestra circular y tamano de fuente.
   - Posicion: Botones direccionales para ubicar el texto.

5. `Verificacion: OK`:
   - Panel Contenido leido con el texto decodificado.
   - Panel Diagnostico con el estado Codigo QR reconocido y legible.
   - Botones para verificacion manual y prueba extendida.

## Ejecucion

Para iniciar la aplicacion:
```powershell
cargo run
```

Modo rapido por linea de comandos (CLI):
```powershell
cargo run -- "https://rust-lang.org" codigo_qr.png
```
