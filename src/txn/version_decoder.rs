
pub fn verion_number(txn_hex: &str) -> u32 {
    let txn_bytes = hex::decode(txn_hex).expect("Error decoding to hex");
    let version_bytes = <[u8; 4]>::try_from(&txn_bytes[0..4]).unwrap();
    println!("version bytes: {:?}", version_bytes);
    u32::from_le_bytes(version_bytes)
}