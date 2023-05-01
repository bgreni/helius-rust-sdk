use derive_alias::derive_alias;
use serde::{Deserialize, Serialize};

use std::ops::BitOr;

pub(crate) use serde_enum_str::{Deserialize_enum_str, Serialize_enum_str};


derive_alias! {
    serializable => #[derive(Serialize, Deserialize, Clone, Debug)]
    enum_serializable => #[derive(Deserialize_enum_str, Serialize_enum_str, Debug, PartialEq, Eq, Clone)]
}

macro_rules! serializable_camel_case {
    ($i:item) => {
        #[derive(Serialize, Deserialize, Clone, Debug)]
        #[serde(rename_all="camelCase")]
        $i
    }
}

#[allow(clippy::single_component_path_imports)]
pub(crate) use {enum_serializable, serializable};

#[derive(Clone, Copy)]
pub enum Cluster {
    MainnetBeta,
    Devnet,
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
    #[serde(rename_all="camelCase")]
    pub enum AccountWebhookEncoding {
        JsonParsed
    }
}

enum_serializable! {
    #[serde(rename_all="lowercase")]
    pub enum TxnStatus {
        All,
        Success,
        Failed
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
    pub enum ProgramName {
        Unkown,
        JupiterV1,
        JupiterV2,
        JupiterV3,
        JupiterV4,
        MercurialStableSwap,
        SaberStableSwap,
        SaberExchange,
        SerumDexV1,
        SerumDexV2,
        SerumDexV3,
        SerumSwap,
        StepFinance,
        Cropper,
        RaydiumLiquidityPoolV2,
        RaydiumLiquidityPoolV3,
        RaydiumLiquidityPoolV4,
        AldrinAmmV1,
        AldrinAmmV2,
        Crema,
        Lifinity,
        LifinityV2,
        Cykura,
        OrcaTokenSwapV1,
        OrcaTokenSwapV2,
        OrcaWhirlpools,
        Marinade,
        Stepn,
        SenchaExchange,
        SarosAmm,
        FoxyStake,
        FoxySwap,
        FoxyRaffle,
        FoxyTokenMarket,
        FoxyMissions,
        FoxyMarmalade,
        FoxyCoinflip,
        FoxyAuction,
        Citrus,
        HadeSwap,
        Zeta,
        CardinalRent,
        CardinalStaking,
        SharkyFi,
        OpenCreatorProtocol,
        Bubblegum,
        CoralCube,
        #[serde(other)]
        Other(String)
    }
}

enum_serializable! {
    #[allow(non_camel_case_types)]
    #[serde(rename_all="SCREAMING_SNAKE_CASE")]
    pub enum Source {
        FormFunction,
        ExchangeArt,
        CandyMachineV3,
        CandyMachineV2,
        CandyMachineV1,
        Unknown,
        Solanart,
        Solsea,
        MagicEden,
        Holaplex,
        Metaplex,
        Opensea,
        SolanaProgramLibrary,
        Anchor,
        Phantom,
        SystemProgram,
        StakeProgram,
        Coinbase,
        CoralCube,
        Hedge,
        LaunchMyNft,
        GemBank,
        GemFarm,
        Degods,
        Bsl,
        Yawww,
        Atadia,
        DigitalEyes,
        Hyperspace,
        Tensor,
        Bifrost,
        Jupiter,
        Mecurial,
        Saber,
        Serum,
        StepFinance,
        Cropper,
        Raydium,
        Aldrin,
        Crema,
        Lifinity,
        Cykura,
        Orca,
        Marinade,
        Stepn,
        Sencha,
        Saros,
        EnglishAuction,
        Foxy,
        Hadeswap,
        FoxyStaking,
        FoxyRaffle,
        FoxyTokenMarket,
        FoxyMissions,
        FoxyMarmalade,
        FoxyCoinflip,
        FoxyAuction,
        Citrus,
        Zeta,
        Elixir,
        ElixirLaunchpad,
        CardinalRent,
        CardinalStaking,
        BpfLoader,
        BpfUpgradeableLoader,
        Squads,
        SharkyFi,
        OpenCreatorProtocol,
        Bubblegum,
        // Mints
        W_SOL,
        DUST,
        SOLI,
        USDC,
        FLWR,
        HDG,
        MEAN,
        UXD,
        SHDW,
        POLIS,
        ATLAS,
        USH,
        TRTLS,
        RUNNER,
        INVICTUS,
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
