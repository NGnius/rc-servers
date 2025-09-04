use num::BigInt;
use rand::Rng;
use simple_rijndael::impls::RijndaelCbc;
use simple_rijndael::paddings::Pkcs7Padding;

pub struct Keys {
    pub pub_key: Vec<u8>,
    pub enc: CryptoImpl,
}

const SECRET_LEN: usize = 160 / 8;
const PRIME_768: &[u8] = &[255, 255, 255, 255, 255, 255, 255, 255, 201, 15,
                        218, 162, 33, 104, 194, 52, 196, 198, 98, 139,
                        128, 220, 28, 209, 41, 2, 78, 8, 138, 103,
                        204, 116, 2, 11, 190, 166, 59, 19, 155, 34,
                        81, 74, 8, 121, 142, 52, 4, 221, 239, 149,
                        25, 179, 205, 58, 67, 27, 48, 43, 10, 109,
                        242, 95, 20, 55, 79, 225, 53, 109, 109, 81,
                        194, 69, 228, 133, 181, 118, 98, 94, 126, 198,
                        244, 76, 66, 233, 166, 58, 54, 32, 255, 255,
                        255, 255, 255, 255, 255, 255];
const PRIME_ROOT: u8 = 22;

pub fn generate_encryption_details(client_pub_key: &[u8]) -> Keys {
    let client_num = BigInt::from_bytes_be(num::bigint::Sign::Plus, client_pub_key);
    let big_0 = BigInt::from(0);
    let prime_root = BigInt::from(PRIME_ROOT);
    let my_prime = BigInt::from_bytes_be(num::bigint::Sign::Plus, PRIME_768);
    log::debug!("Generating keys for client pub key {}", client_num);
    let mut rng = rand::rng();
    let mut bytes = rng.random::<[u8; SECRET_LEN]>();
    let mut my_secret = BigInt::from_bytes_be(num::bigint::Sign::Plus, &bytes);
    while my_secret >= &my_prime - 1 || my_secret == big_0 {
        bytes = rng.random::<[u8; SECRET_LEN]>();
        my_secret = BigInt::from_bytes_be(num::bigint::Sign::Plus, &bytes);
        log::debug!("Generated secret {} (prime to beat: {})", my_secret, my_prime);
    }
    let my_pub_key = prime_root.modpow(&my_secret, &my_prime);
    let shared_key = client_num.modpow(&my_secret, &my_prime);
    log::debug!("Generated shared key {} and pub key {}", shared_key, my_pub_key);
    let shared_key = shared_key.to_bytes_be().1;
    let enc_key: Vec<u8> = ring::digest::digest(&ring::digest::SHA256, &shared_key).as_ref().into();
    log::debug!("Encryption key is {:?}", enc_key.as_slice());
    Keys {
        pub_key: my_pub_key.to_signed_bytes_be(),
        enc: CryptoImpl::new(enc_key),
    }
}

pub struct CryptoImpl {
    crypto: RijndaelCbc<Pkcs7Padding>,
    iv: [u8; 16],
    key: Vec<u8>
}

impl CryptoImpl {
    fn new(key: Vec<u8>) -> Self {
        Self {
            crypto: RijndaelCbc::<Pkcs7Padding>::new(&key, 16).unwrap(),
            iv: [0u8; 16],
            key,
        }
    }

    fn decrypt(&self, data: Vec<u8>) -> Vec<u8> {
        self.crypto.decrypt(&self.iv, data).unwrap_or_default()
    }

    fn encrypt(&self, data: Vec<u8>) -> Vec<u8> {
        self.crypto.encrypt(&self.iv, data).unwrap_or_default()
    }
}

impl polariton::packet::Cryptographer for CryptoImpl {
    fn decrypt(&self, data: Vec<u8>) -> Vec<u8> {
        self.decrypt(data)
    }

    fn encrypt(&self, data: Vec<u8>) -> Vec<u8> {
        self.encrypt(data)
    }

    fn secret(&self) -> &'_ [u8] {
        self.key.as_slice()
    }
}
