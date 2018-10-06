use sha_crypt::{errors::CryptError, sha512_simple, Sha512Params};
use std::result::Result;

pub enum PasswordScheme {
    Sha512Crypt,
    // TODO add some more and let the user decide, for now SHA512_CRYPT should
    // be alright
}

pub fn hash(scheme: PasswordScheme, value: String) -> Result<String, CryptError> {
    match scheme {
        PasswordScheme::Sha512Crypt => {
            let params = Sha512Params::default();
            let mut pass = "{SHA512-CRYPT}".to_string();
            pass += &sha512_simple(&value, &params)?;
            Ok(pass)
        }
    }
}
