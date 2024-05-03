use ethnum::U256;

struct ECPoint {
  x: U256,
  y: U256,
  curve: Secp256k1
}

impl ECPoint {
  pub fn new(x: U256, y: U256, curve: Secp256k1) -> Self {
    Self {x, y, curve}
  }

  fn eq(&self, ec_point: ECPoint) -> bool {
    self.x == ec_point.x &&
    self.y == ec_point.y &&
    self.curve == ec_point.curve
  }
}

struct Secp256k1 {
  // y^2 = x^3 + ax + b mod(p)
  a: U256,
  b: U256,
  p: U256,
  // Generator Point
  G: U256
}

impl Secp256k1 {
  // y^2 = x^3 + 0x + 7
  // 
  fn new(&self) -> Self {
    let a: U256 = u256!(0);
    let b: U256 = u256!(7);
    // p = 2^256 −2^32 −977
    let p: U256 = u256!("115792089237316195423570985008687907852837564279074904382605163141518161494337");

    G = ECPoint::new(u256!("55066263022277343669578718895168534326250603453777594175500187360389116729240"), 
                     u256!("32670510020758816978083085130507043184471273380659243275938904335757337482424",
                     self));
  }

  fn add_points(&self, a: ECPoint, b: ECPoint) -> ECPoint {
    // slope = (y2 - y1) / (x2 - x1)
    let m = ((b.y - a.y) * (b.x - a.x).pow(-1, self.p)) % (self.p);

    // x3 = slope^2 - x1 - x2
    // y3 = slope * (x1 - x3) - y1

    let x3 = (m.pow(2) - a.x - b.x) % self.p;
    let y3 = (m * (a.x - x3) - a.y) % self.p;

    ECPoint {
      x3,
      y3,
    }
  }

}