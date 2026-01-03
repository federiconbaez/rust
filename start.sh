#!/bin/bash

# Script para iniciar el backend en modo desarrollo

echo "🚀 Iniciando NexusDB Backend..."

# Verificar si existe .env
if [ ! -f .env ]; then
    echo "⚠️  No se encontró archivo .env, copiando desde .env.example..."
    cp .env.example .env
    echo "✅ Archivo .env creado. Por favor, configura tus secretos antes de usar en producción."
fi

# Ejecutar con cargo
echo "📦 Compilando y ejecutando..."
RUST_LOG=info,nexusdb_backend=debug cargo run
