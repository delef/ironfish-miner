use sodalite;
use base64;

//
// Types and helper functions related to a peer's identity.
//

// A base64-encoded 32-byte public key exposed to other peers on the network.
pub struct Identity {
	pub public_key: sodalite::BoxPublicKey,
}

impl Identity {
	#[allow(dead_code)]
	fn can_init_webrtc(&self, dest: &Identity) -> bool {
		self.public_key.len() > dest.public_key.len()
	}
	
	#[allow(dead_code)]
	fn can_keep_dupticate_conn(&self, dest: &Identity) -> bool {
		self.can_init_webrtc(dest)
	}

	// old: is_identity
	#[allow(dead_code)]
	fn is_valid(&self) -> bool {
		let b64 = base64::encode(&self.public_key);
		b64.len() == sodalite::BOX_PUBLIC_KEY_LEN // && b64 == obj
	}
}

// The entire identity required to send messages on the peer network.
// An object consisting of a public key and a private key.
pub struct PrivateIdentity {
	pub public_key: sodalite::BoxPublicKey,
	pub secret_key: sodalite::BoxPublicKey,
}

impl PrivateIdentity {
	#[allow(dead_code)]
	fn to_identity(&self) -> Identity {
		Identity{public_key: self.public_key}
	}
}

// Length of the identity as a base64-encoded string.
// pub fn base64_identity_len() -> i32 {
// 	let n = ((sodalite::BOX_PUBLIC_KEY_LEN as f64) / 3.0).ceil() as i32;
// 	n * 4
// }