enum Temperature {
  Celcius(i32),
  Fahrenheit(i32),
}
pub fn function() {
  let temperature = Temperature::Celcius(25);
  match temperature {
    Temperature::Celcius(t) if t > 30 => println!("It's really hot"),
    Temperature::Celcius(t) => println!("It's {t}C, it ain't that hot"),
    Temperature::Fahrenheit(t) if t > 100 => println!("IT's really hot"),
    Temperature::Fahrenheit(t) => println!("It's {t}F, it ain't that hot"),
  }
}
