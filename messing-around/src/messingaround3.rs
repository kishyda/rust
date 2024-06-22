struct Bro {
  x: i32,
  y: i32,
}

struct Bro2 {
  x: i32,
  y: i32, 
  z: i32,
}

enum Foo {
  Bar, 
  Baz,
  Qux(u32)
}

pub fn function() {
  let bro = Bro {x: 3, y: 5};
  let bro2 = Bro2 {x: 3, y: 4, z: 5};
  if let Bro {x: x1, y: y1} = bro {
    println!("{}, {}", x1, y1);
  } else {
    println!("Not working");
  }
}

pub fn function2() {
  let number = Some(6);
  let letter: Option<i32> = None;
  let emoticon: Option<i32> = None;

  if let Some(i) = number {
    println!("Matched{:?}!", i);
  }
  if let Some(i) = letter {
    println!("Matched {:?}", i);
  } else {
    println!("Didn't match a number. Let's go with a letter!");
  }
  let i_like_letters = false;

  if let Some(i) = emoticon {
    println!("Matched {:?}!", i);
  } else if i_like_letters {
    println!("Didn't match a number. Let's go with a letter!");
  } else {
    println!("I don't like letters. Let's go with an emotcon :)!");
  } 
}

pub fn function3() {
  let a = Foo::Bar;
  let b = Foo::Baz;
  let c = Foo::Qux(100);

  if let Foo::Bar = a {
    println!("a is foobar");
  }
  if let Foo::Bar = b {
    println!("b is a foobar");
  }
  if let Foo::Qux(value) = c {
    println!("c is {}", value);
  }
  if let Foo::Qux(value @ 100) = c {
    println!("c is {}", value);
  }
}

fn get_count_item(s: &str) -> (u64, &str) {
  let mut it = s.split(' ');
  let (Some(count_str), Some(item)) = (it.next(), it.next()) else {
    panic!("Can't segment count item pair: '{s}'");
  };
  let Ok(count) = count_str.parse() else {
    panic!("Can't parse integer: '{count_str}'");
  };
  (count, item)
}

//fn get_count_items2(s: &str) {
//  let mut it = s.split(' ');
//  let (count_str, item) = match (it.next(), it.next()) {
//    (Some(count_str), Some(item)) => (count_str, item),
//    _ => panic!("Couldn't segment count item pair: '{s}'"),
//  };
//  let count: i64 = match count_str.parse() {
//    Ok(value) => value,
//    Err(why) => panic!("error parsing from str to int, {}", why),
//  };
//}


