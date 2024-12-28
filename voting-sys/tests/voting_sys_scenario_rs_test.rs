use multiversx_sc_scenario::*;

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();

    blockchain.register_contract("mxsc:output/voting-sys.mxsc.json", voting_sys::ContractBuilder);
    blockchain
}

#[test]
fn empty_rs() {
    world().run("scenarios/voting_sys.scen.json");
}
