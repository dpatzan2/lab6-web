# 🏆 API de Gestión de Partidos Deportivos

![Rust](https://img.shields.io/badge/Rust-1.70+-black?logo=rust)
![Actix-web](https://img.shields.io/badge/Actix--web-4.0+-red)
![MongoDB](https://img.shields.io/badge/MongoDB-6.0+-green?logo=mongodb)

## 📌 Descripción
API RESTful para gestión de partidos deportivos desarrollada con:
- **Backend**: Rust + Actix-web
- **Base de datos**: MongoDB
- **CORS**: Configuración flexible para desarrollo

## 🚀 Instalación

```bash
# 1. Clonar repositorio
git clone https://github.com/dpatzan2/lab6-web.git

# 2. Entrar al directorio
cd mi-primer-api

# 3. Ejecutar (requiere MongoDB local)
cargo watch -x run
```

## 🖼️ Demostración en Frontend

### Interfaz de Listado de Partidos
![Listado de partidos](https://github.com/user-attachments/assets/0074d0c1-300e-4f56-b90f-7043e7a360ea)
*Vista principal que muestra todos los partidos disponibles obtenidos mediante `GET /api/matches`*

### Formulario de Creación
![Crear partido](./screenshots/create-form.png)  
*Formulario que envía datos a `POST /api/matches` para crear nuevos partidos*

### Detalle de Partido
![Detalle de partido](./screenshots/match-detail.png)  
*Vista detallada que consume `GET /api/matches/{id}` para mostrar información específica*

### Edición de Partido
![Editar partido](./screenshots/edit-form.png)  
*Formulario de edición que utiliza `PUT /api/matches/{id}` para actualizar datos*

