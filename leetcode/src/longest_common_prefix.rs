pub fn access_string(strs: Vec<String>, index: usize) -> String {
  let result = (strs).into_iter().nth(index).expect("");
  result
}

pub fn access_char(str: String, index: usize) -> char {
  str.chars().nth(index).expect("")
}

pub fn access_char_from_vec(strs: Vec<String>, vec_index: usize, string_index: usize) -> char {
  let result = (strs).into_iter().nth(vec_index).expect("");
  access_char(result, string_index)
}

pub fn longest_common_prefix(strs: Vec<String>) -> String {
  if strs.is_empty() {
    return String::new();
  }

  let mut prefix = strs[0].clone();

  for s in strs.iter().skip(1) {
    while !s.starts_with(&prefix) {
      if prefix.is_empty() {
        return String::new();
      }
      prefix.pop();
    }
  }

  prefix
}
