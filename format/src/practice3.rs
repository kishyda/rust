pub struct StupidStruct {
  pub a: i8,
  pub b: i8,
  pub c: i8,
  pub d: i8,
  pub e: i8,
}

impl std::fmt::Debug for StupidStruct {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f, "bro... {} is equal to {}\nbro... {} is equal to {}\nbro... {} is equal to {}\nbro... {} is equal to {}\nbro... {} is equal to {}\n and that's a fact",self.a, self.a, self.b, self.b, self.c, self.c, self.d, self.d, self.e,self.e )
  }
}

impl std::fmt::Display for StupidStruct {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f, "bro... {} is equal to {}\nbro... {} is equal to {}\nbro... {} is equal to {}\nbro... {} is equal to {}\nbro... {} is equal to {}\n",self.a, self.a, self.b, self.b, self.c, self.c, self.d, self.d, self.e,self.e )
  }
}
