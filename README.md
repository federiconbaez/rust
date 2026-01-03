# NexusDB Backend

Backend seguro en Rust para NexusDB Studio, un cliente universal para múltiples bases de datos.

## Características de Seguridad

### Autenticación y Autorización
- **JWT (JSON Web Tokens)**: Autenticación stateless con tokens de 24 horas
- **Argon2**: Hash de passwords con salt aleatorio
- **Middleware de autenticación**: Protección automática de rutas sensibles

### Encriptación
- **AES-256-GCM**: Encriptación de credenciales de base de datos en reposo
- **Nonces aleatorios**: Cada operación de encriptación usa un nonce único
- **Gestión segura de claves**: Claves desde variables de entorno

### Protección contra Abuso
- **Rate Limiting**: 100 requests por minuto por IP
- **Tower Governor**: Implementación eficiente con Rust
- **Validación de queries**: Detección básica de patrones SQL peligrosos
- **Sanitización de identificadores**: Prevención de SQL injection en nombres

### CORS y Headers
- **CORS configurable**: Permitir solo orígenes autorizados
- **Headers de seguridad**: Configuración mediante tower-http
- **Tracing y logging**: Auditoría completa de requests

## Arquitectura

```
backend/
├── src/
│   ├── main.rs              # Punto de entrada y configuración del servidor
│   ├── config.rs            # Gestión de configuración desde .env
│   ├── models.rs            # Modelos de datos y DTOs
│   ├── api/                 # Endpoints REST
│   │   ├── mod.rs
│   │   ├── auth.rs          # Registro, login, obtener usuario
│   │   ├── connections.rs   # CRUD de conexiones DB
│   │   ├── scripts.rs       # CRUD de scripts guardados
│   │   └── health.rs        # Health check
│   ├── db/                  # Capa de datos
│   │   ├── mod.rs           # Pool y migraciones SQLite
│   │   └── repository.rs    # Repositorios para cada entidad
│   └── security/            # Módulos de seguridad
│       ├── mod.rs
│       ├── auth.rs          # JWT y password hashing
│       ├── encryption.rs    # AES-256-GCM
│       ├── rate_limit.rs    # Configuración rate limiting
│       └── validation.rs    # Validación de queries
├── Cargo.toml
├── .env.example
└── .gitignore
```

## Configuración

1. **Copiar variables de entorno**:
```bash
cp .env.example .env
```

2. **Generar clave de encriptación**:
```bash
openssl rand -hex 32
```

3. **Editar `.env`**:
```env
JWT_SECRET=tu-secreto-jwt-muy-largo-y-aleatorio
ENCRYPTION_KEY=clave-hex-64-caracteres-generada-con-openssl
CORS_ORIGIN=http://localhost:3000
```

## Desarrollo

### Requisitos
- Rust 1.75+ (edición 2021)
- SQLite3

### Compilar
```bash
cargo build
```

### Ejecutar tests
```bash
cargo test
```

### Ejecutar servidor (desarrollo)
```bash
cargo run
```

El servidor iniciará en `http://localhost:8080`

### Ejecutar en modo release
```bash
cargo build --release
./target/release/nexusdb-backend
```

## API Endpoints

### Públicos (sin autenticación)

#### Health Check
```http
GET /health
```

#### Registrar usuario
```http
POST /api/auth/register
Content-Type: application/json

{
  "username": "testuser",
  "email": "test@example.com",
  "password": "SecurePassword123!"
}
```

#### Login
```http
POST /api/auth/login
Content-Type: application/json

{
  "username": "testuser",
  "password": "SecurePassword123!"
}
```

Respuesta:
```json
{
  "token": "eyJ...",
  "user": {
    "id": "uuid",
    "username": "testuser",
    "email": "test@example.com"
  }
}
```

### Protegidos (requieren autenticación)

Incluir header: `Authorization: Bearer <token>`

#### Obtener usuario actual
```http
GET /api/auth/me
```

#### Crear conexión
```http
POST /api/connections
Content-Type: application/json

{
  "name": "Mi PostgreSQL",
  "db_type": "postgres",
  "host": "localhost",
  "port": 5432,
  "username": "postgres",
  "password": "mypassword",
  "database_name": "mydb"
}
```

#### Listar conexiones
```http
GET /api/connections
```

#### Obtener conexión
```http
GET /api/connections/:id
```

#### Eliminar conexión
```http
DELETE /api/connections/:id
```

#### Crear script
```http
POST /api/scripts
Content-Type: application/json

{
  "name": "Query de usuarios",
  "query": "SELECT * FROM users LIMIT 10",
  "db_type": "postgres"
}
```

#### Listar scripts
```http
GET /api/scripts
```

#### Eliminar script
```http
DELETE /api/scripts/:id
```

## Base de Datos

El backend usa SQLite para almacenar:
- Usuarios y sus contraseñas hasheadas
- Conexiones a bases de datos (con credenciales encriptadas)
- Scripts SQL guardados
- Historial de ejecuciones (futuro)

Las migraciones se ejecutan automáticamente al iniciar el servidor.

## Próximas Características

- [ ] Ejecución real de queries contra bases de datos configuradas
- [ ] Soporte para MySQL, PostgreSQL, MongoDB, Redis
- [ ] Historial de queries ejecutadas
- [ ] Exportación de resultados (CSV, JSON, Excel)
- [ ] WebSockets para queries de larga duración
- [ ] Refresh tokens
- [ ] Roles y permisos
- [ ] Límites de uso por usuario

## Seguridad en Producción

### Recomendaciones

1. **JWT_SECRET**: Usar secreto de al menos 64 caracteres aleatorios
2. **ENCRYPTION_KEY**: Generar con `openssl rand -hex 32` y nunca commitear
3. **HTTPS**: Usar siempre HTTPS en producción
4. **Firewall**: Limitar acceso al puerto 8080
5. **Reverse Proxy**: Usar Nginx o similar con rate limiting adicional
6. **Backups**: Hacer backup regular de nexusdb.db
7. **Logs**: Monitorear logs para detectar intentos de ataque
8. **Updates**: Mantener dependencias actualizadas

### Variables de Entorno Críticas

NO commitear al repositorio:
- `JWT_SECRET`
- `ENCRYPTION_KEY`
- `.env`

Usar gestor de secretos (Vault, AWS Secrets Manager, etc.) en producción.

## Licencia

MIT
