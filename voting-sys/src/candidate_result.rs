use multiversx_sc::derive_imports::*;
use multiversx_sc::imports::*;


#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode, TypeAbi, ManagedVecItem)]
pub struct CandidateResult<M: ManagedTypeApi> {
    pub name: ManagedBuffer<M>,
    pub votes: u64,
}