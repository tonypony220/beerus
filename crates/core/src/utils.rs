use bitvec::prelude::{BitSlice, BitVec, Msb0};
use bitvec::view::BitView;
use ethers::types::{Address, Bytes};
use eyre::{eyre, Result};
use helios::types::CallOpts;
use stark_hash::Felt;
use starknet::core::types::FieldElement;

pub fn felt_to_bits(felt: Felt) -> BitVec<u8, Msb0> {
    felt.to_be_bytes().view_bits::<Msb0>()[5..].to_bitvec()
}

pub fn felt_rs2path(felt: FieldElement) -> Felt {
    Felt::from_be_bytes(felt.to_bytes_be()).expect("felt conversion failed")
}

pub fn felt_from_bits(bits: &BitSlice<u8, Msb0>, mask: Option<usize>) -> Result<Felt> {
    if bits.len() != 251 {
        return Err(eyre!("expecting 251 bits"));
    }

    let mask = if let Some(x) = mask { x } else { 0 };

    let mut bytes = [0u8; 32];
    bytes.view_bits_mut::<Msb0>()[5 + mask..].copy_from_bitslice(&bits[mask..]);

    Felt::from_be_bytes(bytes).map_err(|e| eyre!(format!("{e}")))
}

pub fn simple_call_opts(addr: Address, data: Bytes) -> CallOpts {
    CallOpts { from: None, to: Some(addr), gas: None, gas_price: None, value: None, data: Some(data.into()) }
}
