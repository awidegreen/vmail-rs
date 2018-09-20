use rand::{OsRng, RngCore};
use sha2::{Digest, Sha512};
use std::io;

pub enum PasswordScheme {
    SHA512_CRYPT,
}

pub fn hash(scheme: PasswordScheme, value: String) -> io::Result<String> {
    let mut hasher = match scheme {
        SHA512_CRYPT => Sha512::default(),
    };

    let mut rng = OsRng::new()?;
    let mut salt = [0u8; 16];
    rng.try_fill_bytes(&mut salt)?;

    hasher.input(value.as_bytes());

    let out = format!("{:?}", hasher.result().as_slice());
    //let out = format!("out: {:?}", out);

    let out = format!("{{SHA512_CRYPT}}{}", out);

    Ok(out)
}

#[cfg(test)]
mod test {
    use super::{hash, PasswordScheme};

    #[test]
    fn test_sha512() {
        // {SHA512-CRYPT}$6$KSlmgPC2Vch4xRCg$y05xTawj81BeVY0a8P00qxVwBP9kCAxaXiyGg02Aj2fdgQS24dhCT07Ud0/z5Aotzqodc6hzBUJkOIskvqHFr1
        println!(
            "{}",
            hash(PasswordScheme::SHA512_CRYPT, "foobar".to_string()).unwrap()
        );
        //assert_eq!(
        //None,
        //hash(PasswordScheme::SHA512_CRYPT, "foobar".to_string())
        //);
    }
}
