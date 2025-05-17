use argon2::{
    Argon2,
    password_hash::{Error, PasswordHash, PasswordHasher, PasswordVerifier, Salt, SaltString},
};
use rand::{TryRngCore, rngs::OsRng};

pub fn hash_password(password: &str) -> String {
    let mut bytes = [0u8; Salt::RECOMMENDED_LENGTH];
    OsRng.try_fill_bytes(&mut bytes).unwrap();
    let salt = SaltString::encode_b64(&bytes).unwrap();

    let argon2 = Argon2::default();
    argon2
        .hash_password(password.as_bytes(), &salt)
        .unwrap()
        .to_string()
}

pub fn verify_password(password_hash_string: &str, input_password: &str) -> Result<(), Error> {
    let parsed_hash = PasswordHash::new(&password_hash_string)?;
    Argon2::default().verify_password(input_password.as_bytes(), &parsed_hash)
}
