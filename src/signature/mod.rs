mod agent;
mod create_signature;

pub(crate) use create_signature::{sign_typed_data, sign_typed_data_multi_sig};

// Public API for signature collection
pub use agent::*;
pub use create_signature::{
    sign_l1_action, sign_multi_sig_action, sign_multi_sig_l1_action_payload,
    sign_multi_sig_l1_action_single, sign_multi_sig_user_signed_action_single,
};
