use revmc::primitives::hex as hex_str;
#[allow(dead_code)]
const FIBONACCI_CODE: &[u8] =
  &hex_str!("5f355f60015b8215601a578181019150909160019003916005565b9150505f5260205ff3");
#[allow(dead_code)]
const FIBONACCI_HASH: [u8; 32] =
  hex_str!("ab1ad1211002e1ddb8d9a4ef58a902224851f6a0273ee3e87276a8d21e649ce8");
