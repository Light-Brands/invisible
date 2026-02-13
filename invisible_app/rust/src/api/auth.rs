use flutter_rust_bridge::frb;

/// Hash a PIN using Argon2id with secure parameters
#[frb(sync)]
pub fn auth_hash_pin(pin: String) -> Result<String, String> {
    use argon2::{
        password_hash::{PasswordHasher, SaltString},
        Argon2
    };
    use rand::rngs::OsRng;

    let salt = SaltString::generate(&mut OsRng);
    Argon2::default()
        .hash_password(pin.as_bytes(), &salt)
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

    match PasswordHash::new(&stored_hash) {
        Ok(parsed_hash) => {
            Argon2::default()
                .verify_password(pin.as_bytes(), &parsed_hash)
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

    let secret_bytes = match base32::decode(base32::Alphabet::RFC4648 { padding: false }, &secret) {
        Some(bytes) => bytes,
        None => return false,
    };

    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    // Check current and +/- 1 time step (30 second window)
    for offset in [-1, 0, 1] {
        let time = ((timestamp as i64 + offset * 30) / 30) as u64;
        let expected = totp_custom::<Sha1>(30, 6, &secret_bytes, time);
        if code == expected {
            return true;
        }
    }

    false
}
