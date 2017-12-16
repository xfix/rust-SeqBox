#[allow(non_camel_case_types)]
pub enum HashType {
    SHA1,
    SHA2_256,     SHA256,
    SHA2_512_256,
    SHA2_512_512, SHA512,
    BLAKE2B_256,
    BLAKE2B_512,
    BLAKE2S_128,
    BLAKE2S_256
}

pub type HashBytes = (HashType, Box<[u8]>);

pub mod specs {
    use super::*;

    struct Test<'a> {
        a : &'a [u8]
    }

    pub struct Param {
        hash_func_type : Box<[u8]>,
        digest_length  : usize
    }

    macro_rules! param {
        (
            [ $( $val:expr ),* ]; $len:expr
        ) => {
            Param { hash_func_type : Box::new([ $( $val ),* ]),
                    digest_length  : $len }
        }
    }

    pub fn hash_type_to_param (hash_type : &HashType) -> Param {
        use super::HashType::*;
        match *hash_type {
            SHA1                  => param!([0x11      ]; 0x14),
            SHA2_256     | SHA256 => param!([0x12      ]; 0x20),
            SHA2_512_256          => param!([0x13      ]; 0x20),
            SHA2_512_512 | SHA512 => param!([0x13      ]; 0x40),
            BLAKE2B_256           => param!([0xb2, 0x20]; 0x20),
            BLAKE2B_512           => param!([0xb2, 0x40]; 0x40),
            BLAKE2S_128           => param!([0xb2, 0x50]; 0x10),
            BLAKE2S_256           => param!([0xb2, 0x60]; 0x20),
        }
    }
}

mod hash {
    extern crate ring;
    extern crate blake;

    use super::*;

    pub struct Ctx {
        ctx : _Ctx
    }

    #[allow(non_camel_case_types)]
    enum _Ctx {
        SHA1(ring::digest::Context),
        SHA256(ring::digest::Context),
        SHA512(ring::digest::Context),
        BLAKE2B_256(blake::Blake),
        BLAKE2B_512(blake::Blake)
    }

    impl Ctx {
        fn new(hash_type : HashType) -> Result<Ctx, ()> {
            let ctx = match hash_type {
                HashType::SHA1        =>
                    Some(_Ctx::SHA1(
                        ring::digest::Context::new(&ring::digest::SHA1))),
                HashType::SHA256      =>
                    Some(_Ctx::SHA256(
                        ring::digest::Context::new(&ring::digest::SHA256))),
                HashType::SHA512      =>
                    Some(_Ctx::SHA512(
                        ring::digest::Context::new(&ring::digest::SHA512))),
                HashType::BLAKE2B_256 =>
                    Some(_Ctx::BLAKE2B_256(
                        blake::Blake::new(256).unwrap())),
                HashType::BLAKE2B_512 =>
                    Some(_Ctx::BLAKE2B_512(
                        blake::Blake::new(512).unwrap())),
                _                     => None
            };
            match ctx {
                Some(ctx) => Ok(Ctx { ctx }),
                None      => Err(())
            }
        }

        fn hash_type_is_supported(hash_type : HashType) -> bool {
            match Self::new(hash_type) {
                Ok(_)  => true,
                Err(_) => false
            }
        }

        fn update(&mut self, data : &[u8]) {
            match self.ctx {
                _Ctx::SHA1(ref mut ctx)        =>
                    ctx.update(data),
                _Ctx::SHA256(ref mut ctx)      =>
                    ctx.update(data),
                _Ctx::SHA512(ref mut ctx)      =>
                    ctx.update(data),
                _Ctx::BLAKE2B_256(ref mut ctx) =>
                    ctx.update(data),
                _Ctx::BLAKE2B_512(ref mut ctx) =>
                    ctx.update(data)
            }
        }

        fn finish(self, hashval : &mut [u8]) {
            match self.ctx {
                _Ctx::SHA1(ctx)            =>
                    hashval.copy_from_slice(ctx.finish().as_ref()),
                _Ctx::SHA256(ctx)          =>
                    hashval.copy_from_slice(ctx.finish().as_ref()),
                _Ctx::SHA512(ctx)          =>
                    hashval.copy_from_slice(ctx.finish().as_ref()),
                _Ctx::BLAKE2B_256(mut ctx) =>
                    ctx.finalise(hashval),
                _Ctx::BLAKE2B_512(mut ctx) =>
                    ctx.finalise(hashval)
            }
        }
    }
}