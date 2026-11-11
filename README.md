# Creador de Códigos QR V7.8.1.3 (Windows) - Edición Rust

Aplicación nativa de escritorio en **Rust** diseñada para ser visual y funcionalmente idéntica al software profesional de referencia (*QR Code Creator - Logo-Module II*), **completamente en español** y con diseño sobre fondo blanco limpio.

---

## 📸 Interfaz 100% en Español Fiel a las Capturas

- **Barra de Menú Superior**:
  - `Archivo`: Nuevo (Ctrl+N), Abrir..., Guardar como..., Exportar Código QR >, Configurar página, Imprimir (Ctrl+P), Actualizar (F12), Salir (Ctrl+Q).
  - `Edición`: Copiar texto, Limpiar entrada.
  - `Plantillas QR`: Texto libre, Enlace (URL), Tweet de X (Twitter), Contacto (vCard), Red WiFi (WLAN), Fecha / Evento, Pago PayPal, Datos bancarios IBAN / BIC, Código TOTP (2FA), Programar código QR.
  - `Ver`: Restablecer vista.
  - `Configuración`: Idioma: Español.
  - `Ayuda`: Acerca de Creador de Códigos QR.

- **Barra de Herramientas (Toolbar)**:
  - 🔄 **Actualizar**: Regenera la visualización del código QR en tiempo real.
  - ⚏ **Plantillas ▾**: Menú desplegable completo con todas las plantillas.
  - 📋 **Copiar**: Copia el texto o imagen al portapapeles.
  - 🛡 **Verificación**: Acceso directo al verificador de lectura de código QR.
  - 📄 **Exportar ▾**: Exportación rápida a PNG, JPEG, SVG vectorial, EPS, Base64 o MarkDown.
  - 🧰 **Herramientas ▾**: Generador de paletas aleatorias armoniosas e inversión de colores.
  - ✨ **Lienzos**: Alternar escalas de resolución.

- **Lienzo Blanco Central**:
  - Marco cuadrado blanco puro (`#FFFFFF`) que enmarca el código QR con contraste nítido y óptimo.
  - Tirador visual divisorio `===` justo debajo del lienzo.

---

## 🗂 Las 5 Pestañas de Control (En Español)

1. **`Entrada`**:
   - Área multilínea con texto por defecto *"¡Mi primer código QR!"*.
   - Contador de caracteres en tiempo real (*"{} Caracteres"*).
   - Casilla de verificación *"Usar 'Texto en QR' como entrada."*.
   - Botones de acción en azul clásico:
     - **`Importar`**: Carga texto desde archivos locales (`.txt`, `.csv`).
     - **`Leer en voz alta`**: Simula lectura por voz.
     - **`Vaciar`**: Limpia el campo de entrada inmediatamente.

2. **`Colores y tamaño`**:
   - **Configuración de matriz**:
     - *Corrección de error*: Selector `Bajo (L)`, `Medio (M)`, `Cuartil (Q)`, `Alto (H)`.
     - *Borde*: Configuración de zona de silencio (0 a 8 módulos).
     - *Factor de escala*: Escala de píxeles por módulo (2 a 15).
     - *Tamaño*: Cálculo dinámico de resolución (ej. `350 x 350 px`).
   - **Color del código QR**:
     - Botones `Color Fondo` y `Color Frente` con muestras circulares de color y código hexadecimal (`h00FFFFFF`, `h00000000`).
     - Botón de paleta de colores aleatorios 🎨 y botón de intercambio de colores (⇄).

3. **`Logo en QR`**:
   - Casilla `Activar logo`.
   - Botones `Seleccionar imagen...` y `Quitar imagen`.
   - Plantillas integradas: **Corazón / Pulso** (idéntico al corazón con electrocardiograma), **Estrella**, **Escudo**, **Símbolo Wi-Fi**, **X (Twitter)** y **Avatar de contacto**.
   - Control de tamaño en píxeles al cuadrado (*"{} Píxeles al cuadrado"*).
   - Casillas `Mantener centrado al agrandar` y `Comprobar validez`.
   - Botón `Ajuste automático` (establece nivel de error Alto y tamaño óptimo).
   - **Posición**: D-Pad con flechas direccionales (▲, ◀, ▼, ▶), botón de centrado (`Centrar`) y slider de píxeles por clic.

4. **`Texto en QR`**:
   - Casilla `Activar texto` y campo de texto (*"Texto de ejemplo"*).
   - Selector de tipografía (*Fuente*), selector de mayúsculas/minúsculas (*Campo de texto*, *minúsculas*, *MAYÚSCULAS*).
   - Estilos: *Negrita*, *Cursiva*, *Subrayado*.
   - Selector de color de texto con muestra circular (`h00FF80FF`) y tamaño de fuente.
   - **Posición**: D-Pad direccional completo para ajustar la ubicación del texto.

5. **`Verificación: OK`**:
   - Panel **Contenido leído**: Muestra el contenido decodificado del código QR.
   - Panel **Verificación**:
     - Estado destacado: **`Tipo reconocido: Código QR reconocido`** (en azul).
     - Botones: `Comprobar de nuevo manualmente` y `Abrir procedimiento de prueba extendido`.

---

## 🚀 Ejecución

```powershell
# Ejecutar la aplicación gráfica en español
cargo run

# Modo rápido por terminal (CLI)
cargo run -- "https://rust-lang.org" codigo_qr.png
```
