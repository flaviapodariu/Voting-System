#![allow(non_snake_case)]

mod proxy;

use std::env;
use std::fs;
use multiversx_sc_snippets::imports::*;
use multiversx_sc_snippets::sdk;
use serde::{Deserialize, Serialize};
use std::{
    io::{Read, Write},
    path::Path,
};


const GATEWAY: &str = sdk::gateway::DEVNET_GATEWAY;
const STATE_FILE: &str = "state.toml";


#[tokio::main]
async fn main() {
    env_logger::init();

    let mut args = std::env::args();
    let _ = args.next();
    let cmd = args.next().expect("at least one argument required");
    let mut interact = ContractInteract::new().await;
    match cmd.as_str() {
        "deploy" => interact.deploy().await,
        "addCandidate" => interact.add_candidate().await,
        "startSession" => interact.start_session().await,
        "closeSession" => interact.close_session().await,
        "getResults" => interact.get_results().await,
        "castVote" => interact.cast_vote().await,
        "getCandidates" => interact.get_candidates().await,
        "register" => interact.register().await,
        _ => panic!("unknown command: {}", &cmd),
    }
}


#[derive(Debug, Default, Serialize, Deserialize)]
struct State {
    contract_address: Option<Bech32Address>
}

impl State {
        // Deserializes state from file
        pub fn load_state() -> Self {
            if Path::new(STATE_FILE).exists() {
                let mut file = std::fs::File::open(STATE_FILE).unwrap();
                let mut content = String::new();
                file.read_to_string(&mut content).unwrap();
                toml::from_str(&content).unwrap()
            } else {
                Self::default()
            }
        }
    
        /// Sets the contract address
        pub fn set_address(&mut self, address: Bech32Address) {
            self.contract_address = Some(address);
        }
    
        /// Returns the contract address
        pub fn current_address(&self) -> &Bech32Address {
            self.contract_address
                .as_ref()
                .expect("no known contract, deploy first")
        }
    }
    
    impl Drop for State {
        // Serializes state to file
        fn drop(&mut self) {
            let mut file = std::fs::File::create(STATE_FILE).unwrap();
            file.write_all(toml::to_string(self).unwrap().as_bytes())
                .unwrap();
        }
    }

struct ContractInteract {
    interactor: Interactor,
    wallet_address: Address,
    contract_code: BytesValue,
    state: State
}

impl ContractInteract {
    async fn new() -> Self {
        let current_dir = env::current_dir().expect("Failed to get current directory");
        println!("Current working directory: {:?}", current_dir); // Print the current directory

        let pem_path = current_dir.join("wallet.pem");

        let pem = fs::read_to_string(pem_path).expect("failed to read PEM file");

        let wallet = Wallet::from_pem_file_contents(pem).expect("invalid PEM file");

        let mut interactor = Interactor::new(GATEWAY).await;
        let wallet_address = interactor.register_wallet(wallet.clone());
        
        let contract_code = BytesValue::interpret_from(
            "mxsc:../output/voting-sys.mxsc.json",
            &InterpreterContext::default(),
        );

        ContractInteract {
            interactor,
            wallet_address,
            contract_code,
            state: State::load_state()
        }
    }

    async fn deploy(&mut self) {
        let candidate_fee = BigUint::<StaticApi>::from(5_000_000_000u128);

        let new_address = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .gas(30_000_000u64)
            .typed(proxy::VotingSysProxy)
            .init(candidate_fee)
            .code(&self.contract_code)
            .returns(ReturnsNewAddress)
            .prepare_async()
            .run()
            .await;
        let new_address_bech32 = bech32::encode(&new_address);
        self.state
            .set_address(Bech32Address::from_bech32_string(new_address_bech32.clone()));

    }

    async fn add_candidate(&mut self) {
        let egld_amount = BigUint::<StaticApi>::from(5_000_000_000u128);

        let name = ManagedBuffer::new_from_bytes(&b""[..]);

        let _response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(proxy::VotingSysProxy)
            .add_candidate(name)
            .egld(egld_amount)
            .returns(ReturnsResultUnmanaged)
            .prepare_async()
            .run()
            .await;
    }

    async fn start_session(&mut self) {
        let _response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(proxy::VotingSysProxy)
            .start_session()
            .returns(ReturnsResultUnmanaged)
            .prepare_async()
            .run()
            .await;
    }

    async fn close_session(&mut self) {
        let _response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(proxy::VotingSysProxy)
            .close_session()
            .returns(ReturnsResultUnmanaged)
            .prepare_async()
            .run()
            .await;
    }

    async fn get_results(&mut self) {
        let _result_value = self
            .interactor
            .query()
            .to(self.state.current_address())
            .typed(proxy::VotingSysProxy)
            .get_results()
            .returns(ReturnsResultUnmanaged)
            .prepare_async()
            .run()
            .await;
    }

    async fn cast_vote(&mut self) {
        let candidate = ManagedBuffer::new_from_bytes(&b""[..]);

        let _response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(proxy::VotingSysProxy)
            .cast_vote(candidate)
            .returns(ReturnsResultUnmanaged)
            .prepare_async()
            .run()
            .await;
    }

    async fn get_candidates(&mut self) {
        let _result_value = self
            .interactor
            .query()
            .to(self.state.current_address())
            .typed(proxy::VotingSysProxy)
            .get_candidates()
            .returns(ReturnsResultUnmanaged)
            .prepare_async()
            .run()
            .await;
    }

    async fn register(&mut self) {
        let _response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(proxy::VotingSysProxy)
            .register()
            .returns(ReturnsResultUnmanaged)
            .prepare_async()
            .run()
            .await;
    }

}
