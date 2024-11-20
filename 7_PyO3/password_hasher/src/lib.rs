use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use pyo3::prelude::*;

#[pyfunction]
fn hash_password(password: &str) -> PyResult<String> {
    let salt = SaltString::generate(&mut OsRng);
    // Argon2 з параметрами за замовчуванням (Argon2id v19)
    let argon2 = Argon2::default();
    // Хеш-пароль до рядка PHC ($argon2id$v=19$...)
    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?
        .to_string();
    Ok(password_hash)
}

#[pyfunction]
fn verify_password(password: &str, password_hash: &str) -> PyResult<bool> {
    // Звірити пароль з PHC-рядком.
    let parsed_hash = PasswordHash::new(password_hash)
        .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?;
    let is_valid = Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok();

    Ok(is_valid)
}

/// Python модуль на основі Rust
#[pymodule]
fn password_hasher(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(hash_password, m)?)?;
    m.add_function(wrap_pyfunction!(verify_password, m)?)?;
    Ok(())
}
