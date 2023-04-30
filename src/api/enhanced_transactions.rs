use crate::common::serializable;
use crate::{Helius, ProgramName, Source, TokenStandard, TransactionContext, TransactionType};
use serde::{Deserialize, Serialize};
use serde_json::Number;

pub trait EnhancedTransactionsApi {
    fn parse_transaction(
        &self,
        transactions: &ParseTransactionsRequest
    ) -> reqwest::Result<Vec<EnrichedTransaction>>;
}

impl EnhancedTransactionsApi for Helius {
    fn parse_transaction(
        &self,
        transactions: &ParseTransactionsRequest
    ) -> reqwest::Result<Vec<EnrichedTransaction>> {
        return self.http_client
            .post(self.get_url_v0("transactions"))
            .json(transactions)
            .send()?
            .error_for_status()?
            .json();
    }
}

serializable! {
    pub struct ParseTransactionsRequest {
        pub transactions: Vec<String>
    }
}

serializable! {
    #[serde(rename_all="camelCase")]
    pub struct EnrichedTransaction {
        description: String,
        #[serde(rename="type")] // so we don't shadow a keyword
        transaction_type: TransactionType,
        source: Source,
        fee: Number,
        fee_payer: String,
        signature: String,
        slot: Number,
        native_transfers: Option<Vec<NativeTransfer>>,
        token_transfers: Option<Vec<TokenTransfer>>,
        account_data: Vec<AccountData>,
        transaction_error: Option<TransactionError>,
        instructions: Vec<Instruction>,
        events: TransactionEvent
    }
}

serializable! {
    pub struct TransactionEvent {
        nft: Option<NFTEvent>,
        swap: Option<SwapEvent>,
        compressed: Option<CompressedNftEvent>
    }
}

serializable! {
    #[serde(rename_all="camelCase")]
    pub struct CompressedNftEvent {
        #[serde(rename="type")]
        transaction_type: TransactionType,
        tree_id: String,
        leaf_index: Option<Number>,
        seq: Option<Number>,
        asset_id: Option<String>,
        instruction_index: Option<Number>,
        inner_instruction_index: Option<Number>,
        new_leaf_owner: Option<String>,
        old_leaf_owner: Option<String>
    }
}

serializable! {
    #[serde(rename_all="camelCase")]
    pub struct SwapEvent {
        native_input: NativeBalanceChange,
        native_output: NativeBalanceChange,
        token_inputs: Vec<TokenBalanceChange>,
        token_outputs: Vec<TokenBalanceChange>,
        token_fees: Vec<TokenBalanceChange>,
        native_fees: Vec<NativeBalanceChange>,
        inner_swaps: Vec<TokenSwap>
    }
}

serializable! {
    #[serde(rename_all="camelCase")]
    pub struct TokenSwap {
        native_input: Option<NativeTransfer>,
        native_output: Option<NativeTransfer>,
        token_inputs: Vec<TokenTransfer>,
        token_outputs: Vec<TokenTransfer>,
        token_fees: Vec<TokenTransfer>,
        native_fees: Vec<NativeTransfer>,
        program_info: ProgramInfo
    }
}

serializable! {
    pub struct ProgramInfo {
        source: Source,
        account: String,
        program_name: ProgramName,
        instruction_name: String
    }
}

serializable! {
    #[serde(rename_all="camelCase")]
    pub struct NFTEvent {
        seller: String,
        buyer: String,
        timestamp: Number,
        amount: Number,
        fee: Number,
        signature: String,
        source: Source,
        #[serde(rename="type")]
        transaction_type: TransactionType,
        sale_type: TransactionContext,
        nfts: Vec<Token>
    }
}

serializable! {
    #[serde(rename_all="camelCase")]
    pub struct Token {
        mint: String,
        token_standard: TokenStandard
    }
}

serializable! {
    pub struct TransactionError {
        error: String
    }
}

serializable! {
    pub struct NativeBalanceChange {
        account: String,
        amount: Number
    }
}

serializable! {
    #[serde(rename_all="camelCase")]
    pub struct AccountData {
        account: String,
        native_balance_change: Number,
        token_balance_changes: Option<Vec<TokenBalanceChange>>
    }
}

serializable! {
    #[serde(rename_all="camelCase")]
    pub struct TokenBalanceChange {
        user_account: String,
        token_account: String,
        raw_token_amount: RawTokenAmount,
        mint: String
    }
}

serializable! {
    #[serde(rename_all="camelCase")]
    pub struct RawTokenAmount {
        token_amount: String,
        decimals: Number
    }
}

serializable! {
    #[serde(rename_all="camelCase")]
    pub struct TokenTransfer {
        #[serde(flatten)]
        user_accounts: TransferUserAccounts,
        from_token_account: Option<String>,
        to_token_account: Option<String>,
        token_amount: Number,
        token_standard: TokenStandard,
        mint: String
    }
}

serializable! {
    #[serde(rename_all="camelCase")]
    pub struct TransferUserAccounts {
        from_user_account: Option<String>,
        to_user_account: Option<String>,
    }
}

serializable! {
    #[serde(rename_all="camelCase")]
    pub struct NativeTransfer {
        #[serde(flatten)]
        user_accounts: TransferUserAccounts,
        amount: Number
    }
}

serializable! {
    #[serde(rename_all="camelCase")]
    pub struct Instruction {
        accounts: Vec<String>,
        data: String,
        program_id: String,
        inner_instructions: Vec<InnerInstruction>
    }
}

serializable! {
    #[serde(rename_all="camelCase")]
    pub struct InnerInstruction {
        accounts: Vec<String>,
        data: String,
        program_id: String
    }
}
