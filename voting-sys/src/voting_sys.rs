#![no_std]

#[allow(unused_imports)]
use multiversx_sc::imports::*;

pub mod candidate_result;

use candidate_result::CandidateResult;

#[multiversx_sc::contract]
pub trait VotingSys {
    
    #[init]
    fn init(&self, candidate_fee: BigUint) {
        self.is_active().set(&false);
        self.candidates().clear();
        self.candidate_fee().set(candidate_fee);
    }

    #[only_owner]
    #[endpoint(addCandidate)]
    #[payable("EGLD")]
    fn add_candidate(&self, name: ManagedBuffer<Self::Api>) {
        let payment = self.call_value().egld_value().clone_value();
        let candidate_fee = self.candidate_fee().get();

        require!(
            payment >= candidate_fee,
            "Insufficient payment! Registration costs 0.1 EGLD."
        );

        for candidate in self.candidates().iter() {
            if candidate.name == name {
                sc_panic!("Candidate already exists!");
            }
        }

        self.candidates().push(&CandidateResult {
            name: name,
            votes: 0,
        });
    }

    #[only_owner]
    #[endpoint(startSession)]
    fn start_session(&self) {
        let candidates_len = self.candidates().len();
        require!(candidates_len > 0, "No candidates have been added!");
        require!(candidates_len != 1, "This is not communism!");

        let current_time = self.blockchain().get_block_timestamp();
        self.start_time().set(&current_time);
        self.is_active().set(&true);
    }

    #[only_owner]
    #[endpoint(closeSession)]
    fn close_session(&self) {

        let current_time = self.blockchain().get_block_timestamp();
        
        require!(current_time >= self.start_time().get(), "Voting has not started yet!");
        require!(self.is_active().get(), "Voting session is already closed!");

        self.end_time().set(&current_time);
        self.is_active().set(&false);
    }

    #[view(getResults)]
    fn get_results(&self) -> MultiValueEncoded<CandidateResult<Self::Api>> {
        require!(!self.is_active().get(), "Results are not available until the voting session ends!");
    
        let mut results = MultiValueEncoded::new();
    
        for candidate in self.candidates().iter() {
            results.push(candidate);
        }
    
        results
    }

    #[endpoint(castVote)]
    fn cast_vote(&self, candidate: ManagedBuffer<Self::Api>) {
        let caller = self.blockchain().get_caller();

        require!(self.candidates().len() > 0, "No candidates available!");
        require!(self.is_active().get(), "Voting session is not active!");
        
        let has_voted = self.registered_voters().get(&caller).unwrap_or(false);

        require!(!has_voted, "You have already voted!");

        let mut found = false;

        for (idx, cand) in self.candidates().iter().enumerate() {
            if cand.name.as_ref() == candidate.as_ref() {
                let mut candidate_result = cand.clone();
                candidate_result.votes += 1;
                self.candidates().set(idx, &candidate_result);
                found = true;
                break;
            }

        }

        require!(found, "Invalid candidate!");

        self.registered_voters().insert(caller, true);
    }

    #[view(getCandidates)]
    fn get_candidates(&self) -> MultiValueEncoded<ManagedBuffer<Self::Api>> {
        let mut candidates_list = MultiValueEncoded::new();
        for candidate in self.candidates().iter() {
            candidates_list.push(candidate.name.clone());
        }
        candidates_list
    }

    #[endpoint(register)]
    fn register(&self) {
        let caller = self.blockchain().get_caller();
        require!(
            !self.registered_voters().contains_key(&caller),
            "You are already registered!"
        );

        self.registered_voters().insert(caller.clone(), false);
    }

    #[storage_mapper("candidates")]
    fn candidates(&self) -> VecMapper<CandidateResult<Self::Api>>;

    #[storage_mapper("candidate_fee")]
    fn candidate_fee(&self) -> SingleValueMapper<BigUint>;

    #[view(isActive)]
    #[storage_mapper("is_active")]
    fn is_active(&self) -> SingleValueMapper<bool>;

    #[storage_mapper("start_time")]
    fn start_time(&self) -> SingleValueMapper<u64>;

    #[storage_mapper("end_time")]
    fn end_time(&self) -> SingleValueMapper<u64>;

    #[storage_mapper("registered_voters")]
    fn registered_voters(&self) -> MapMapper<ManagedAddress, bool>;
}   
