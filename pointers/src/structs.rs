struct Foo {
  x: (u32, u32),
  y: u32,
}
pub fn function() {
  let foo = Foo { x: (1, 2), y: 3 };
  match foo {
    // Foo { x: (1, b), y: c } => println!("x: ({}, {}), y: {}", 1, b, c),
    Foo { x: (_, _), y: _ } => println!("The values do not matter at all"),
  }
  let faa = Foo { x: (1, 2), y: 3 };
  let Foo { x: x0, y: y0 } = faa;
  println!("Outside: x0 = {x0:?}, y0 = {y0:?}");
  struct Bar {
    foo: Foo,
  }

  let bar = Bar { foo: faa };
  let Bar {
    foo: Foo {
      x: nested_x,
      y: nested_y,
    },
  } = bar;
  println!("Nested: nested_x = {nested_x:?}, nested_y = {nested_y:?}");
}
