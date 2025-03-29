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

![Listado de partidos](https://github.com/user-attachments/assets/6edcbaa8-372e-4f18-8c32-32ef815858f7)


*Vista principal que muestra todos los partidos disponibles obtenidos mediante `GET /api/matches`*

### Formulario de Creación

![Crear partido](https://github.com/user-attachments/assets/784f0489-62d9-4a61-a582-72fb47408273)

*Formulario que envía datos a `POST /api/matches` para crear nuevos partidos*

### Detalle de Partido

![Detalle de partido](https://github.com/user-attachments/assets/c46038e9-3d3c-4e15-9589-409f110d83aa)


*Vista detallada que consume `GET /api/matches/{id}` para mostrar información específica*

### Edición de Partido

![Editar partido](https://github.com/user-attachments/assets/8de5c575-20b5-40a5-82a5-c7fb366b95ae)

*Formulario de edición que utiliza `PUT /api/matches/{id}` para actualizar datos*

### Eliminiar partido

![Eliminar partido](https://github.com/user-attachments/assets/c3ed2925-c170-412b-bec5-dae3715cdc68)

*Campo para insertar el id del partido a elimininar*

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

Para facilitar las pruebas de la API, pueden importar nuestra [colección de Postman](https://documenter.getpostman.com/view/19231920/2sB2cPj5aU) que incluye todos los endpoints documentados y ejemplos de uso.
