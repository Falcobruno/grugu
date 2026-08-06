# Grugu API

Backend en Rust para una aplicación de comunicación en parejas. Permite registrar usuarios, vincularlos como pareja, registrar estados de ánimo, y recibir sugerencias generadas por IA basadas en esos registros.

## Stack

- **Rust** + **Axum** — framework web
- **Tokio** — runtime asíncrono
- **SQLite** vía **sqlx** — persistencia
- **bcrypt** — hasheo de contraseñas
- **jsonwebtoken** — autenticación JWT
- **tracing** — logging
- **Docker** — containerización

## Features

- Registro y autenticación de usuarios con JWT
- Autorización a nivel de recurso (cada usuario solo puede modificar/eliminar su propio perfil)
- Eliminación lógica (soft delete)
- Vinculación de usuarios como pareja
- Registro de estados de ánimo con respuesta generada por IA (en desarrollo)
- Cambio de contraseña
- Logging estructurado
- Contenedor Docker listo para producción

## Cómo correrlo

### Localmente

```bash
cargo run
```

El servidor levanta en `http://localhost:3000`.

### Con Docker

```bash
docker build -t grugu .
docker run -p 3000:3000 grugu
```

## Endpoints

### Públicos

| Método | Ruta | Descripción |
|--------|------|-------------|
| GET | `/` | Estado de la API |
| POST | `/register` | Registrar un usuario nuevo |
| POST | `/login` | Iniciar sesión, devuelve un token JWT |

### Protegidos (requieren header `Authorization: Bearer <token>`)

| Método | Ruta | Descripción |
|--------|------|-------------|
| GET | `/users` | Listar todos los usuarios activos |
| GET | `/user/{id}` | Obtener un usuario por ID |
| PUT | `/user/{id}` | Actualizar un usuario (solo el dueño) |
| DELETE | `/user/{id}` | Eliminar (soft delete) un usuario (solo el dueño) |
| POST | `/link/{partner_id}` | Vincular al usuario logueado con otro como pareja |
| POST | `/mood` | Registrar un estado de ánimo y recibir respuesta de la IA |
| POST | `/change-password` | Cambiar la contraseña del usuario logueado |

## Estructura del proyecto

```
grugu/
├── Cargo.toml
├── Dockerfile
├── src/
│   ├── main.rs          # Rutas y arranque del servidor
│   ├── db.rs             # Conexión y creación de tablas
│   ├── handlers.rs       # Lógica de cada endpoint
│   └── models/
│       ├── api_response.rs
│       ├── api_status.rs
│       ├── user.rs
│       ├── claims.rs
│       └── mood.rs
```

## Base de datos

SQLite, persistida en disco (no en memoria). Tablas principales:

- **users** — id, name, age, relationship_years, password (hash), active, partner_id
- **mood_entries** — id, user_id, text, ai_response_to_user, ai_suggestion_to_partner, created_at

## Próximos pasos

- Conectar la IA real (Anthropic) para generar respuestas y sugerencias
- Registro de ciclo menstrual
- Recuperación de contraseña
- Frontend
