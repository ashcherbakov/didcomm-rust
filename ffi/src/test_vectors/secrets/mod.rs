use lazy_static::lazy_static;
mod alice;
mod bob;

// TODO: Remove allow
#[allow(unused_imports)]
pub(crate) use alice::*;

// TODO: Remove allow
#[allow(unused_imports)]
pub(crate) use bob::*;
lazy_static! {
    pub(crate) static ref ALICE_AND_BOB_SECRETS: Vec<String> = vec![
        ALICE_SECRET_AUTH_KEY_ED25519.clone(),
        ALICE_SECRET_AUTH_KEY_P256.clone(),
        ALICE_SECRET_AUTH_KEY_SECP256K1.clone(),
        ALICE_SECRET_KEY_AGREEMENT_KEY_X25519.clone(),
        ALICE_SECRET_KEY_AGREEMENT_KEY_P256.clone(),
        ALICE_SECRET_KEY_AGREEMENT_KEY_P521.clone(),

        BOB_SECRET_KEY_AGREEMENT_KEY_X25519_1.clone(),
        BOB_SECRET_KEY_AGREEMENT_KEY_X25519_2.clone(),
        BOB_SECRET_KEY_AGREEMENT_KEY_X25519_3.clone(),
        BOB_SECRET_KEY_AGREEMENT_KEY_P256_1.clone(),
        BOB_SECRET_KEY_AGREEMENT_KEY_P256_2.clone(),
        BOB_SECRET_KEY_AGREEMENT_KEY_P384_1.clone(),
        BOB_SECRET_KEY_AGREEMENT_KEY_P384_2.clone(),
        BOB_SECRET_KEY_AGREEMENT_KEY_P521_1.clone(),
        BOB_SECRET_KEY_AGREEMENT_KEY_P521_2.clone(),
    ];
}