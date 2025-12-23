use alloy::{
    dyn_abi::Eip712Domain,
    primitives::{keccak256, Address, B256},
    sol_types::{eip712_domain, SolValue},
};
use serde::{Deserialize, Serialize, Serializer};

use super::{cancel::CancelRequestCloid, BuilderInfo};
use crate::helpers::next_nonce;
use crate::{
    eip712::Eip712,
    exchange::{cancel::CancelRequest, modify::ModifyRequest, order::OrderRequest},
    HyperliquidChain,
};

fn eip_712_domain(chain_id: u64) -> Eip712Domain {
    eip712_domain! {
        name: "HyperliquidSignTransaction",
        version: "1",
        chain_id: chain_id,
        verifying_contract: Address::ZERO,
    }
}

fn serialize_hex<S>(val: &u64, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    s.serialize_str(&format!("0x{val:x}"))
}

fn default_signature_chain_id(
    hyperliquid_chain: &HyperliquidChain,
    signature_chain_id: Option<u64>,
) -> u64 {
    match signature_chain_id {
        Some(signature_chain_id) => signature_chain_id,
        None => {
            if hyperliquid_chain.is_mainnet() {
                42161 // Arbitrum One
            } else {
                421614 // Arbitrum Sepolia
            }
        }
    }
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct MultiSigExtension {
    pub payload_multi_sig_user: String,
    pub outer_signer: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct UsdSend {
    #[serde(serialize_with = "serialize_hex")]
    pub signature_chain_id: u64,
    pub hyperliquid_chain: String,
    pub destination: String,
    pub amount: String,
    pub time: u64,
}

impl UsdSend {
    pub fn new(
        hyperliquid_chain: HyperliquidChain,
        destination: String,
        amount: String,
        signature_chain_id: Option<u64>,
    ) -> Self {
        let signature_chain_id = default_signature_chain_id(&hyperliquid_chain, signature_chain_id);
        Self {
            signature_chain_id,
            hyperliquid_chain: hyperliquid_chain.action_chain_name(),
            destination,
            amount,
            time: next_nonce(),
        }
    }
}

impl Eip712 for UsdSend {
    fn domain(&self) -> Eip712Domain {
        eip_712_domain(self.signature_chain_id)
    }

    fn struct_hash(&self) -> B256 {
        let items = (
            keccak256("HyperliquidTransaction:UsdSend(string hyperliquidChain,string destination,string amount,uint64 time)"),
            keccak256(&self.hyperliquid_chain),
            keccak256(&self.destination),
            keccak256(&self.amount),
            &self.time
        );
        keccak256(items.abi_encode())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UpdateLeverage {
    pub asset: u32,
    pub is_cross: bool,
    pub leverage: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UpdateIsolatedMargin {
    pub asset: u32,
    pub is_buy: bool,
    pub ntli: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BulkOrder {
    pub orders: Vec<OrderRequest>,
    pub grouping: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub builder: Option<BuilderInfo>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BulkCancel {
    pub cancels: Vec<CancelRequest>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BulkModify {
    pub modifies: Vec<ModifyRequest>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BulkCancelCloid {
    pub cancels: Vec<CancelRequestCloid>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ApproveAgent {
    #[serde(serialize_with = "serialize_hex")]
    pub signature_chain_id: u64,
    pub hyperliquid_chain: String,
    pub agent_address: Address,
    pub agent_name: Option<String>,
    pub nonce: u64,
}

impl ApproveAgent {
    pub fn new(
        hyperliquid_chain: HyperliquidChain,
        agent_address: Address,
        agent_name: Option<String>,
        signature_chain_id: Option<u64>,
    ) -> Self {
        let signature_chain_id = default_signature_chain_id(&hyperliquid_chain, signature_chain_id);
        Self {
            signature_chain_id,
            hyperliquid_chain: hyperliquid_chain.action_chain_name(),
            agent_address,
            agent_name,
            nonce: next_nonce(),
        }
    }
}

impl Eip712 for ApproveAgent {
    fn domain(&self) -> Eip712Domain {
        eip_712_domain(self.signature_chain_id)
    }

    fn struct_hash(&self) -> B256 {
        let items = (
            keccak256("HyperliquidTransaction:ApproveAgent(string hyperliquidChain,address agentAddress,string agentName,uint64 nonce)"),
            keccak256(&self.hyperliquid_chain),
            &self.agent_address,
            keccak256(self.agent_name.as_deref().unwrap_or("")),
            &self.nonce
        );
        keccak256(items.abi_encode())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Withdraw3 {
    #[serde(serialize_with = "serialize_hex")]
    pub signature_chain_id: u64,
    pub hyperliquid_chain: String,
    pub destination: String,
    pub amount: String,
    pub time: u64,
}

impl Withdraw3 {
    pub fn new(
        hyperliquid_chain: HyperliquidChain,
        destination: String,
        amount: String,
        signature_chain_id: Option<u64>,
    ) -> Self {
        let signature_chain_id = default_signature_chain_id(&hyperliquid_chain, signature_chain_id);
        Self {
            signature_chain_id,
            hyperliquid_chain: hyperliquid_chain.action_chain_name(),
            destination,
            amount,
            time: next_nonce(),
        }
    }
}

impl Eip712 for Withdraw3 {
    fn domain(&self) -> Eip712Domain {
        eip_712_domain(self.signature_chain_id)
    }

    fn struct_hash(&self) -> B256 {
        let items = (
            keccak256("HyperliquidTransaction:Withdraw(string hyperliquidChain,string destination,string amount,uint64 time)"),
            keccak256(&self.hyperliquid_chain),
            keccak256(&self.destination),
            keccak256(&self.amount),
            &self.time,
        );
        keccak256(items.abi_encode())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SpotSend {
    #[serde(serialize_with = "serialize_hex")]
    pub signature_chain_id: u64,
    pub hyperliquid_chain: String,
    pub destination: String,
    pub token: String,
    pub amount: String,
    pub time: u64,
}

impl SpotSend {
    pub fn new(
        hyperliquid_chain: HyperliquidChain,
        destination: String,
        token: String,
        amount: String,
        signature_chain_id: Option<u64>,
    ) -> Self {
        let signature_chain_id = default_signature_chain_id(&hyperliquid_chain, signature_chain_id);
        Self {
            signature_chain_id,
            hyperliquid_chain: hyperliquid_chain.action_chain_name(),
            destination,
            token,
            amount,
            time: next_nonce(),
        }
    }
}

impl Eip712 for SpotSend {
    fn domain(&self) -> Eip712Domain {
        eip_712_domain(self.signature_chain_id)
    }

    fn struct_hash(&self) -> B256 {
        let items = (
            keccak256("HyperliquidTransaction:SpotSend(string hyperliquidChain,string destination,string token,string amount,uint64 time)"),
            keccak256(&self.hyperliquid_chain),
            keccak256(&self.destination),
            keccak256(&self.token),
            keccak256(&self.amount),
            &self.time,
        );
        keccak256(items.abi_encode())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SpotUser {
    pub class_transfer: ClassTransfer,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ClassTransfer {
    pub usdc: u64,
    pub to_perp: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SendAsset {
    #[serde(serialize_with = "serialize_hex")]
    pub signature_chain_id: u64,
    pub hyperliquid_chain: String,
    pub destination: String,
    pub source_dex: String,
    pub destination_dex: String,
    pub token: String,
    pub amount: String,
    pub from_sub_account: String,
    pub nonce: u64,
    #[serde(skip)]
    pub multi_sig_ext: Option<MultiSigExtension>,
}

impl SendAsset {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        hyperliquid_chain: HyperliquidChain,
        destination: String,
        source_dex: String,
        destination_dex: String,
        token: String,
        amount: String,
        from_sub_account: String,
        multi_sig_ext: Option<MultiSigExtension>,
        signature_chain_id: Option<u64>,
    ) -> Self {
        let signature_chain_id = default_signature_chain_id(&hyperliquid_chain, signature_chain_id);
        Self {
            signature_chain_id,
            hyperliquid_chain: hyperliquid_chain.action_chain_name(),
            destination,
            source_dex,
            destination_dex,
            token,
            amount,
            from_sub_account,
            nonce: next_nonce(),
            multi_sig_ext,
        }
    }
}

impl Eip712 for SendAsset {
    fn domain(&self) -> Eip712Domain {
        eip_712_domain(self.signature_chain_id)
    }

    fn struct_hash(&self) -> B256 {
        if let Some(multi_sig_ext) = &self.multi_sig_ext {
            let multi_sig_user: Address = multi_sig_ext.payload_multi_sig_user.parse().unwrap();
            let outer_signer: Address = multi_sig_ext.outer_signer.parse().unwrap();

            let items = (
                keccak256("HyperliquidTransaction:SendAsset(string hyperliquidChain,address payloadMultiSigUser,address outerSigner,string destination,string sourceDex,string destinationDex,string token,string amount,string fromSubAccount,uint64 nonce)"),
                keccak256(&self.hyperliquid_chain),
                multi_sig_user,
                outer_signer,
                keccak256(&self.destination),
                keccak256(&self.source_dex),
                keccak256(&self.destination_dex),
                keccak256(&self.token),
                keccak256(&self.amount),
                keccak256(&self.from_sub_account),
                &self.nonce,
            );
            keccak256(items.abi_encode())
        } else {
            let items = (
                keccak256("HyperliquidTransaction:SendAsset(string hyperliquidChain,string destination,string sourceDex,string destinationDex,string token,string amount,string fromSubAccount,uint64 nonce)"),
                keccak256(&self.hyperliquid_chain),
                keccak256(&self.destination),
                keccak256(&self.source_dex),
                keccak256(&self.destination_dex),
                keccak256(&self.token),
                keccak256(&self.amount),
                keccak256(&self.from_sub_account),
                &self.nonce,
            );
            keccak256(items.abi_encode())
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct VaultTransfer {
    pub vault_address: Address,
    pub is_deposit: bool,
    pub usd: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SetReferrer {
    pub code: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct EvmUserModify {
    pub using_big_blocks: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ApproveBuilderFee {
    #[serde(serialize_with = "serialize_hex")]
    pub signature_chain_id: u64,
    pub hyperliquid_chain: String,
    pub builder: Address,
    pub max_fee_rate: String,
    pub nonce: u64,
}

impl ApproveBuilderFee {
    pub fn new(
        hyperliquid_chain: HyperliquidChain,
        builder: Address,
        max_fee_rate: String,
        signature_chain_id: Option<u64>,
    ) -> Self {
        let signature_chain_id = default_signature_chain_id(&hyperliquid_chain, signature_chain_id);
        Self {
            signature_chain_id,
            hyperliquid_chain: hyperliquid_chain.action_chain_name(),
            builder,
            max_fee_rate,
            nonce: next_nonce(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ScheduleCancel {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<u64>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ClaimRewards;

impl Eip712 for ApproveBuilderFee {
    fn domain(&self) -> Eip712Domain {
        eip_712_domain(self.signature_chain_id)
    }

    fn struct_hash(&self) -> B256 {
        let items = (
            keccak256("HyperliquidTransaction:ApproveBuilderFee(string hyperliquidChain,string maxFeeRate,address builder,uint64 nonce)"),
            keccak256(&self.hyperliquid_chain),
            keccak256(&self.max_fee_rate),
            &self.builder,
            &self.nonce,
        );
        keccak256(items.abi_encode())
    }
}

// Multi-sig related structs

/// Convert a regular user account to a multi-sig account
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ConvertToMultiSig {
    #[serde(serialize_with = "serialize_hex")]
    pub signature_chain_id: u64,
    pub hyperliquid_chain: String,
    pub multi_sig_threshold: u64,
    pub time: u64,
}

impl ConvertToMultiSig {
    pub fn new(
        hyperliquid_chain: HyperliquidChain,
        multi_sig_threshold: u64,
        signature_chain_id: Option<u64>,
    ) -> Self {
        let signature_chain_id = default_signature_chain_id(&hyperliquid_chain, signature_chain_id);
        Self {
            signature_chain_id,
            hyperliquid_chain: hyperliquid_chain.action_chain_name(),
            multi_sig_threshold,
            time: next_nonce(),
        }
    }
}

impl Eip712 for ConvertToMultiSig {
    fn domain(&self) -> Eip712Domain {
        eip_712_domain(self.signature_chain_id)
    }

    fn struct_hash(&self) -> B256 {
        let items = (
            keccak256("HyperliquidTransaction:ConvertToMultiSig(string hyperliquidChain,uint64 multiSigThreshold,uint64 time)"),
            keccak256(&self.hyperliquid_chain),
            &self.multi_sig_threshold,
            &self.time,
        );
        keccak256(items.abi_encode())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct UpdateMultiSigAddresses {
    #[serde(serialize_with = "serialize_hex")]
    pub signature_chain_id: u64,
    pub hyperliquid_chain: String,
    pub to_add: Vec<Address>,
    pub to_remove: Vec<Address>,
    pub time: u64,
}

impl UpdateMultiSigAddresses {
    pub fn new(
        hyperliquid_chain: HyperliquidChain,
        to_add: Vec<Address>,
        to_remove: Vec<Address>,
        signature_chain_id: Option<u64>,
    ) -> Self {
        let signature_chain_id = default_signature_chain_id(&hyperliquid_chain, signature_chain_id);
        Self {
            signature_chain_id,
            hyperliquid_chain: hyperliquid_chain.action_chain_name(),
            to_add,
            to_remove,
            time: next_nonce(),
        }
    }
}

impl Eip712 for UpdateMultiSigAddresses {
    fn domain(&self) -> Eip712Domain {
        eip_712_domain(self.signature_chain_id)
    }

    fn struct_hash(&self) -> B256 {
        let to_add_encoded = self.to_add.iter().fold(B256::ZERO, |acc, addr| {
            keccak256([acc.as_slice(), addr.as_slice()].concat())
        });
        let to_remove_encoded = self.to_remove.iter().fold(B256::ZERO, |acc, addr| {
            keccak256([acc.as_slice(), addr.as_slice()].concat())
        });

        let items = (
            keccak256("HyperliquidTransaction:UpdateMultiSigAddresses(string hyperliquidChain,address[] toAdd,address[] toRemove,uint64 time)"),
            keccak256(&self.hyperliquid_chain),
            to_add_encoded,
            to_remove_encoded,
            &self.time,
        );
        keccak256(items.abi_encode())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MultiSigEnvelope {
    #[serde(serialize_with = "serialize_hex")]
    pub signature_chain_id: u64,
    pub hyperliquid_chain: String,
    pub multi_sig_action_hash: B256,
    pub nonce: u64,
}

impl Eip712 for MultiSigEnvelope {
    fn domain(&self) -> Eip712Domain {
        eip_712_domain(self.signature_chain_id)
    }

    fn struct_hash(&self) -> B256 {
        let items = (
            keccak256("HyperliquidTransaction:SendMultiSig(string hyperliquidChain,bytes32 multiSigActionHash,uint64 nonce)"),
            keccak256(&self.hyperliquid_chain),
            &self.multi_sig_action_hash,
            &self.nonce,
        );
        keccak256(items.abi_encode())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::Result;
    use crate::HyperliquidChain;
    use alloy::primitives::address;

    #[test]
    fn test_usd_send_new_helper() -> Result<()> {
        let with_helper = UsdSend::new(
            HyperliquidChain::Testnet,
            "0x0D1d9635D0640821d15e323ac8AdADfA9c111414".to_string(),
            "1".to_string(),
            None,
        );
        let time = with_helper.time;

        let manual = UsdSend {
            signature_chain_id: 421614,
            hyperliquid_chain: "Testnet".to_string(),
            destination: "0x0D1d9635D0640821d15e323ac8AdADfA9c111414".to_string(),
            amount: "1".to_string(),
            time,
        };

        assert_eq!(manual, with_helper);

        Ok(())
    }

    #[test]
    fn test_withdraw3_new_helper() -> Result<()> {
        let with_helper = Withdraw3::new(
            HyperliquidChain::Testnet,
            "0x0D1d9635D0640821d15e323ac8AdADfA9c111414".to_string(),
            "1".to_string(),
            None,
        );
        let time = with_helper.time;

        let manual = Withdraw3 {
            signature_chain_id: 421614,
            hyperliquid_chain: "Testnet".to_string(),
            destination: "0x0D1d9635D0640821d15e323ac8AdADfA9c111414".to_string(),
            amount: "1".to_string(),
            time,
        };

        assert_eq!(manual, with_helper);

        Ok(())
    }

    #[test]
    fn test_approve_builder_fee_new_helper() -> Result<()> {
        let with_helper = ApproveBuilderFee::new(
            HyperliquidChain::Testnet,
            address!("0x1234567890123456789012345678901234567890"),
            "0.001%".to_string(),
            None,
        );
        let nonce = with_helper.nonce;

        let manual = ApproveBuilderFee {
            signature_chain_id: 421614,
            hyperliquid_chain: "Testnet".to_string(),
            builder: address!("0x1234567890123456789012345678901234567890"),
            max_fee_rate: "0.001%".to_string(),
            nonce,
        };

        assert_eq!(manual, with_helper);

        Ok(())
    }

    #[test]
    fn test_send_asset_new_helper() -> Result<()> {
        let with_helper = SendAsset::new(
            HyperliquidChain::Testnet,
            "0x1234567890123456789012345678901234567890".to_string(),
            "spot".to_string(),
            "".to_string(),
            "USDC".to_string(),
            "50".to_string(),
            "".to_string(),
            None,
            None,
        );
        let nonce = with_helper.nonce;

        let manual = SendAsset {
            signature_chain_id: 421614,
            hyperliquid_chain: "Testnet".to_string(),
            destination: "0x1234567890123456789012345678901234567890".to_string(),
            source_dex: "spot".to_string(),
            destination_dex: "".to_string(),
            token: "USDC".to_string(),
            amount: "50".to_string(),
            from_sub_account: "".to_string(),
            nonce,
            multi_sig_ext: None,
        };

        assert_eq!(manual, with_helper);

        Ok(())
    }

    #[test]
    fn test_convert_to_multi_sig_new_helper() -> Result<()> {
        let with_helper = ConvertToMultiSig::new(HyperliquidChain::Testnet, 1, None);
        let time = with_helper.time;

        let manual = ConvertToMultiSig {
            signature_chain_id: 421614,
            hyperliquid_chain: "Testnet".to_string(),
            multi_sig_threshold: 1,
            time,
        };

        assert_eq!(manual, with_helper);

        Ok(())
    }

    #[test]
    fn test_update_multi_sig_addresses_new_helper() -> Result<()> {
        let with_helper = UpdateMultiSigAddresses::new(
            HyperliquidChain::Testnet,
            vec![
                address!("0x0D1d9635D0640821d15e323ac8AdADfA9c111414"),
                address!("0x1234567890123456789012345678901234567890"),
            ],
            vec![address!("0xabcdabcdabcdabcdabcdabcdabcdabcdabcdabcd")],
            None,
        );
        let time = with_helper.time;

        let manual = UpdateMultiSigAddresses {
            signature_chain_id: 421614,
            hyperliquid_chain: "Testnet".to_string(),
            to_add: vec![
                address!("0x0D1d9635D0640821d15e323ac8AdADfA9c111414"),
                address!("0x1234567890123456789012345678901234567890"),
            ],
            to_remove: vec![address!("0xabcdabcdabcdabcdabcdabcdabcdabcdabcdabcd")],
            time,
        };

        assert_eq!(manual, with_helper);

        Ok(())
    }
}
