use flutter_rust_bridge::frb;

/// Hash a PIN using Argon2id with secure parameters
#[frb(sync)]
pub fn auth_hash_pin(pin: String) -> Result<String, String> {
    use argon2::{
        password_hash::{PasswordHasher, SaltString},
        Algorithm, Argon2, Params, Version
    };
    use rand::rngs::OsRng;
    use zeroize::Zeroizing;

    // Input validation
    if pin.len() < 6 {
        return Err("PIN must be at least 6 characters".to_string());
    }
    if pin.len() > 128 {
        return Err("PIN exceeds maximum length".to_string());
    }

    // Wrap PIN in Zeroizing to securely erase from memory after use
    let pin_zeroizing = Zeroizing::new(pin);

    // Configure Argon2id per specification: 256 MB, 4 iterations, 4 parallelism
    let params = Params::new(
        262_144, // 256 MB = 262,144 KiB
        4,       // 4 iterations (t_cost)
        4,       // 4 parallelism (p_cost)
        None     // default output length (32 bytes)
    ).map_err(|e| format!("Failed to create Argon2 params: {}", e))?;

    let argon2 = Argon2::new(
        Algorithm::Argon2id,
        Version::V0x13,
        params,
    );

    let salt = SaltString::generate(&mut OsRng);
    argon2
        .hash_password(pin_zeroizing.as_bytes(), &salt)
        .map(|hash| hash.to_string())
        .map_err(|e| e.to_string())
}

/// Verify a PIN against stored Argon2 hash
#[frb(sync)]
pub fn auth_verify_pin(pin: String, stored_hash: String) -> bool {
    use argon2::{
        password_hash::{PasswordHash, PasswordVerifier},
        Argon2
    };
    use zeroize::Zeroizing;

    // Wrap PIN in Zeroizing to securely erase from memory after use
    let pin_zeroizing = Zeroizing::new(pin);

    match PasswordHash::new(&stored_hash) {
        Ok(parsed_hash) => {
            Argon2::default()
                .verify_password(pin_zeroizing.as_bytes(), &parsed_hash)
                .is_ok()
        }
        Err(_) => false,
    }
}

/// Generate a new TOTP secret (base32 encoded)
#[frb(sync)]
pub fn auth_generate_2fa_secret() -> String {
    use rand::RngCore;

    let mut secret = vec![0u8; 20];
    rand::thread_rng().fill_bytes(&mut secret);
    base32::encode(base32::Alphabet::RFC4648 { padding: false }, &secret)
}

/// Verify a TOTP code against a secret (30-second window, +/- 1 step)
#[frb(sync)]
pub fn auth_verify_2fa_code(secret: String, code: String) -> bool {
    use totp_lite::{totp_custom, Sha1};
    use subtle::ConstantTimeEq;

    // Validate code format (must be 6 digits)
    if code.len() != 6 || !code.chars().all(|c| c.is_ascii_digit()) {
        return false;
    }

    let secret_bytes = match base32::decode(base32::Alphabet::RFC4648 { padding: false }, &secret) {
        Some(bytes) => bytes,
        None => return false,
    };

    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or(std::time::Duration::from_secs(0))
        .as_secs();

    // Check current and +/- 1 time step (30 second window)
    for offset in [-1, 0, 1] {
        let time = ((timestamp as i64 + offset * 30) / 30) as u64;
        let expected = totp_custom::<Sha1>(30, 6, &secret_bytes, time);
        // Use constant-time comparison to prevent timing attacks
        if code.as_bytes().ct_eq(expected.as_bytes()).into() {
            return true;
        }
    }

    false
}
