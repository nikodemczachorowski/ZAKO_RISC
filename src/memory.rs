pub trait Memable {
    const SIZE: usize;
    fn from_bytes(bytes: &[u8]) -> Self;
    fn to_bytes(self, dest: &mut [u8]);
}

pub struct Memory {
    mem: Vec<u8>,
}

macro_rules! impl_mem_ops {
    ($($t:ty),*) => {
        $(
            impl Memable for $t {
                const SIZE: usize = std::mem::size_of::<$t>();

                fn from_bytes(bytes: &[u8]) -> Self{
                    let mut buf = [0u8; Self::SIZE];
                    buf.copy_from_slice(&bytes[..Self::SIZE]);
                    <$t>::from_le_bytes(buf)
                }

                fn to_bytes(self, dest: &mut [u8]) {
                    dest.copy_from_slice(&self.to_le_bytes());
                }
            }
        )*
    };
}

impl_mem_ops!(u8, u16, u32, i8, i16, i32);

impl Memory {
    pub fn read<T: Memable>(&self, addr: u32) -> T {
        let us_addr = addr as usize;
        T::from_bytes(&self.mem[us_addr..us_addr + T::SIZE])
    }

    pub fn write<T: Memable>(&mut self, addr: u32, val: T) {
        let us_addr = addr as usize;
        val.to_bytes(&mut self.mem[us_addr..us_addr + T::SIZE]);
    }
}
