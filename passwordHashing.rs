/*
  Example on how to use the passwordHashing + RustCrypto crate with pbkdf2
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

// abstracted it in HashPack impl for struct
impl HashPack {

    // TAKE: https://stackoverflow.com/posts/74954705/revisions
    pub fn get_passwordhash_object<'a>(
            bytes_to_hash: &'a [u8],
            salt: &'a SaltString,
    ) -> Result<String, Error> //Result<PasswordHash<'a>, Error>
    {
        // obtain the hashed pw and match if it worked, on error return the error, on success return hash as String
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

        // return true or false, if ok or not. 
        return match password_hash.verify_password(algs, input_password)
        {
            Ok(_) => true,
            Err(_) => false
        };
    }
}

fn main () 
{
      let salt = SaltString::generate(&mut OsRng); // Generate a random salt using OS (secure?) Random Number Generator (RNG)
      let hash = HashPack::get_passwordhash_object(input.field.as_bytes(), &salt); // call get_pw_hash_obj here, input is your plaintext pw -> bytes
  
      match hash
      {
          Ok(h) => {
              println!("Success Hashing:{}", h)
              if HashPack::verify_hash(input.field, &h) // call the Hashpack -> verify_hash func input.field is your plaintext pw, h is your hash
              {
                  println!("Success Verify Hash");
              }
          },
          Err(e) => println!("Error: {}", e)
      };
}
