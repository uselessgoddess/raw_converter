use bitvec::prelude::*;

use std::convert::From;
use num_traits::AsPrimitive;
use num_traits::Unsigned;

fn split_bytes_into_7bit_chunks<T:Unsigned + From<u8> + AsPrimitive<u8> + BitStore>(bytes: &[u8]) -> (Vec<T>, usize){
    let mut chunks:Vec<T> = Vec::new();
    let bits_size = 8 * std::mem::size_of::<T>()-1;
    let mut buffer = BitVec::<T>::with_capacity(bits_size);
    let mut buffer_size = 0;

    for byte in bytes{
        let mut byte = *byte as u8;
        for i in 0..8 {
            if buffer_size == bits_size{
                let chunk = buffer.into_vec().iter().fold(0u8, |acc, bit| (acc << 1) | bit.as_());
                chunks.push(T::from(chunk));
                buffer = BitVec::<T>::with_capacity(bits_size);
                buffer_size = 0;
            }
            buffer.push(byte & 128 == 128);
            byte <<= 1;
            buffer_size += 1;
        }
    }
    if buffer_size > 0 {
        let chunk = buffer.into_vec().iter().fold(0u8, |acc, bit| (acc << 1) | bit.as_());
        chunks.push(T::from(chunk));
    }
    (chunks, buffer_size)
}

fn main(){

}

