pub const DIFFICULT_LEVEL: i32 = 3;
pub const MINING_REWARD: f32 = 100f32;

pub fn get_difficult_string() -> String {
  let mut s = String::new();

  for _i in 0..DIFFICULT_LEVEL {
    s.push_str("0");
  }

  s
}
