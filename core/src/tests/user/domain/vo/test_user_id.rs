#[cfg(test)]
mod tests {
    use crate::user::domain::vo::UserId;
    use uuid::Uuid;

    #[test]
    fn test_user_id_creation_and_parsing() {
        // ✅ Caso 1: creación automática
        let user_id = UserId::new();
        println!("✅ UserId::new() generado: {}", user_id.as_uuid());
        assert_ne!(user_id.as_uuid(), Uuid::nil(), "El UUID generado no debe ser nil");

        // ✅ Caso 2: creación desde UUID existente
        let uuid = Uuid::new_v4();
        let from_uuid = UserId::from_uuid(uuid);
        println!("✅ UserId::from_uuid() válido: {}", uuid);
        assert_eq!(from_uuid.as_uuid(), uuid);

        // ✅ Caso 3: conversión desde &str válido
        let valid_uuid_str = "550e8400-e29b-41d4-a716-446655440000";
        let parsed = UserId::try_from(valid_uuid_str);
        assert!(
            parsed.is_ok(),
            "El UUID '{}' debería ser válido",
            valid_uuid_str
        );
        if let Ok(uid) = parsed {
            println!("✅ '{}' → creado como {:?}", valid_uuid_str, uid);
        }

        // ❌ Caso 4: string malformado (no UUID)
        let invalid_uuid_str = "not-a-valid-uuid";
        let parsed = UserId::try_from(invalid_uuid_str);
        assert!(
            parsed.is_err(),
            "El valor '{}' debería ser inválido",
            invalid_uuid_str
        );
        if let Err(err) = parsed {
            println!("✅ '{}' → error esperado: {}", invalid_uuid_str, err);
        }

        // ❌ Caso 5: string vacío
        let empty_uuid = "";
        let parsed = UserId::try_from(empty_uuid);
        assert!(
            parsed.is_err(),
            "Un string vacío no debería producir un UUID válido"
        );
        if let Err(err) = parsed {
            println!("✅ UUID vacío → error esperado: {}", err);
        }

        // ❌ Caso 6: longitud incorrecta (demasiado corto)
        let short_uuid = "550e8400";
        let parsed = UserId::try_from(short_uuid);
        assert!(
            parsed.is_err(),
            "El valor '{}' debería ser inválido por longitud incorrecta",
            short_uuid
        );
        if let Err(err) = parsed {
            println!("✅ '{}' → error esperado: {}", short_uuid, err);
        }
    }
}
