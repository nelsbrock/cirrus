pub fn hash_password(password: &[u8]) -> anyhow::Result<String> {
    use scrypt::{
        password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
        Scrypt,
    };

    let password_salt = SaltString::generate(&mut OsRng);
    Ok(Scrypt.hash_password(password, &password_salt)?.to_string())
}
