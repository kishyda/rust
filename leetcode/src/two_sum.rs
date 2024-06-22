pub fn two_sum(vector: Vec<i32>, target: i32) -> Vec<i32> {
  let mut result = Vec::new();
  let vecslice = &vector[..];
  'outer: for (index1, num1) in vecslice.iter().enumerate(){
    for (index2, num2) in vecslice.iter().enumerate().skip(index1 + 1) {
      if num1 + num2 == target {
        result.push(index1 as i32);
        result.push(index2 as i32);
        break 'outer;
      }
    }
  }
  result
}
