pub struct Timetracker {
  status: Option<String>,
}

impl Timetracker {
  pub fn new() -> Self {
    Timetracker { status: None }
  }

  pub fn get_status(&mut self) -> String {
    match &self.status {
      Some(s) => s.to_string(),
      None => String::from(""),
    }
  }
  pub fn set_status(&mut self, value: &str) {
    self.status = Some(value.to_string());
  }
}
