use std::io::{Result, Write, Read};
use salsa20;
use rsa;
use rand::rngs::OsRng;
use rsa::pkcs1::{EncodeRsaPublicKey, DecodeRsaPublicKey};
use salsa20::cipher::{KeyIvInit, StreamCipher};
use rand::RngCore;

pub struct ASecureStream<T: Read + Write + Clone> {
    pub stream: T,
    pub key_mine: Vec<u8>,
    pub key_other: Vec<u8>,
    pub rsa_mine: rsa::RsaPrivateKey,
    pub rsa_other: rsa::RsaPublicKey
}

impl<T: Read + Write + Clone> ASecureStream<T> {
    pub fn new(is_conn_starter: bool, stream: &mut T) -> Result<Self> {
        let key_rsa_mine: rsa::RsaPrivateKey;
        let key_rsa_other: rsa::RsaPublicKey;
        let key_cam_mine: Vec<u8>;
        let key_cam_other: Vec<u8>;
        if is_conn_starter {
            key_rsa_mine = rsa::RsaPrivateKey::new(&mut OsRng, 4096).unwrap();
            stream.write_all(key_rsa_mine.to_public_key().to_pkcs1_der().unwrap().as_bytes())?;
            stream.flush()?;
            let mut temp: Vec<u8> = vec![];
            stream.read_to_end(&mut temp)?;
            key_rsa_other = rsa::RsaPublicKey::from_pkcs1_der(&temp).unwrap();
            let mut dest = [0u8; 16];
            rand::thread_rng().fill_bytes(&mut dest);
            key_cam_mine = dest.to_vec();
            stream.write_all(key_rsa_other.encrypt(&mut OsRng, rsa::Pkcs1v15Encrypt, key_cam_mine.as_slice()).unwrap().as_slice())?;
            stream.flush()?;
            let mut temp2: Vec<u8> = vec![];
            stream.read_to_end(&mut temp2)?;
            key_cam_other = key_rsa_mine.decrypt(rsa::Pkcs1v15Encrypt, temp2.as_slice()).unwrap();
        } else {
            key_rsa_mine = rsa::RsaPrivateKey::new(&mut OsRng, 4096).unwrap();
            let mut temp: Vec<u8> = vec![];
            stream.read_to_end(&mut temp)?;
            key_rsa_other = rsa::RsaPublicKey::from_pkcs1_der(&temp).unwrap();
            stream.write_all(key_rsa_mine.to_public_key().to_pkcs1_der().unwrap().as_bytes())?;
            stream.flush()?;
            let mut dest = [0u8; 16];
            rand::thread_rng().fill_bytes(&mut dest);
            key_cam_mine = dest.to_vec();
            let mut temp2: Vec<u8> = vec![];
            stream.read_to_end(&mut temp2)?;
            key_cam_other = key_rsa_mine.decrypt(rsa::Pkcs1v15Encrypt, temp2.as_slice()).unwrap();
            stream.write_all(key_rsa_other.encrypt(&mut OsRng, rsa::Pkcs1v15Encrypt, key_cam_mine.as_slice()).unwrap().as_slice())?;
            stream.flush()?;
        }
        Ok(Self {stream: stream.clone(), key_mine: key_cam_mine, key_other: key_cam_other, rsa_mine: key_rsa_mine, rsa_other: key_rsa_other})
    }
}
impl<T: Read + Write + Clone> Read for ASecureStream<T> {
    fn read(&mut self, mut buf: &mut [u8]) -> Result<usize> {
        let mut temp: Vec<u8> = Vec::new();
        self.stream.read_to_end(&mut temp).unwrap();
        let nonce = self.rsa_mine.decrypt(rsa::Pkcs1v15Encrypt, temp[0..512].as_ref()).unwrap();
        let mut cipher = salsa20::Salsa20::new(salsa20::Key::from_slice(&self.key_mine.as_slice()), salsa20::Nonce::from_slice(&nonce.as_slice()));
        let mut message = temp[512..temp.len()].to_vec();
        cipher.apply_keystream(&mut message);
        buf.write(message.as_slice())
    }
}
impl<T: Read + Write + Clone> Write for ASecureStream<T> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        let mut nonce = [0u8; 16];
        rand::thread_rng().fill_bytes(&mut nonce);
        let mut encnonce = self.rsa_other.encrypt(&mut OsRng, rsa::Pkcs1v15Encrypt, &nonce).unwrap();
        let mut encmsg = buf.clone().to_vec();
        let mut cipher = salsa20::Salsa20::new(salsa20::Key::from_slice(&self.key_other.as_slice()), salsa20::Nonce::from_slice(&nonce.as_slice()));
        cipher.apply_keystream(&mut encmsg);
        let mut tosend: Vec<u8> = Vec::new();
        tosend.append(&mut encnonce);
        tosend.append(&mut encmsg);
        self.stream.write_all(&tosend);
        self.stream.flush();
        Ok(tosend.len())
    }
    fn flush(&mut self) -> Result<()> {
        self.stream.flush()
    }
}
impl<T: Read + Write + Clone> Clone for ASecureStream<T> {
    fn clone(&self) -> Self {
        Self {stream: self.stream.clone(), key_mine: self.key_mine.clone(), key_other: self.key_other.clone(), rsa_mine: self.rsa_mine.clone(), rsa_other: self.rsa_other.clone()}
    }
}