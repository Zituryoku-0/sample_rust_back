use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2,
};

#[allow(dead_code)]
fn main() -> Result<(), argon2::password_hash::Error> {
    let password = "abcdefgh";
    let salt = SaltString::generate(&mut OsRng);
    let hash = Argon2::default()
        .hash_password(password.as_bytes(), &salt)?
        .to_string();

    println!("hash is : {}", hash);
    Ok(())
}
