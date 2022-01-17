use sodalite::{box_, box_open, BoxNonce};

use super::identity::{Identity, PrivateIdentity};

#[derive(Debug)]
pub enum EncryptionError {
	BoxingFailed,
	UnboxingFailed,
}

#[allow(dead_code)]
pub fn box_message(plain_msg: &String, sender: &PrivateIdentity, recipient: &Identity) -> Result<String, EncryptionError> {
	// this is a dummy placeholder until we work out how to increment nonce
	let nbox: BoxNonce = [0u8; 24];

	// пока не знаю нужен ли
	// let nonce = base64::encode(nbox.to_vec());

	// initialise ciphertext with a default value 
	let mut cipher_text = [0u8];

	// Encrypt data returning cipher_text
	match box_(&mut cipher_text, plain_msg.as_bytes(), &nbox, &recipient.public_key, &sender.secret_key) {
		Err(_e) => return Err(EncryptionError::BoxingFailed),
		Ok(_s) => ()
	};

	Ok(String::from_utf8(cipher_text.to_vec()).unwrap())
}

#[allow(dead_code)]
pub fn unbox_message(boxed_msg: &String, nonce: &BoxNonce, sender: &Identity, recipient: &PrivateIdentity) -> Result<String, EncryptionError> {
	// initialise ciphertext with a default value 
	let mut cipher_text = [0u8];

	// Decrypt cipher_text
	match box_open(&mut cipher_text, boxed_msg.as_bytes(), &nonce, &sender.public_key, &recipient.secret_key) {
		Err(_e) => return Err(EncryptionError::UnboxingFailed),
		Ok(_s) => ()
	}

	Ok(String::from_utf8(cipher_text.to_vec()).unwrap())
}
