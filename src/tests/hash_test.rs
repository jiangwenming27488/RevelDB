use crate::utils::hash::hash;


#[test]
pub fn test1() {
    let val = hash("", 0usize, 0xbc9f1d34u32);
    assert_eq!(val, 0xbc9f1d34u32);
}

#[test]
pub fn test2() {
    let data1 = [0x62];

    let msg = std::str::from_utf8(&data1).expect("msg expect");
    let val = hash(msg, msg.len(), 0xbc9f1d34);
    assert_eq!(0xef1345c4, val);
}


#[test]
pub fn test3() {
    let data2 = [0xc3, 0x97];
    let msg = std::str::from_utf8(&data2).expect("utf8 vec expect");
    let val = hash(msg, msg.len(), 0xbc9f1d34);
    assert_eq!(val, 0x5b663814);
}

#[test]
pub fn test4() {
    let data3: [u8; 3] = [0xe2, 0x99, 0xa5];
    let msg = std::str::from_utf8(&data3).expect("utf8 vec expected");
    let val = hash(msg, msg.len(), 0xbc9f1d34);
    assert_eq!(val, 0x323c078f);
}

#[test]
pub fn test5() {
    let data4: [u8; 4] = [0xe1, 0x80, 0xb9, 0x32];
    let msg = std::str::from_utf8(&data4).expect("utf8 vec expected");
    let val = hash(msg, msg.len(), 0xbc9f1d34);
    assert_eq!(val, 0xed21633a);
}


