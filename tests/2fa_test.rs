use std::time::{SystemTime, UNIX_EPOCH};
use totp_rs::{Algorithm, Rfc6238, Secret, TOTP};

#[test]
fn test_2fa() {
    let totp = TOTP::new_unchecked(
        Algorithm::SHA1,
        6,
        1,
        30,
        Secret::Encoded("E3BEK27KXIP225OA".to_string())
            .to_bytes()
            .unwrap(),
    );

    // let totp = TOTP::new(
    //     Algorithm::SHA1,
    //     6,
    //     1,
    //     30,
    //     Secret::Encoded("E3BEK27KXIP225OA".to_string())
    //         .to_bytes()
    //         .unwrap(),
    // )
    // .unwrap();

    // let mut rfc = Rfc6238::with_defaults(
    //     // "E3BEK27KXIP225OA".as_bytes().to_vec(),
    //     Secret::Encoded("E3BEK27KXIP225OA".to_string()).to_bytes().unwrap(),
    // ).unwrap();
    // let totp = TOTP::from_rfc6238(rfc).unwrap();

    let t = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    println!("t:{:?}", t);
    let step_t = (t / totp.step);
    println!("step_t:{:?}", step_t);
    let secret_as_byte: &[u8] = totp.secret.as_ref();
    println!(
        "secret_as_byte: {:?}, secret_with_base32:{:?}",
        hex::encode(secret_as_byte),
        totp.get_secret_base32()
    );

    // let token = totp.generate_current().unwrap();
    let token = totp.generate(t);
    println!("{}", token);
}

#[test]
fn test_2fa_valid() {
    let mut rfc = Rfc6238::with_defaults(
        "E3BEK27KXIP225OA".as_bytes().to_vec(),
        // Secret::Raw("E3BEK27KXIP225OA".as_bytes().to_vec()).to_bytes().unwrap(),
    )
    .unwrap();
    let totp = TOTP::from_rfc6238(rfc).unwrap();

    let check_res = totp.check_current("393786");
    println!("{:?}", check_res);
}
