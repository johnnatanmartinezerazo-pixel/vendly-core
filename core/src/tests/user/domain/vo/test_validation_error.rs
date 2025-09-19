#[cfg(test)]
mod tests_validation_error {
    use crate::user::domain::ValidationError;
    use std::error::Error;

    #[test]
    fn test_validation_error_display_messages() {
        let cases = vec![
            (ValidationError::InvalidEmail, "El email proporcionado no es válido o supera los 320 caracteres."),
            (ValidationError::InvalidPhone, "El número de teléfono no es válido. Debe comenzar con '+' y contener solo dígitos, máximo 20 caracteres."),
            (ValidationError::InvalidUsernameLength, "El nombre de usuario no es válido. Debe tener entre 6 y 30 caracteres."),
            (ValidationError::InvalidUsernameFormat, "El nombre de usuario no es válido. Tiene un Formato inválido"),
            (ValidationError::InvalidExternalIdEmpty, "El identificador externo no puede estar vacío."),
            (ValidationError::InvalidExternalIdLength, "El identificador externo excede la longitud máxima permitida (255 caracteres)."),
            (ValidationError::InvalidExternalIdFormat, "El identificador externo contiene caracteres inválidos o de control."),
            (ValidationError::InvalidRole, "El rol proporcionado no es válido."),
            (ValidationError::InvalidUserStatus, "El estado de usuario proporcionado no es válido."),
            (ValidationError::InvalidLocale, "El locale proporcionado no es válido (ej: es-ES)."),
            (ValidationError::InvalidTimezone, "La zona horaria proporcionada no es válida."),
            (ValidationError::InvalidGender, "El género proporcionado no es válido."),
            (ValidationError::InvalidAuthType, "El tipo de autenticación proporcionado no es válido."),
            (ValidationError::InvalidSubscriptionTier, "El nivel de suscripción proporcionado no es válido."),
            (ValidationError::InvalidSubscriptionStatus, "El estado de suscripción proporcionado no es válido."),
            (ValidationError::InvalidConsentType, "El tipo de consentimiento proporcionado no es válido."),
        ];

        for (error, expected_msg) in cases {
            let msg = error.to_string();
            println!("Probando {:?} -> '{}'", error, msg);
            assert_eq!(msg, expected_msg);
        }
    }

    #[test]
    fn test_validation_error_implements_std_error() {
        let err: Box<dyn Error> = Box::new(ValidationError::InvalidEmail);
        println!("Boxed error: {}", err);
        assert_eq!(err.to_string(), "El email proporcionado no es válido o supera los 320 caracteres.");
    }
}
