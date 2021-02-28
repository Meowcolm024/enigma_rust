use rand::rngs::StdRng;
use rand::seq::SliceRandom;
use rand::SeedableRng;
use std::convert::TryInto;

type Table = (Vec<u8>, Vec<u8>);

pub struct Enigma {
    rotors: (Table, Table, Table),
    reflector: Vec<u8>,
    pointer: (u8, u8, u8),
}

fn assoc(l: Vec<u8>) -> Table {
    let mut x: Vec<(u8, u8)> = Vec::new();
    for i in 0..26 {
        x.push((l[i], i.try_into().unwrap()));
    }
    x.sort_by_key(|k| k.0);
    let result: Vec<u8> = x.iter().map(|x| x.1).collect();
    (l, result)
}

fn step(e: &mut Enigma) {
    fn divmod(x: u8, y: u8) -> (u8, u8) {
        (x % y, x / y)
    }
    let (a, b, c) = e.pointer;
    let (c1, z) = divmod(c + 1, 26);
    let (c2, y) = divmod(b + c1, 26);
    let x = (a + c2) % 26;
    e.pointer = (x, y, z);
}

fn cipher(p: u8, e: &mut Enigma) -> u8 {
    struct Val {
        val: u8,
    }
    trait Chainable {
        fn chain(&mut self, l: &Vec<u8>, s: u8) -> &mut Self;
    }
    impl Chainable for Val {
        fn chain(&mut self, l: &Vec<u8>, s: u8) -> &mut Self {
            self.val = (l[usize::from((s + self.val) % 26)] + 26 - s) % 26;
            self
        }
    }
    let a = &e.rotors.0;
    let b = &e.rotors.1;
    let c = &e.rotors.2;
    let (x, y, z) = e.pointer;
    let out = Val { val: p }
        .chain(&c.0, z)
        .chain(&b.0, y)
        .chain(&a.0, x)
        .chain(&e.reflector, 0)
        .chain(&a.1, x)
        .chain(&b.1, y)
        .chain(&c.1, z)
        .val;
    step(e);
    out
}

pub fn enigma(seed: u64, pos: (u8, u8, u8)) -> Enigma {
    let mut r = StdRng::seed_from_u64(seed);
    let rt: Vec<u8> = (0..26).collect();
    let mut rt1 = rt.clone();
    rt1.shuffle(&mut r);
    let mut rt2 = rt.clone();
    rt2.shuffle(&mut r);
    let mut rt3 = rt.clone();
    rt3.shuffle(&mut r);
    let reflc: Vec<u8> = (0..26).rev().collect();
    Enigma {
        rotors: (assoc(rt1), assoc(rt2), assoc(rt3)),
        reflector: reflc,
        pointer: pos,
    }
}

pub fn cipherstr(p: String, e: &mut Enigma) -> String {
    p.as_bytes()
        .iter()
        .map(|x| (cipher(*x - 65, e) + 65) as char)
        .collect()
}

// pub fn ciphervec(p: Vec<u8>, e: &mut Enigma) -> Vec<u8> {
//     p.iter().map(|x| cipher(*x, e)).collect()
// }
