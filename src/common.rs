use std::ops::BitOr;
use serde::{Deserialize, Serialize};
use serde_enum_str::{Deserialize_enum_str, Serialize_enum_str};
use derive_alias::derive_alias;

derive_alias! {
    serializable => #[derive(Serialize, Deserialize, Clone, Debug)]
    enum_serializable => #[derive(Deserialize_enum_str, Serialize_enum_str, Debug, PartialEq, Eq, Clone)]
}
pub(crate) use serializable;

#[derive(Clone, Copy)]
pub enum Cluster {
    MainnetBeta,
    Devnet
}

serializable! {
    pub struct HeliusOptions {
        pub limit: Option<u32>,
        #[serde(rename = "paginationToken")]
        pub pagination_token: Option<String>
    }
}

serializable! {
    pub enum CollectionIdentifier {
        #[serde(rename = "firstVerifiedCreators")]
        FirstVerifiedCreators(Vec<String>),
        #[serde(rename = "verifiedCollectionAddress")]
        VerifiedCollectionAddress(Vec<String>)
    }
}

enum_serializable! {
    #[serde(rename_all="SCREAMING_SNAKE_CASE")]
    pub enum TransactionContext {
        Auction,
        InstantSale,
        Offer,
        GlobalOffer,
        Mint,
        Unknown,
        #[serde(other)]
        Other(String)
    }
}

enum_serializable! {
    pub enum TokenStandard {
        ProgrammableNonFungible,
        NonFungible,
        Fungible,
        FungibleAsset,
        NonFungibleEdition,
        UnknownStandard,
        #[serde(other)]
        Other(String)
    }
}

enum_serializable! {
    #[serde(rename_all="SCREAMING_SNAKE_CASE")]
    pub enum TransactionType {
        Unknown,
        NftBid,
        NftBidCancelled,
        NftListing,
        NftCancelListing,
        NftSale,
        NftMint,
        NftAuctionCreated,
        NftAuctionUpdated,
        NftAuctionCancelled,
        NftParticipationReward,
        NftMintRejected,
        CreateStore,
        WhitelistCreator,
        AddToWhitelist,
        RemoveFromWhitelist,
        AuctionManagerClaimBid,
        EmptyPaymentAccount,
        UpdatePrimarySaleMetadata,
        AddTokenToVault,
        ActivateVault,
        InitVault,
        InitBank,
        InitStake,
        MergeStake,
        SplitStake,
        SetBankFlags,
        SetVaultLock,
        UpdateVaultOwner,
        UpdateBankManager,
        RecordRarityPoints,
        AddRaritiesToBank,
        InitFarm,
        InitFarmer,
        RefreshFarmer,
        UpdateFarm,
        AuthorizeFunder,
        DeauthorizeFunder,
        FundReward,
        CancelReward,
        LockReward,
        Payout,
        ValidateSafetyDepositBoxV2,
        SetAuthority,
        InitAuctionManagerV2,
        UpdateExternalPriceAccount,
        AuctionHouseCreate,
        CloseEscrowAccount,
        Withdraw,
        Deposit,
        Transfer,
        Burn,
        BurnNft,
        PlatformFee,
        Loan,
        RepayLoan,
        AddToPool,
        RemoveFromPool,
        ClosePosition,
        Unlabeled,
        CloseAccount,
        WithdrawGem,
        DepositGem,
        StakeToken,
        UnstakeToken,
        StakeSol,
        UnstakeSol,
        ClaimRewards,
        BuySubscription,
        Swap,
        InitSwap,
        CancelSwap,
        RejectSwap,
        InitializeAccount,
        TokenMint,
        CreateApparaisal,
        Fuse,
        DepositFractionalPool,
        Fractionalize,
        CreateRaffle,
        BuyTickets,
        UpdateItem,
        ListItem,
        DelistItem,
        AddItem,
        CloseItem,
        BuyItem,
        FillOrder,
        UpdateOrder,
        CreateOrder,
        CloseOrder,
        CancelOrder,
        KickItem,
        UpgradeFox,
        UpgradeFoxRequest,
        LoanFox,
        BorrowBox,
        SwitchFoxRequest,
        SwitchFox,
        CreateEscrow,
        AcceptRequeestArtist,
        CancelEscrow,
        AcceptEscrowArtist,
        AcceptEscrowUser,
        PlaceBet,
        PlaceSolBet,
        CreateBet,
        NftRentUpdateListing,
        NftRentActivate,
        NftRentCancelListing,
        NftRentListing,
        FinalizeProgramInstruction,
        UpgradeProgramInstruction,
        NftGlobalBix,
        NftGlobalBidCancel,
        ExecuteTransaction,
        ApproveTransaction,
        CreateTransaction,
        RejectTransaction,
        CancelTransaction,
        AddInstruction,
        AttachMetadata,
        RequestPnftMigration,
        StartPnftMigration,
        MigrateToPnft,
        UpdateRaffle,
        #[serde(other)]
        Other(String)
    }
}
impl BitOr<TransactionType> for TransactionType {
    type Output = Vec<Self>;

    fn bitor(self, rhs: Self) -> Self::Output {
        return vec![self, rhs];
    }
}

impl BitOr<Vec<TransactionType>> for TransactionType {
    type Output = Vec<Self>;
    fn bitor(self, mut rhs: Vec<TransactionType>) -> Self::Output {
        rhs.push(self);
        return rhs;
    }
}

impl BitOr<TransactionType> for Vec<TransactionType> {
    type Output = Self;

    fn bitor(mut self, rhs: TransactionType) -> Self::Output {
        self.push(rhs);
        return self;
    }
}
