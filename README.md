# Rust File Organizer

Organizador de archivos por extensión, desarrollado con Rust y su biblioteca estándar.

Agrupa archivos de una carpeta en categorías; permite simular movimientos antes de aplicarlos.

## Funciones actuales

- Categorías Images, Documents, Audio, Video, Archives, Code y Other.
- Simulación con `--dry-run` y filtro por categoría con `--only`.
- Omisión de nombres que comienzan con punto, salvo `--include-hidden`.
- Nombres numerados si el destino ya existe.
- Resumen por categoría, archivos omitidos y tamaño total.

## Requisitos

Rust estable y Cargo, con acceso de lectura y escritura a la carpeta elegida. Los ejemplos utilizan una terminal tipo Bash; en Windows reemplaza la ruta por una carpeta local.

## Instalación y simulación

```bash
git clone https://github.com/DonMilo-22/Rust-File-Organizer.git
cd Rust-File-Organizer
cargo run -- ~/Downloads --dry-run
```

Después de revisar la salida, aplica los movimientos:

```bash
cargo run -- ~/Downloads
```

## Opciones

| Opción | Uso |
|---|---|
| `<carpeta>` | Carpeta cuyos archivos inmediatos se organizarán. |
| `--dry-run` | Imprime movimientos sin crear carpetas ni mover archivos. |
| `--only <categoría>` | Procesa solo esa categoría; no distingue mayúsculas. |
| `--include-hidden` | Incluye archivos cuyos nombres comienzan con punto. |

```bash
cargo run -- ~/Downloads --only Documents --dry-run
```

## Ejemplo antes y después

Ejemplo ilustrativo de una organización completa:

| Antes | Después |
|---|---|
| `Downloads/foto.jpg` | `Downloads/Images/foto.jpg` |
| `Downloads/apuntes.pdf` | `Downloads/Documents/apuntes.pdf` |
| `Downloads/programa.py` | `Downloads/Code/programa.py` |
| `Downloads/archivo.xyz` | `Downloads/Other/archivo.xyz` |

Si `Images/foto.jpg` ya existe, el nuevo destino será `Images/foto_1.jpg` o el siguiente número disponible.

## Almacenamiento y límites

Modifica la carpeta indicada; no crea una base de datos. No recorre subcarpetas ni ofrece deshacer. Las categorías se determinan por extensión, no por contenido. La detección de archivos ocultos se basa en nombres con punto, no en atributos de Windows. El tamaño se imprime como `MB` con un divisor binario. La simulación no reserva destinos ni comprueba que todos los movimientos posteriores puedan completarse.

## Comprobaciones

`cargo build` comprueba la compilación. Prueba `--dry-run` sobre una carpeta de ejemplo antes de usarlo con tus archivos.

## Autor y licencia

Desarrollado por [DonMilo-22](https://github.com/DonMilo-22).

Licencia: MIT.

## Actualización 2026-10-12

- Added `--list-categories` to print all supported organizer categories and exit.

## Historial

Consulta [CHANGELOG.md](CHANGELOG.md) para ver las actualizaciones.
