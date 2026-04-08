use aes_gcm::{
    Aes256Gcm, Key, Nonce,
    aead::{Aead, KeyInit},
};
use argon2::{Algorithm, Argon2, Params, Version};
use rand::{RngCore, rngs::OsRng};
use std::{fs, io, path::Path};
use zeroize::Zeroizing;

const SALT_LEN: usize = 16;
const NONCE_LEN: usize = 12;
const MIN_DATA_LEN: usize = SALT_LEN + NONCE_LEN + 16;

const ARGON2_M_COST: u32 = 19456;
const ARGON2_T_COST: u32 = 2;
const ARGON2_P_COST: u32 = 1;

#[derive(Debug)]
pub enum EncryptionError {
    Io(io::Error),
    DecryptFailed,
    CorruptedFile,
}

impl std::fmt::Display for EncryptionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EncryptionError::Io(e) => write!(f, "I/O error: {e}"),
            EncryptionError::DecryptFailed => {
                write!(f, "decryption failed, wrong password or corrupted data")
            }
            EncryptionError::CorruptedFile => {
                write!(f, "encrypted file is too short or corrupted")
            }
        }
    }
}

impl std::error::Error for EncryptionError {}

impl From<io::Error> for EncryptionError {
    fn from(e: io::Error) -> Self {
        EncryptionError::Io(e)
    }
}

fn argon2() -> Argon2<'static> {
    let params = Params::new(ARGON2_M_COST, ARGON2_T_COST, ARGON2_P_COST, Some(32))
        .expect("invalid argon2 params");
    Argon2::new(Algorithm::Argon2id, Version::V0x13, params)
}
fn derive_key(password: &[u8], salt: &[u8; SALT_LEN]) -> Zeroizing<[u8; 32]> {
    let mut key = Zeroizing::new([0u8; 32]);
    argon2()
        .hash_password_into(password, salt, key.as_mut())
        .expect("argon2 key derivation failed");
    key
}

pub fn encrypt(plaintext: &[u8], password: &[u8]) -> Vec<u8> {
    let mut salt = [0u8; SALT_LEN];
    let mut nonce_bytes = [0u8; NONCE_LEN];
    OsRng.fill_bytes(&mut salt);
    OsRng.fill_bytes(&mut nonce_bytes);

    let key = derive_key(password, &salt);
    let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(key.as_ref()));
    let ciphertext = cipher
        .encrypt(Nonce::from_slice(&nonce_bytes), plaintext)
        .expect("aes-gcm encryption failed");

    let mut out = Vec::with_capacity(SALT_LEN + NONCE_LEN + ciphertext.len());
    out.extend_from_slice(&salt);
    out.extend_from_slice(&nonce_bytes);
    out.extend_from_slice(&ciphertext);
    out
}

pub fn decrypt(data: &[u8], password: &[u8]) -> Result<Vec<u8>, EncryptionError> {
    if data.len() < MIN_DATA_LEN {
        return Err(EncryptionError::CorruptedFile);
    }

    let salt: [u8; SALT_LEN] = data[..SALT_LEN].try_into().unwrap();
    let nonce_bytes: [u8; NONCE_LEN] = data[SALT_LEN..SALT_LEN + NONCE_LEN].try_into().unwrap();
    let ciphertext = &data[SALT_LEN + NONCE_LEN..];

    let key = derive_key(password, &salt);
    let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(key.as_ref()));
    cipher
        .decrypt(Nonce::from_slice(&nonce_bytes), ciphertext)
        .map_err(|_| EncryptionError::DecryptFailed)
}

pub fn prompt_password(prompt: &str) -> Result<Zeroizing<String>, EncryptionError> {
    rpassword::prompt_password(prompt)
        .map(Zeroizing::new)
        .map_err(EncryptionError::Io)
}

pub fn prompt_password_confirmed() -> Result<Zeroizing<String>, EncryptionError> {
    let pw1 = prompt_password("Password: ")?;
    let pw2 = prompt_password("Confirm password: ")?;

    if *pw1 != *pw2 {
        eprintln!("error: passwords do not match");
        std::process::exit(1);
    }

    Ok(pw1)
}

pub fn write_encrypted(
    path: &Path,
    content: &[u8],
    password: &[u8],
) -> Result<(), EncryptionError> {
    let encrypted = encrypt(content, password);
    fs::write(path, encrypted)?;
    Ok(())
}

pub fn read_encrypted(path: &Path, password: &[u8]) -> Result<Vec<u8>, EncryptionError> {
    let data = fs::read(path)?;
    decrypt(&data, password)
}

pub fn secure_delete(path: &Path) -> io::Result<()> {
    if path.exists() {
        let len = fs::metadata(path)?.len() as usize;
        fs::write(path, vec![0u8; len])?;
        fs::remove_file(path)?;
    }
    Ok(())
}
