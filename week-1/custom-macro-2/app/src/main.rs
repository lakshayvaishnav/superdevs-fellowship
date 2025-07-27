use serialize_macro::{ DeserializeNumberStruct, SerializeNumberStruct };
use serialize_macro_traits::{ Deserialize, Serialize };
use std::fmt::Error;

#[derive(SerializeNumberStruct, DeserializeNumberStruct, Debug)]
pub struct Quant {
    qty1: i32,
    qty2: i32,
}
fn main() {
    let lax = Quant {
        qty1 : 23,
        qty2 : 69
    };

    let laxserial = lax.serialize();
    println!("serialized struct : {:?}",laxserial );

    let laxdeserial = Quant::deserialize(&laxserial);

    println!("dserialized : {:?}", laxdeserial);

}
