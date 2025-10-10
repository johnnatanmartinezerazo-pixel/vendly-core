#[cfg(test)]
mod tests {
    use crate::user::domain::vo::RoleName;

    #[test]
    fn test_role_name_creation() {
        let inputs = vec![
            "admin",                // ✅ válido
            "user",                 // ✅ válido
            "super_admin",          // ✅ válido (si el regex lo permite)
            "manager",              // ✅ válido
            "  Editor  ",           // ✅ válido (espacios, mayúscula)
            "guest",                // ✅ válido
            "viewer",               // ✅ válido
            "",                     // ❌ vacío
            "  ",                   // ❌ solo espacios
            "ad",                   // ❌ demasiado corto (<3)
            "a",                    // ❌ demasiado corto (<3)
            "thisisaverylongrolenamethatiswaytoolongtobeacceptedbythesystem", // ❌ demasiado largo (>50)
            "Admin!",               // ❌ formato inválido (carácter especial)
            "user role",            // ❌ formato inválido (espacio interno)
            "root@",                // ❌ formato inválido (carácter no alfanumérico)
        ];

        for input in inputs {
            let result = RoleName::new(input);

            match result {
                Ok(role) => println!("✅ '{input}' → creado como: {}", role),
                Err(err) => println!("❌ '{input}' → error: {}", err),
            }
        }
    }

    #[test]
    fn test_role_name_display_and_as_ref() {
        let role = RoleName::new("Administrator").unwrap();
        assert_eq!(role.as_str(), "administrator");
        assert_eq!(role.as_ref(), "administrator");
        println!("🎭 Display: {}", role);
    }
}
