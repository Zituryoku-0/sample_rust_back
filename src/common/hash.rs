use argon2::{
    password_hash::{
        rand_core::OsRng, Error, PasswordHash, PasswordHasher, PasswordVerifier, SaltString,
    },
    Argon2,
};

fn hash_password(password: &str) -> Result<String, Error> {
    // ランダムなソルトを生成
    let salt = SaltString::generate(&mut OsRng);

    // Argon2id(default)
    let argon2 = Argon2::default();

    // ハッシュ化してPHC文字列として返す
    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)?
        .to_string();

    Ok(password_hash)
}

pub fn verify_password(password: &str, password_hash: &str) -> Result<bool, Error> {
    // 保存済みハッシュ文字列を解析
    let parsed_hash = PasswordHash::new(password_hash)?;

    // 検証
    Ok(Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok())
}

#[cfg(test)]
mod tests {
    use super::verify_password;

    #[test]
    fn verify_password_matches_known_hash() {
        let hash = "$argon2id$v=19$m=19456,t=2,p=1$t/xTvNhIo+rbXtco5UXvMw$SeIhM8Tr6n7fZLyhbYLb+5+qcEcpG4Wj9345ZinwB2E";

        assert!(verify_password("abcdefgh", hash).unwrap());
        assert!(!verify_password("test-user", hash).unwrap());
    }
}
