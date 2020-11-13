pub struct GameLog {
  pub entries: Vec<String>,
}

impl GameLog {
  pub fn add(mut self, msg: String) {
    self.entries.push(msg);
  }
}
