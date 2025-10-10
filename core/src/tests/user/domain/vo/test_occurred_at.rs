#[cfg(test)]
mod tests {
    use crate::user::domain::vo::OccurredAt;
    use chrono::{Utc, TimeZone};
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test_occurred_at_creation_and_parsing() {
        // ✅ Caso 1: creación con now()
        let occ_now_1 = OccurredAt::now();
        thread::sleep(Duration::from_millis(10)); // pequeña espera para garantizar diferencia
        let occ_now_2 = OccurredAt::now();

        assert_ne!(
            occ_now_1.value(),
            occ_now_2.value(),
            "Cada instancia de OccurredAt::now() debe tener una marca de tiempo distinta"
        );

        println!("✅ now() generado: {}", occ_now_1.value());

        // ✅ Caso 2: creación desde DateTime explícito
        let dt = Utc.with_ymd_and_hms(2024, 5, 10, 12, 30, 0).unwrap();
        let occ_dt = OccurredAt::from_datetime(dt);
        assert_eq!(occ_dt.value(), dt);
        println!("✅ from_datetime() correcto: {}", occ_dt.value());

        // ✅ Caso 3: parseo correcto desde &str ISO 8601 UTC
        let valid_str = "2024-05-10T12:30:00Z";
        let parsed = OccurredAt::try_from(valid_str);
        assert!(parsed.is_ok(), "El valor '{}' debería ser parseable", valid_str);

        if let Ok(ok_val) = parsed {
            println!("✅ '{}' → parseado correctamente: {}", valid_str, ok_val.value());
        }

        // ❌ Caso 4: string vacío
        let empty_str = "";
        let parsed = OccurredAt::try_from(empty_str);
        assert!(parsed.is_err(), "Un string vacío debe causar error");
        if let Err(err) = parsed {
            println!("✅ '{}' → error esperado: {}", empty_str, err);
        }

        // ❌ Caso 5: formato inválido (no ISO ni UTC)
        let invalid_str = "2024/05/10 12:30";
        let parsed = OccurredAt::try_from(invalid_str);
        assert!(parsed.is_err(), "El valor '{}' debería fallar al parsear", invalid_str);
        if let Err(err) = parsed {
            println!("✅ '{}' → error esperado: {}", invalid_str, err);
        }

        // ❌ Caso 6: fecha sin zona horaria (no UTC)
        let no_tz = "2024-05-10T12:30:00";
        let parsed = OccurredAt::try_from(no_tz);
        assert!(parsed.is_err(), "El valor '{}' debe fallar por falta de zona UTC", no_tz);
        if let Err(err) = parsed {
            println!("✅ '{}' → error esperado: {}", no_tz, err);
        }
    }
}
