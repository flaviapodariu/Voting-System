#![no_std]

#[allow(unused_imports)]
use multiversx_sc::imports::*;

mod candidate_result;
use candidate_result:: CandidateResult;

#[multiversx_sc::contract]
pub trait VotingSys {

    #[init]
    fn init(&self) {
        self.is_active().set(&false);
        self.candidates().clear();
    }

    // #[only_owner]
    #[endpoint(addCandidate)]
    fn add_candidate(&self, name: ManagedBuffer<Self::Api>) {
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

    // #[only_owner]
    #[endpoint(startSession)]
    fn start_session(&self) {
        let candidates_len = self.candidates().len();
        require!(candidates_len > 0, "No candidates have been added!");
        require!(candidates_len != 1, "This is not communism!");

        let current_time = self.blockchain().get_block_timestamp();
        self.start_time().set(&current_time);
        self.is_active().set(&true);
    }

    // #[only_owner]
    #[endpoint(closeSession)]
    fn close_session(&self) {

        let current_time = self.blockchain().get_block_timestamp();
        
        require!(current_time >= self.start_time().get(), "Voting has not started yet!");
        require!(self.is_active().get(), "Voting session is already closed!");

        self.end_time().set(&current_time);
        self.is_active().set(&false);
    }

    #[view(getResults)]
    fn get_results(&self) -> ManagedVec<CandidateResult<Self::Api>> {
        require!(
            !self.is_active().get(),
            "Results are not available until the voting session ends!"
        );
    
        let mut results = ManagedVec::new();
    
        for candidate in self.candidates().iter() {
            results.push(candidate);
        }
    
        results
    }

    #[endpoint(castVote)]
    fn cast_vote(&self, candidate: ManagedBuffer<Self::Api>) {
        let caller = self.blockchain().get_caller();

        require!(self.is_active().get(), "Voting session is not active!");

        require!(
            !self.has_voted(&caller).get(),
            "You have already voted!"
        );

        let valid_candidate = self.candidates()
            .iter()
            .any(|c| c.name == candidate);

        require!(valid_candidate, "Invalid candidate!");

        let current_votes = self.votes(&candidate).get();
        self.votes(&candidate).set(&(current_votes + 1));

        self.has_voted(&caller).set(&true);
    }

    #[view(getCandidates)]
    fn get_candidates(&self) -> ManagedVec<ManagedBuffer<Self::Api>> {
        let mut candidates_list = ManagedVec::new();
        for candidate in self.candidates().iter() {
            candidates_list.push(candidate.name.clone());
        }
        candidates_list
    }

    #[storage_mapper("candidates")]
    fn candidates(&self) -> VecMapper<CandidateResult<Self::Api>>;

    #[storage_mapper("votes")]
    fn votes(&self, candidate: &ManagedBuffer) -> SingleValueMapper<u64>;

    #[storage_mapper("has_voted")]
    fn has_voted(&self, voter: &ManagedAddress) -> SingleValueMapper<bool>;

    #[storage_mapper("is_active")]
    fn is_active(&self) -> SingleValueMapper<bool>;

    #[storage_mapper("start_time")]
    fn start_time(&self) -> SingleValueMapper<u64>;

    #[storage_mapper("end_time")]
    fn end_time(&self) -> SingleValueMapper<u64>;
}
