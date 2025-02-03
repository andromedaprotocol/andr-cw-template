#[cfg(test)]
mod tests {
    use crate::helpers::CwTemplateContract;
    use crate::msg::InstantiateMsg;
    use andromeda_testing::{
        mock::{mock_app, MockApp},
        mock_builder::MockAndromedaBuilder,
        MockAndromeda, MockContract,
    };
    use cosmwasm_std::{coin, Addr, Empty};
    use cw_multi_test::{Contract, ContractWrapper, Executor};

    pub fn contract_template() -> Box<dyn Contract<Empty>> {
        let contract = ContractWrapper::new(
            crate::contract::execute,
            crate::contract::instantiate,
            crate::contract::query,
        );
        Box::new(contract)
    }

    const USER: &str = "useraddress";
    const ADMIN: &str = "adminaddress";
    const NATIVE_DENOM: &str = "denom";
    const CONTRACT_NAME: &str = "{{project-name}}";

    fn setup() -> (MockApp, MockAndromeda) {
        let mut router = mock_app(Some(vec![NATIVE_DENOM]));
        let andr = MockAndromedaBuilder::new(&mut router, "admin")
            .with_wallets(vec![
                (ADMIN, vec![coin(1000, NATIVE_DENOM)]),
                (USER, vec![]),
            ])
            .with_contracts(vec![(CONTRACT_NAME, contract_template())])
            .build(&mut router);

        (router, andr)
    }

    fn proper_instantiate() -> (MockApp, CwTemplateContract) {
        let (mut app, andr) = setup();
        let code_id = andr.get_code_id(&mut app, CONTRACT_NAME);
        let msg = InstantiateMsg {
            count: 1i32,
            kernel_address: andr.kernel.addr().to_string(),
            owner: None,
        };
        let cw_template_contract_addr = app
            .instantiate_contract(code_id, app.api().addr_make(ADMIN), &msg, &[], "test", None)
            .unwrap();

        let cw_template_contract = CwTemplateContract(cw_template_contract_addr);

        (app, cw_template_contract)
    }

    mod count {
        use super::*;
        use crate::msg::ExecuteMsg;

        #[test]
        fn count() {
            let (mut app, cw_template_contract) = proper_instantiate();

            let msg = ExecuteMsg::Increment {};
            let cosmos_msg = cw_template_contract.call(msg).unwrap();
            app.execute(Addr::unchecked(USER), cosmos_msg).unwrap();
        }
    }
}
