use bcrypt::{hash, verify, DEFAULT_COST};

pub fn hash_password(password: &str) -> Result<String, bcrypt::BcryptError> {

    // Hash the password using the generated salt
    let hashed_password = hash(password, DEFAULT_COST)?;

    Ok(hashed_password)
}

pub fn verify_password(password: &str, hashed_password: &str) -> bool {
    // Verify the provided password against the stored hashed password
    match verify(password, hashed_password) {
        Ok(result) => result,
        Err(_) => false, // Verification failed
    }
}
