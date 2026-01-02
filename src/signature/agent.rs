pub(crate) mod l1 {
    use alloy::{
        dyn_abi::Eip712Domain,
        primitives::{Address, B256},
        sol,
        sol_types::{eip712_domain, SolStruct},
    };

    use crate::eip712::Eip712;

    sol! {
        #[derive(Debug)]
        struct Agent {
            string source;
            bytes32 connectionId;
        }
    }

    impl Agent {
        pub fn new(is_mainnet: bool, connection_id: B256) -> Agent {
            let source = if is_mainnet { "a" } else { "b" }.to_string();
            Agent {
                source,
                connectionId: connection_id,
            }
        }
    }

    impl Eip712 for Agent {
        fn domain(&self) -> Eip712Domain {
            eip712_domain! {
                name: "Exchange",
                version: "1",
                chain_id: 1337,
                verifying_contract: Address::ZERO,
            }
        }
        fn struct_hash(&self) -> B256 {
            self.eip712_hash_struct()
        }
    }
}
