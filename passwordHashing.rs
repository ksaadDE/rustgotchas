/*
  Example on how to use the password hashing crate with pbkdf2
*/
use argon2::Argon2;
use scrypt::Scrypt;

/// PBKDF2 type for use with [`PasswordHasher`].
 use pbkdf2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHash, PasswordHasher, PasswordVerifier, SaltString, Error
    },
    Pbkdf2
};

struct HashPack();

impl HashPack {

    // TAKE: https://stackoverflow.com/posts/74954705/revisions
    pub fn get_passwordhash_object<'a>(
            bytes_to_hash: &'a [u8],
            salt: &'a SaltString,
    ) -> Result<String, Error> //Result<PasswordHash<'a>, Error>
    {
        let hashed_password = &Pbkdf2.hash_password(&bytes_to_hash, salt);
        match hashed_password
        {
            Ok(e) => Ok(e.to_string()),
            Err(e) => Err(*e)
        }
    }

    pub fn verify_hash (input_password:&str, hash: &str) -> bool
    {
        let password_hash = PasswordHash::new(hash).expect("invalid password hash");

        // Trait objects for algorithms to support
        let algs: &[&dyn PasswordVerifier] = &[&Argon2::default(), &Pbkdf2, &Scrypt];

        return match password_hash.verify_password(algs, input_password)
        {
            Ok(_) => true,
            Err(_) => false
        };
    }
}

fn main () 
{
      let salt = SaltString::generate(&mut OsRng);
      let hash = HashPack::get_passwordhash_object(input.field.as_bytes(), &salt);
  
      match hash
      {
          Ok(h) => {
              if HashPack::verify_hash(input.field, &h)
              {
                  println!("Success 2221");
              }
              println!("Success:{}", h)
          },
          Err(e) => println!("Error: {}", e)
      };
}
