use crate::common::*;
use crate::{Helius, ProgramName, Source, TokenStandard, TransactionContext, TransactionType};
use serde::{Deserialize, Serialize};
use serde_json::Number;

impl Helius {
    pub fn parse_transaction(
        &self,
        transactions: &ParseTransactionsRequest
    ) -> reqwest::Result<Vec<EnrichedTransaction>> {
        return self.handler.post(self.get_url_v0("transactions"), transactions);
    }
}

serializable_camel_case! {
    pub struct EnrichedTransaction {
        pub description: String,
        #[serde(rename="type")] // so we don't shadow a keyword
        pub transaction_type: TransactionType,
        pub source: Source,
        pub fee: Number,
        pub fee_payer: String,
        pub signature: String,
        pub slot: Number,
        pub native_transfers: Option<Vec<NativeTransfer>>,
        pub token_transfers: Option<Vec<TokenTransfer>>,
        pub account_data: Vec<AccountData>,
        pub transaction_error: Option<TransactionError>,
        pub instructions: Vec<Instruction>,
        pub events: TransactionEvent
    }
}

serializable! {
    pub struct ParseTransactionsRequest {
        pub transactions: Vec<String>
    }
}

serializable! {
    pub struct TransactionEvent {
        pub nft: Option<NFTEvent>,
        pub swap: Option<SwapEvent>,
        pub compressed: Option<CompressedNftEvent>
    }
}

serializable_camel_case! {
    pub struct CompressedNftEvent {
        #[serde(rename="type")]
        pub transaction_type: TransactionType,
        pub tree_id: String,
        pub leaf_index: Option<Number>,
        pub seq: Option<Number>,
        pub asset_id: Option<String>,
        pub instruction_index: Option<Number>,
        pub inner_instruction_index: Option<Number>,
        pub new_leaf_owner: Option<String>,
        pub old_leaf_owner: Option<String>
    }
}

serializable_camel_case! {
    pub struct SwapEvent {
        pub native_input: NativeBalanceChange,
        pub native_output: NativeBalanceChange,
        pub token_inputs: Vec<TokenBalanceChange>,
        pub token_outputs: Vec<TokenBalanceChange>,
        pub token_fees: Vec<TokenBalanceChange>,
        pub native_fees: Vec<NativeBalanceChange>,
        pub inner_swaps: Vec<TokenSwap>
    }
}

serializable_camel_case! {
    pub struct TokenSwap {
        pub native_input: Option<NativeTransfer>,
        pub native_output: Option<NativeTransfer>,
        pub token_inputs: Vec<TokenTransfer>,
        pub token_outputs: Vec<TokenTransfer>,
        pub token_fees: Vec<TokenTransfer>,
        pub native_fees: Vec<NativeTransfer>,
        pub program_info: ProgramInfo
    }
}

serializable_camel_case! {
    pub struct ProgramInfo {
        pub source: Source,
        pub account: String,
        pub program_name: ProgramName,
        pub instruction_name: String
    }
}

serializable_camel_case! {
    pub struct NFTEvent {
        pub seller: String,
        pub buyer: String,
        pub timestamp: Number,
        pub amount: Number,
        pub fee: Number,
        pub signature: String,
        pub source: Source,
        #[serde(rename="type")]
        pub transaction_type: TransactionType,
        pub sale_type: TransactionContext,
        pub nfts: Vec<Token>
    }
}

serializable_camel_case! {
    pub struct Token {
        pub mint: String,
        pub token_standard: TokenStandard
    }
}

serializable! {
    pub struct TransactionError {
        pub error: String
    }
}

serializable! {
    pub struct NativeBalanceChange {
        pub account: String,
        pub amount: Number
    }
}

serializable_camel_case! {
    pub struct AccountData {
        pub account: String,
        pub native_balance_change: Number,
        pub token_balance_changes: Option<Vec<TokenBalanceChange>>
    }
}

serializable_camel_case! {
    pub struct TokenBalanceChange {
        pub user_account: String,
        pub token_account: String,
        pub raw_token_amount: RawTokenAmount,
        pub mint: String
    }
}

serializable_camel_case! {
    pub struct RawTokenAmount {
        pub token_amount: String,
        pub decimals: Number
    }
}

serializable_camel_case! {
    pub struct TokenTransfer {
        #[serde(flatten)]
        pub user_accounts: TransferUserAccounts,
        pub from_token_account: Option<String>,
        pub to_token_account: Option<String>,
        pub token_amount: Number,
        pub token_standard: TokenStandard,
        pub mint: String
    }
}

serializable_camel_case! {
    pub struct TransferUserAccounts {
        pub from_user_account: Option<String>,
        pub to_user_account: Option<String>,
    }
}

serializable_camel_case! {
    pub struct NativeTransfer {
        #[serde(flatten)]
        pub user_accounts: TransferUserAccounts,
        pub amount: Number
    }
}

serializable_camel_case! {
    pub struct Instruction {
        pub accounts: Vec<String>,
        pub data: String,
        pub program_id: String,
        pub inner_instructions: Vec<InnerInstruction>
    }
}

serializable_camel_case! {
    pub struct InnerInstruction {
        pub accounts: Vec<String>,
        pub data: String,
        pub program_id: String
    }
}
