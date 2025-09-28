#[cfg(test)]
mod tests_email {
    use crate::user::domain::Email;
    use std::convert::TryFrom;

    #[test]
    fn test_validated_emails() {
        let valid_emails = vec![
            "reportes_financieros_2025@contabilidad.mi-negocio.online",
            "dev-ops-team-alpha@server-01.internal.local",
            "informacion@servidor.area.empresa.com",
            "ana@ejemplo.com",
            "usuario.valido@mi-empresa-.com",
            "_nombre.apellido@ejemplo.com",
            "RePoRtE.AnUaL-2025@InfoGlobal.co",
            "Julio.Silva@tigo.com.co",
            "Julio.Silva.C@tigo.com.co",
            "Conde guru@ejemplo.com.space"
        ];

        for input in valid_emails {
            let result = Email::try_from(input);

            match result {
                Ok(result) => {
                    // Aquí usas Display ({}), no Debug ({:?})
                    println!("Correo válido '{}': éxito -> {}", input, result);
                }
                Err(e) => println!("Correo inválido '{}': error -> {}", input, e),
            }
        }
    }

    #[test]
    fn test_display_trait() {
        let email = Email::try_from("Test@Domain.com").unwrap();
        println!("Display = {}", email);
        assert_eq!(email.to_string(), "test@domain.com");
    }
}
