# 🏆 API de Gestión de Partidos Deportivos

![Rust](https://img.shields.io/badge/Rust-1.70+-black?logo=rust)
![Actix-web](https://img.shields.io/badge/Actix--web-4.0+-red)
![MongoDB](https://img.shields.io/badge/MongoDB-6.0+-green?logo=mongodb)

## 📌 Descripción

API RESTful para gestión de partidos deportivos que permite crear, leer, actualizar y eliminar información sobre partidos deportivos. Desarrollada con tecnologías modernas y robustas:

- **Backend**: Rust + Actix-web para alto rendimiento y seguridad
- **Base de datos**: MongoDB para flexibilidad en el esquema de datos

## 🔧 Requisitos Previos

- Rust 1.70 o superior
- MongoDB 6.0 o superior
- Cargo (incluido con Rust)

## 🚀 Instalación y Configuración

1. **Clonar el repositorio**

```bash
git clone https://github.com/dpatzan2/lab6-web.git
```

2. **Configurar la base de datos**

- Asegúrate de tener MongoDB ejecutándose localmente
- La aplicación se conectará por defecto a `mongodb://localhost:27017`

3. **Instalar dependencias y ejecutar**

```bash
cd mi-primer-api
cargo build
cargo watch -x run  # Para desarrollo con recarga automática
```

## 📡 API Endpoints

### Partidos

| Método | Ruta                              | Descripción                     |
| ------- | --------------------------------- | -------------------------------- |
| GET     | `/api/matches`                  | Obtener todos los partidos       |
| GET     | `/api/matches/{id}`             | Obtener un partido específico   |
| POST    | `/api/matches`                  | Crear un nuevo partido           |
| PUT     | `/api/matches/{id}`             | Actualizar un partido existente  |
| DELETE  | `/api/matches/{id}`             | Eliminar un partido              |
| PATCH   | `/api/matches/{id}/goals`       | Incrementar el contador de goles |
| PATCH   | `/api/matches/{id}/yellowcards` | Registrar una tarjeta amarilla   |
| PATCH   | `/api/matches/{id}/redcards`    | Registrar una tarjeta roja       |
| PATCH   | `/api/matches/{id}/extratime`   | Registrar tiempo extra           |

## 🖼️ Demostración en Frontend

### Interfaz de Listado de Partidos

![Listado de partidos](https://github.com/user-attachments/assets/0074d0c1-300e-4f56-b90f-7043e7a360ea)

*Vista principal que muestra todos los partidos disponibles obtenidos mediante `GET /api/matches`*

### Formulario de Creación

![Crear partido](https://github.com/user-attachments/assets/de71b9af-a10a-431e-9ff0-a93c7539c82b)

*Formulario que envía datos a `POST /api/matches` para crear nuevos partidos*

### Detalle de Partido

![Detalle de partido](https://github.com/user-attachments/assets/325b9c94-6416-4a6b-bfae-b7141a7780ab)

*Vista detallada que consume `GET /api/matches/{id}` para mostrar información específica*

### Edición de Partido

![Editar partido](https://github.com/user-attachments/assets/40b333e3-7774-4295-b1ac-755250e8325b)

*Formulario de edición que utiliza `PUT /api/matches/{id}` para actualizar datos*

# 🏆 API de Gestión de Partidos Deportivos
## 🛠️ Desarrollo
### Estructura del Proyecto

La estructura actual del proyecto está organizada de la siguiente manera:

```
mi-primer-api/
├── src/
│   └── main.rs         # Punto de entrada de la aplicación y lógica principal
├── Cargo.toml          # Archivo de configuración y dependencias de Rust
├── Cargo.lock          # Versiones exactas de dependencias
├── .gitignore          # Archivos y directorios ignorados por Git
└── README.md           # Documentación del proyecto
```

Cada archivo cumple un rol específico:
- `src/main.rs`: Contiene toda la lógica de la API, incluyendo modelos, rutas y controladores
- `Cargo.toml`: Gestiona las dependencias y metadatos del proyecto
- `Cargo.lock`: Asegura builds reproducibles fijando versiones exactas
- `.gitignore`: Configura qué archivos no deben ser versionados
- `README.md`: Proporciona documentación y guías de uso

## 🔌 Colección de Postman

Para facilitar las pruebas de la API, puedes importar nuestra [colección de Postman](https://www.postman.com/collections/mi-primer-api) que incluye todos los endpoints documentados y ejemplos de uso.
