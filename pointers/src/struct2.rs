struct Y {
  broski: u32,
  dawg: u64,
  mane: char,
}

struct Yo {
  x: u32,
  y: (u32, u32, char),
  z: Y,
}

pub fn function<'a>() {
  let y = Y {
    broski: 10,
    dawg: 15,
    mane: 'h',
  };
  let borrowed_y = &y;
  let Y {
    broski: x1,
    dawg: x2,
    mane: x3,
  } = y;
  println!("{x1}, {x2}, {x3}");
  let yo = Yo {
    x: 129,
    y: (1231, 1123, 'd'),
    z: Y {
      broski: x1,
      dawg: x2,
      mane: x3,
    },
  };
  let Yo {
    x: x2,
    y: y2,
    z: Y {
      broski: x3,
      dawg: y3,
      mane: z3,
    },
  } = yo;
  println!("{x2}, {y2:?}, {x3}, {y3}, {z3}");
}
