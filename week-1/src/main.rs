use std::fmt::Error;

trait Serialize {
    fn serialize(&self) -> Vec<u8>;
}

trait Deserialize {
    fn deserialize(v: Vec<u8>) -> Result<Swap, std::fmt::Error>;
}

#[derive(Debug)]
struct Swap {
    qty_1: u32,
    qty_2: u32,
}

impl Deserialize for Swap {
    fn deserialize(v: Vec<u8>) -> Result<Self, std::fmt::Error> {
        if v.len() < 8 {
            return Err(std::fmt::Error);
        }
        let qty_1 = u32::from_be_bytes([v[0], v[1], v[2], v[3]]);
        let qty_2 = u32::from_be_bytes([v[4], v[5], v[6], v[7]]);

        return Ok(Self {
            qty_1,
            qty_2,
        });
    }
}

impl Serialize for Swap {
    fn serialize(&self) -> Vec<u8> {
        let mut v = vec![];
        v.extend_from_slice(&self.qty_1.to_be_bytes());
        v.extend_from_slice(&self.qty_2.to_be_bytes());
        return v;
    }
}

fn main() {
    let s = Swap {
        qty_1 : 1,
        qty_2 : 2,
    };
    let v= s.serialize();
    println!("{:?}",v);

    let v2 = Swap::deserialize(v).unwrap();
    println!("{:?}", v2);
}
