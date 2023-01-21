#[test]
fn test() {
    let env = Env::default();
    let contract_id = env.register_contract(None, StockLending);
    let client = StockLendingClient::new(&env, &contract_id);

    let depositor_1 = env.accounts().generate();
    let depositor_2 = env.accounts().generate();
    let short_seller = env.accounts().generate();

    
	
}