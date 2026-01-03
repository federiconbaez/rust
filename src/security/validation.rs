use regex::Regex;
use once_cell::sync::Lazy;

static DANGEROUS_SQL_PATTERNS: Lazy<Vec<Regex>> = Lazy::new(|| {
    vec![
        // Comentarios SQL que podrían usarse para bypass
        Regex::new(r"--").unwrap(),
        Regex::new(r"/\*.*\*/").unwrap(),
        // Múltiples statements
        Regex::new(r";\s*(drop|delete|update|insert|create|alter)\s+", ).unwrap(),
        // UNION attacks básicos
        Regex::new(r"\bunion\b.*\bselect\b").unwrap(),
    ]
});

pub fn validate_query(query: &str) -> Result<(), anyhow::Error> {
    let query_lower = query.to_lowercase();

    // Verificar patrones peligrosos
    for pattern in DANGEROUS_SQL_PATTERNS.iter() {
        if pattern.is_match(&query_lower) {
            return Err(anyhow::anyhow!(
                "Query contains potentially dangerous SQL patterns"
            ));
        }
    }

    // Límite de longitud
    if query.len() > 100_000 {
        return Err(anyhow::anyhow!("Query too long (max 100KB)"));
    }

    Ok(())
}

pub fn validate_identifier(identifier: &str) -> Result<(), anyhow::Error> {
    // Solo permitir alfanuméricos, guiones bajos y puntos
    let valid_pattern = Regex::new(r"^[a-zA-Z0-9_\.]+$").unwrap();
    
    if !valid_pattern.is_match(identifier) {
        return Err(anyhow::anyhow!("Invalid identifier format"));
    }

    if identifier.len() > 255 {
        return Err(anyhow::anyhow!("Identifier too long"));
    }

    Ok(())
}

pub fn sanitize_table_name(name: &str) -> Result<String, anyhow::Error> {
    validate_identifier(name)?;
    Ok(name.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_safe_query() {
        assert!(validate_query("SELECT * FROM users WHERE id = 1").is_ok());
    }

    #[test]
    fn test_validate_dangerous_query() {
        assert!(validate_query("SELECT * FROM users; DROP TABLE users;").is_err());
        assert!(validate_query("SELECT * FROM users UNION SELECT password FROM admin").is_err());
    }

    #[test]
    fn test_validate_identifier() {
        assert!(validate_identifier("users").is_ok());
        assert!(validate_identifier("user_table").is_ok());
        assert!(validate_identifier("schema.table").is_ok());
        assert!(validate_identifier("table'; DROP TABLE users--").is_err());
    }
}
