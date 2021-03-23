use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::tokens::TokensHuman;
use cosmwasm_bignumber::{Decimal256, Uint256};
use cosmwasm_std::HumanAddr;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct InitMsg {
    /// Initial owner address
    pub owner_addr: HumanAddr,
    /// Oracle contract address for collateral tokens
    pub oracle_contract: HumanAddr,
    /// Market contract address to receive missing interest buffer
    pub market_contract: HumanAddr,
    /// Liquidation model contract address to compute liquidation amount
    pub liquidation_contract: HumanAddr,
    /// Collector contract address which is purchasing ANC token
    pub collector_contract: HumanAddr,
    /// The base denomination used when fetching oracle price,
    /// reward distribution, and borrow
    pub stable_denom: String,
    /// # of blocks per epoch period
    pub epoch_period: u64,
    /// Distribute interest buffer to market contract,
    /// when deposit_rate < threshold_deposit_rate
    pub threshold_deposit_rate: Decimal256,
    /// Target deposit rate.
    /// When current deposit rate is bigger than this,
    /// Custody contracts send rewards to interest buffer
    pub target_deposit_rate: Decimal256,
    /// Ratio to be distributed from the interest buffer
    pub buffer_distribution_factor: Decimal256,
    /// Ratio to be used for purchasing ANC token from the interest buffer
    pub anc_purchase_factor: Decimal256,
    /// Valid oracle price timeframe
    pub price_timeframe: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum HandleMsg {
    ////////////////////
    /// Owner operations
    ////////////////////

    /// Update Configs
    UpdateConfig {
        owner_addr: Option<HumanAddr>,
        oracle_contract: Option<HumanAddr>,
        liquidation_contract: Option<HumanAddr>,
        threshold_deposit_rate: Option<Decimal256>,
        target_deposit_rate: Option<Decimal256>,
        buffer_distribution_factor: Option<Decimal256>,
        anc_purchase_factor: Option<Decimal256>,
        epoch_period: Option<u64>,
        price_timeframe: Option<u64>,
    },

    /// Create new custody contract for the given collateral token
    Whitelist {
        name: String,                // bAsset name
        symbol: String,              // bAsset symbol
        collateral_token: HumanAddr, // bAsset token contract
        custody_contract: HumanAddr, // bAsset custody contract
        max_ltv: Decimal256,         // Loan To Value ratio
    },
    /// Update registered whitelist info
    UpdateWhitelist {
        collateral_token: HumanAddr,         // bAsset token contract
        custody_contract: Option<HumanAddr>, // bAsset custody contract
        max_ltv: Option<Decimal256>,         // Loan To Value ratio
    },

    /// Claims all staking rewards from the bAsset contracts
    /// and also do a epoch basis updates
    /// 1. Distribute interest buffers to depositors
    /// 2. Invoke [Custody] DistributeRewards
    /// 3. Update epoch state
    ExecuteEpochOperations {},
    UpdateEpochState {
        interest_buffer: Uint256,
    },

    ////////////////////
    /// User operations
    ////////////////////
    LockCollateral {
        collaterals: TokensHuman, // <(Collateral Token, Amount)>
    },
    UnlockCollateral {
        collaterals: TokensHuman, // <(Collateral Token, Amount)>
    },

    /////////////////////////////
    /// Permissionless operations
    /////////////////////////////
    LiquidateCollateral {
        borrower: HumanAddr,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    Config {},
    EpochState {},
    Whitelist {
        collateral_token: Option<HumanAddr>,
        start_after: Option<HumanAddr>,
        limit: Option<u32>,
    },
    Collaterals {
        borrower: HumanAddr,
    },
    AllCollaterals {
        start_after: Option<HumanAddr>,
        limit: Option<u32>,
    },
    BorrowLimit {
        borrower: HumanAddr,
        block_time: Option<u64>,
    },
}

// We define a custom struct for each query response
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ConfigResponse {
    pub owner_addr: HumanAddr,
    pub oracle_contract: HumanAddr,
    pub market_contract: HumanAddr,
    pub liquidation_contract: HumanAddr,
    pub collector_contract: HumanAddr,
    pub threshold_deposit_rate: Decimal256,
    pub target_deposit_rate: Decimal256,
    pub buffer_distribution_factor: Decimal256,
    pub anc_purchase_factor: Decimal256,
    pub stable_denom: String,
    pub epoch_period: u64,
    pub price_timeframe: u64,
}

// We define a custom struct for each query response
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct WhitelistResponseElem {
    pub name: String,
    pub symbol: String,
    pub max_ltv: Decimal256,
    pub custody_contract: HumanAddr,
    pub collateral_token: HumanAddr,
}

// We define a custom struct for each query response
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct WhitelistResponse {
    pub elems: Vec<WhitelistResponseElem>,
}

// We define a custom struct for each query response
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct CollateralsResponse {
    pub borrower: HumanAddr,
    pub collaterals: TokensHuman, // <(Collateral Token, Amount)>
}

// We define a custom struct for each query response
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct AllCollateralsResponse {
    pub all_collaterals: Vec<CollateralsResponse>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct BorrowLimitResponse {
    pub borrower: HumanAddr,
    pub borrow_limit: Uint256,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct MigrateMsg {
    pub target_deposit_rate: Decimal256,
    pub threshold_deposit_rate: Decimal256,
}
