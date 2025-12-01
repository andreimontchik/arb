use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult,
    instruction::{AccountMeta, Instruction},
    program::{invoke, invoke_signed},
    pubkey::Pubkey, program_error::ProgramError,
};
use std::io::Read;
use crate::*;
#[derive(Clone, Debug, PartialEq)]
pub enum WhirlpoolProgramIx {
    InitializeConfig(InitializeConfigIxArgs),
    InitializePool(InitializePoolIxArgs),
    InitializeTickArray(InitializeTickArrayIxArgs),
    InitializeFeeTier(InitializeFeeTierIxArgs),
    InitializeReward(InitializeRewardIxArgs),
    SetRewardEmissions(SetRewardEmissionsIxArgs),
    OpenPosition(OpenPositionIxArgs),
    OpenPositionWithMetadata(OpenPositionWithMetadataIxArgs),
    IncreaseLiquidity(IncreaseLiquidityIxArgs),
    DecreaseLiquidity(DecreaseLiquidityIxArgs),
    UpdateFeesAndRewards,
    CollectFees,
    CollectReward(CollectRewardIxArgs),
    CollectProtocolFees,
    Swap(SwapIxArgs),
    ClosePosition,
    SetDefaultFeeRate(SetDefaultFeeRateIxArgs),
    SetDefaultProtocolFeeRate(SetDefaultProtocolFeeRateIxArgs),
    SetFeeRate(SetFeeRateIxArgs),
    SetProtocolFeeRate(SetProtocolFeeRateIxArgs),
    SetFeeAuthority,
    SetCollectProtocolFeesAuthority,
    SetRewardAuthority(SetRewardAuthorityIxArgs),
    SetRewardAuthorityBySuperAuthority(SetRewardAuthorityBySuperAuthorityIxArgs),
    SetRewardEmissionsSuperAuthority,
    TwoHopSwap(TwoHopSwapIxArgs),
    InitializePositionBundle,
    InitializePositionBundleWithMetadata,
    DeletePositionBundle,
    OpenBundledPosition(OpenBundledPositionIxArgs),
    CloseBundledPosition(CloseBundledPositionIxArgs),
}
impl WhirlpoolProgramIx {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        match maybe_discm {
            INITIALIZE_CONFIG_IX_DISCM => {
                Ok(
                    Self::InitializeConfig(
                        InitializeConfigIxArgs::deserialize(&mut reader)?,
                    ),
                )
            }
            INITIALIZE_POOL_IX_DISCM => {
                Ok(Self::InitializePool(InitializePoolIxArgs::deserialize(&mut reader)?))
            }
            INITIALIZE_TICK_ARRAY_IX_DISCM => {
                Ok(
                    Self::InitializeTickArray(
                        InitializeTickArrayIxArgs::deserialize(&mut reader)?,
                    ),
                )
            }
            INITIALIZE_FEE_TIER_IX_DISCM => {
                Ok(
                    Self::InitializeFeeTier(
                        InitializeFeeTierIxArgs::deserialize(&mut reader)?,
                    ),
                )
            }
            INITIALIZE_REWARD_IX_DISCM => {
                Ok(
                    Self::InitializeReward(
                        InitializeRewardIxArgs::deserialize(&mut reader)?,
                    ),
                )
            }
            SET_REWARD_EMISSIONS_IX_DISCM => {
                Ok(
                    Self::SetRewardEmissions(
                        SetRewardEmissionsIxArgs::deserialize(&mut reader)?,
                    ),
                )
            }
            OPEN_POSITION_IX_DISCM => {
                Ok(Self::OpenPosition(OpenPositionIxArgs::deserialize(&mut reader)?))
            }
            OPEN_POSITION_WITH_METADATA_IX_DISCM => {
                Ok(
                    Self::OpenPositionWithMetadata(
                        OpenPositionWithMetadataIxArgs::deserialize(&mut reader)?,
                    ),
                )
            }
            INCREASE_LIQUIDITY_IX_DISCM => {
                Ok(
                    Self::IncreaseLiquidity(
                        IncreaseLiquidityIxArgs::deserialize(&mut reader)?,
                    ),
                )
            }
            DECREASE_LIQUIDITY_IX_DISCM => {
                Ok(
                    Self::DecreaseLiquidity(
                        DecreaseLiquidityIxArgs::deserialize(&mut reader)?,
                    ),
                )
            }
            UPDATE_FEES_AND_REWARDS_IX_DISCM => Ok(Self::UpdateFeesAndRewards),
            COLLECT_FEES_IX_DISCM => Ok(Self::CollectFees),
            COLLECT_REWARD_IX_DISCM => {
                Ok(Self::CollectReward(CollectRewardIxArgs::deserialize(&mut reader)?))
            }
            COLLECT_PROTOCOL_FEES_IX_DISCM => Ok(Self::CollectProtocolFees),
            SWAP_IX_DISCM => Ok(Self::Swap(SwapIxArgs::deserialize(&mut reader)?)),
            CLOSE_POSITION_IX_DISCM => Ok(Self::ClosePosition),
            SET_DEFAULT_FEE_RATE_IX_DISCM => {
                Ok(
                    Self::SetDefaultFeeRate(
                        SetDefaultFeeRateIxArgs::deserialize(&mut reader)?,
                    ),
                )
            }
            SET_DEFAULT_PROTOCOL_FEE_RATE_IX_DISCM => {
                Ok(
                    Self::SetDefaultProtocolFeeRate(
                        SetDefaultProtocolFeeRateIxArgs::deserialize(&mut reader)?,
                    ),
                )
            }
            SET_FEE_RATE_IX_DISCM => {
                Ok(Self::SetFeeRate(SetFeeRateIxArgs::deserialize(&mut reader)?))
            }
            SET_PROTOCOL_FEE_RATE_IX_DISCM => {
                Ok(
                    Self::SetProtocolFeeRate(
                        SetProtocolFeeRateIxArgs::deserialize(&mut reader)?,
                    ),
                )
            }
            SET_FEE_AUTHORITY_IX_DISCM => Ok(Self::SetFeeAuthority),
            SET_COLLECT_PROTOCOL_FEES_AUTHORITY_IX_DISCM => {
                Ok(Self::SetCollectProtocolFeesAuthority)
            }
            SET_REWARD_AUTHORITY_IX_DISCM => {
                Ok(
                    Self::SetRewardAuthority(
                        SetRewardAuthorityIxArgs::deserialize(&mut reader)?,
                    ),
                )
            }
            SET_REWARD_AUTHORITY_BY_SUPER_AUTHORITY_IX_DISCM => {
                Ok(
                    Self::SetRewardAuthorityBySuperAuthority(
                        SetRewardAuthorityBySuperAuthorityIxArgs::deserialize(
                            &mut reader,
                        )?,
                    ),
                )
            }
            SET_REWARD_EMISSIONS_SUPER_AUTHORITY_IX_DISCM => {
                Ok(Self::SetRewardEmissionsSuperAuthority)
            }
            TWO_HOP_SWAP_IX_DISCM => {
                Ok(Self::TwoHopSwap(TwoHopSwapIxArgs::deserialize(&mut reader)?))
            }
            INITIALIZE_POSITION_BUNDLE_IX_DISCM => Ok(Self::InitializePositionBundle),
            INITIALIZE_POSITION_BUNDLE_WITH_METADATA_IX_DISCM => {
                Ok(Self::InitializePositionBundleWithMetadata)
            }
            DELETE_POSITION_BUNDLE_IX_DISCM => Ok(Self::DeletePositionBundle),
            OPEN_BUNDLED_POSITION_IX_DISCM => {
                Ok(
                    Self::OpenBundledPosition(
                        OpenBundledPositionIxArgs::deserialize(&mut reader)?,
                    ),
                )
            }
            CLOSE_BUNDLED_POSITION_IX_DISCM => {
                Ok(
                    Self::CloseBundledPosition(
                        CloseBundledPositionIxArgs::deserialize(&mut reader)?,
                    ),
                )
            }
            _ => {
                Err(
                    std::io::Error::new(
                        std::io::ErrorKind::Other,
                        format!("discm {:?} not found", maybe_discm),
                    ),
                )
            }
        }
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        match self {
            Self::InitializeConfig(args) => {
                writer.write_all(&INITIALIZE_CONFIG_IX_DISCM)?;
                args.serialize(&mut writer)
            }
            Self::InitializePool(args) => {
                writer.write_all(&INITIALIZE_POOL_IX_DISCM)?;
                args.serialize(&mut writer)
            }
            Self::InitializeTickArray(args) => {
                writer.write_all(&INITIALIZE_TICK_ARRAY_IX_DISCM)?;
                args.serialize(&mut writer)
            }
            Self::InitializeFeeTier(args) => {
                writer.write_all(&INITIALIZE_FEE_TIER_IX_DISCM)?;
                args.serialize(&mut writer)
            }
            Self::InitializeReward(args) => {
                writer.write_all(&INITIALIZE_REWARD_IX_DISCM)?;
                args.serialize(&mut writer)
            }
            Self::SetRewardEmissions(args) => {
                writer.write_all(&SET_REWARD_EMISSIONS_IX_DISCM)?;
                args.serialize(&mut writer)
            }
            Self::OpenPosition(args) => {
                writer.write_all(&OPEN_POSITION_IX_DISCM)?;
                args.serialize(&mut writer)
            }
            Self::OpenPositionWithMetadata(args) => {
                writer.write_all(&OPEN_POSITION_WITH_METADATA_IX_DISCM)?;
                args.serialize(&mut writer)
            }
            Self::IncreaseLiquidity(args) => {
                writer.write_all(&INCREASE_LIQUIDITY_IX_DISCM)?;
                args.serialize(&mut writer)
            }
            Self::DecreaseLiquidity(args) => {
                writer.write_all(&DECREASE_LIQUIDITY_IX_DISCM)?;
                args.serialize(&mut writer)
            }
            Self::UpdateFeesAndRewards => {
                writer.write_all(&UPDATE_FEES_AND_REWARDS_IX_DISCM)
            }
            Self::CollectFees => writer.write_all(&COLLECT_FEES_IX_DISCM),
            Self::CollectReward(args) => {
                writer.write_all(&COLLECT_REWARD_IX_DISCM)?;
                args.serialize(&mut writer)
            }
            Self::CollectProtocolFees => {
                writer.write_all(&COLLECT_PROTOCOL_FEES_IX_DISCM)
            }
            Self::Swap(args) => {
                writer.write_all(&SWAP_IX_DISCM)?;
                args.serialize(&mut writer)
            }
            Self::ClosePosition => writer.write_all(&CLOSE_POSITION_IX_DISCM),
            Self::SetDefaultFeeRate(args) => {
                writer.write_all(&SET_DEFAULT_FEE_RATE_IX_DISCM)?;
                args.serialize(&mut writer)
            }
            Self::SetDefaultProtocolFeeRate(args) => {
                writer.write_all(&SET_DEFAULT_PROTOCOL_FEE_RATE_IX_DISCM)?;
                args.serialize(&mut writer)
            }
            Self::SetFeeRate(args) => {
                writer.write_all(&SET_FEE_RATE_IX_DISCM)?;
                args.serialize(&mut writer)
            }
            Self::SetProtocolFeeRate(args) => {
                writer.write_all(&SET_PROTOCOL_FEE_RATE_IX_DISCM)?;
                args.serialize(&mut writer)
            }
            Self::SetFeeAuthority => writer.write_all(&SET_FEE_AUTHORITY_IX_DISCM),
            Self::SetCollectProtocolFeesAuthority => {
                writer.write_all(&SET_COLLECT_PROTOCOL_FEES_AUTHORITY_IX_DISCM)
            }
            Self::SetRewardAuthority(args) => {
                writer.write_all(&SET_REWARD_AUTHORITY_IX_DISCM)?;
                args.serialize(&mut writer)
            }
            Self::SetRewardAuthorityBySuperAuthority(args) => {
                writer.write_all(&SET_REWARD_AUTHORITY_BY_SUPER_AUTHORITY_IX_DISCM)?;
                args.serialize(&mut writer)
            }
            Self::SetRewardEmissionsSuperAuthority => {
                writer.write_all(&SET_REWARD_EMISSIONS_SUPER_AUTHORITY_IX_DISCM)
            }
            Self::TwoHopSwap(args) => {
                writer.write_all(&TWO_HOP_SWAP_IX_DISCM)?;
                args.serialize(&mut writer)
            }
            Self::InitializePositionBundle => {
                writer.write_all(&INITIALIZE_POSITION_BUNDLE_IX_DISCM)
            }
            Self::InitializePositionBundleWithMetadata => {
                writer.write_all(&INITIALIZE_POSITION_BUNDLE_WITH_METADATA_IX_DISCM)
            }
            Self::DeletePositionBundle => {
                writer.write_all(&DELETE_POSITION_BUNDLE_IX_DISCM)
            }
            Self::OpenBundledPosition(args) => {
                writer.write_all(&OPEN_BUNDLED_POSITION_IX_DISCM)?;
                args.serialize(&mut writer)
            }
            Self::CloseBundledPosition(args) => {
                writer.write_all(&CLOSE_BUNDLED_POSITION_IX_DISCM)?;
                args.serialize(&mut writer)
            }
        }
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
fn invoke_instruction<'info, A: Into<[AccountInfo<'info>; N]>, const N: usize>(
    ix: &Instruction,
    accounts: A,
) -> ProgramResult {
    let account_info: [AccountInfo<'info>; N] = accounts.into();
    invoke(ix, &account_info)
}
fn invoke_instruction_signed<'info, A: Into<[AccountInfo<'info>; N]>, const N: usize>(
    ix: &Instruction,
    accounts: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let account_info: [AccountInfo<'info>; N] = accounts.into();
    invoke_signed(ix, &account_info, seeds)
}
pub const INITIALIZE_CONFIG_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct InitializeConfigAccounts<'me, 'info> {
    pub config: &'me AccountInfo<'info>,
    pub funder: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InitializeConfigKeys {
    pub config: Pubkey,
    pub funder: Pubkey,
    pub system_program: Pubkey,
}
impl From<InitializeConfigAccounts<'_, '_>> for InitializeConfigKeys {
    fn from(accounts: InitializeConfigAccounts) -> Self {
        Self {
            config: *accounts.config.key,
            funder: *accounts.funder.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<InitializeConfigKeys> for [AccountMeta; INITIALIZE_CONFIG_IX_ACCOUNTS_LEN] {
    fn from(keys: InitializeConfigKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.config,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.funder,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.system_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; INITIALIZE_CONFIG_IX_ACCOUNTS_LEN]> for InitializeConfigKeys {
    fn from(pubkeys: [Pubkey; INITIALIZE_CONFIG_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            config: pubkeys[0],
            funder: pubkeys[1],
            system_program: pubkeys[2],
        }
    }
}
impl<'info> From<InitializeConfigAccounts<'_, 'info>>
for [AccountInfo<'info>; INITIALIZE_CONFIG_IX_ACCOUNTS_LEN] {
    fn from(accounts: InitializeConfigAccounts<'_, 'info>) -> Self {
        [
            accounts.config.clone(),
            accounts.funder.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; INITIALIZE_CONFIG_IX_ACCOUNTS_LEN]>
for InitializeConfigAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; INITIALIZE_CONFIG_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            config: &arr[0],
            funder: &arr[1],
            system_program: &arr[2],
        }
    }
}
pub const INITIALIZE_CONFIG_IX_DISCM: [u8; 8] = [208, 127, 21, 1, 194, 190, 196, 70];
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InitializeConfigIxArgs {
    pub fee_authority: Pubkey,
    pub collect_protocol_fees_authority: Pubkey,
    pub reward_emissions_super_authority: Pubkey,
    pub default_protocol_fee_rate: u16,
}
#[derive(Clone, Debug, PartialEq)]
pub struct InitializeConfigIxData(pub InitializeConfigIxArgs);
impl From<InitializeConfigIxArgs> for InitializeConfigIxData {
    fn from(args: InitializeConfigIxArgs) -> Self {
        Self(args)
    }
}
impl InitializeConfigIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INITIALIZE_CONFIG_IX_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        INITIALIZE_CONFIG_IX_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(InitializeConfigIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INITIALIZE_CONFIG_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn initialize_config_ix_with_program_id(
    program_id: Pubkey,
    keys: InitializeConfigKeys,
    args: InitializeConfigIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INITIALIZE_CONFIG_IX_ACCOUNTS_LEN] = keys.into();
    let data: InitializeConfigIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn initialize_config_ix(
    keys: InitializeConfigKeys,
    args: InitializeConfigIxArgs,
) -> std::io::Result<Instruction> {
    initialize_config_ix_with_program_id(crate::ID, keys, args)
}
pub fn initialize_config_invoke_with_program_id(
    program_id: Pubkey,
    accounts: InitializeConfigAccounts<'_, '_>,
    args: InitializeConfigIxArgs,
) -> ProgramResult {
    let keys: InitializeConfigKeys = accounts.into();
    let ix = initialize_config_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn initialize_config_invoke(
    accounts: InitializeConfigAccounts<'_, '_>,
    args: InitializeConfigIxArgs,
) -> ProgramResult {
    initialize_config_invoke_with_program_id(crate::ID, accounts, args)
}
pub fn initialize_config_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: InitializeConfigAccounts<'_, '_>,
    args: InitializeConfigIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: InitializeConfigKeys = accounts.into();
    let ix = initialize_config_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn initialize_config_invoke_signed(
    accounts: InitializeConfigAccounts<'_, '_>,
    args: InitializeConfigIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    initialize_config_invoke_signed_with_program_id(crate::ID, accounts, args, seeds)
}
pub fn initialize_config_verify_account_keys(
    accounts: InitializeConfigAccounts<'_, '_>,
    keys: InitializeConfigKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.config.key, keys.config),
        (*accounts.funder.key, keys.funder),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn initialize_config_verify_writable_privileges<'me, 'info>(
    accounts: InitializeConfigAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.config, accounts.funder] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn initialize_config_verify_signer_privileges<'me, 'info>(
    accounts: InitializeConfigAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.config, accounts.funder] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn initialize_config_verify_account_privileges<'me, 'info>(
    accounts: InitializeConfigAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    initialize_config_verify_writable_privileges(accounts)?;
    initialize_config_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const INITIALIZE_POOL_IX_ACCOUNTS_LEN: usize = 11;
#[derive(Copy, Clone, Debug)]
pub struct InitializePoolAccounts<'me, 'info> {
    pub whirlpools_config: &'me AccountInfo<'info>,
    pub token_mint_a: &'me AccountInfo<'info>,
    pub token_mint_b: &'me AccountInfo<'info>,
    pub funder: &'me AccountInfo<'info>,
    pub whirlpool: &'me AccountInfo<'info>,
    pub token_vault_a: &'me AccountInfo<'info>,
    pub token_vault_b: &'me AccountInfo<'info>,
    pub fee_tier: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub rent: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InitializePoolKeys {
    pub whirlpools_config: Pubkey,
    pub token_mint_a: Pubkey,
    pub token_mint_b: Pubkey,
    pub funder: Pubkey,
    pub whirlpool: Pubkey,
    pub token_vault_a: Pubkey,
    pub token_vault_b: Pubkey,
    pub fee_tier: Pubkey,
    pub token_program: Pubkey,
    pub system_program: Pubkey,
    pub rent: Pubkey,
}
impl From<InitializePoolAccounts<'_, '_>> for InitializePoolKeys {
    fn from(accounts: InitializePoolAccounts) -> Self {
        Self {
            whirlpools_config: *accounts.whirlpools_config.key,
            token_mint_a: *accounts.token_mint_a.key,
            token_mint_b: *accounts.token_mint_b.key,
            funder: *accounts.funder.key,
            whirlpool: *accounts.whirlpool.key,
            token_vault_a: *accounts.token_vault_a.key,
            token_vault_b: *accounts.token_vault_b.key,
            fee_tier: *accounts.fee_tier.key,
            token_program: *accounts.token_program.key,
            system_program: *accounts.system_program.key,
            rent: *accounts.rent.key,
        }
    }
}
impl From<InitializePoolKeys> for [AccountMeta; INITIALIZE_POOL_IX_ACCOUNTS_LEN] {
    fn from(keys: InitializePoolKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.whirlpools_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_mint_a,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_mint_b,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.funder,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.whirlpool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_vault_a,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_vault_b,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.fee_tier,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.system_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.rent,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; INITIALIZE_POOL_IX_ACCOUNTS_LEN]> for InitializePoolKeys {
    fn from(pubkeys: [Pubkey; INITIALIZE_POOL_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            whirlpools_config: pubkeys[0],
            token_mint_a: pubkeys[1],
            token_mint_b: pubkeys[2],
            funder: pubkeys[3],
            whirlpool: pubkeys[4],
            token_vault_a: pubkeys[5],
            token_vault_b: pubkeys[6],
            fee_tier: pubkeys[7],
            token_program: pubkeys[8],
            system_program: pubkeys[9],
            rent: pubkeys[10],
        }
    }
}
impl<'info> From<InitializePoolAccounts<'_, 'info>>
for [AccountInfo<'info>; INITIALIZE_POOL_IX_ACCOUNTS_LEN] {
    fn from(accounts: InitializePoolAccounts<'_, 'info>) -> Self {
        [
            accounts.whirlpools_config.clone(),
            accounts.token_mint_a.clone(),
            accounts.token_mint_b.clone(),
            accounts.funder.clone(),
            accounts.whirlpool.clone(),
            accounts.token_vault_a.clone(),
            accounts.token_vault_b.clone(),
            accounts.fee_tier.clone(),
            accounts.token_program.clone(),
            accounts.system_program.clone(),
            accounts.rent.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; INITIALIZE_POOL_IX_ACCOUNTS_LEN]>
for InitializePoolAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; INITIALIZE_POOL_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            whirlpools_config: &arr[0],
            token_mint_a: &arr[1],
            token_mint_b: &arr[2],
            funder: &arr[3],
            whirlpool: &arr[4],
            token_vault_a: &arr[5],
            token_vault_b: &arr[6],
            fee_tier: &arr[7],
            token_program: &arr[8],
            system_program: &arr[9],
            rent: &arr[10],
        }
    }
}
pub const INITIALIZE_POOL_IX_DISCM: [u8; 8] = [95, 180, 10, 172, 84, 174, 232, 40];
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InitializePoolIxArgs {
    pub bumps: WhirlpoolBumps,
    pub tick_spacing: u16,
    pub initial_sqrt_price: u128,
}
#[derive(Clone, Debug, PartialEq)]
pub struct InitializePoolIxData(pub InitializePoolIxArgs);
impl From<InitializePoolIxArgs> for InitializePoolIxData {
    fn from(args: InitializePoolIxArgs) -> Self {
        Self(args)
    }
}
impl InitializePoolIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INITIALIZE_POOL_IX_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        INITIALIZE_POOL_IX_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(InitializePoolIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INITIALIZE_POOL_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn initialize_pool_ix_with_program_id(
    program_id: Pubkey,
    keys: InitializePoolKeys,
    args: InitializePoolIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INITIALIZE_POOL_IX_ACCOUNTS_LEN] = keys.into();
    let data: InitializePoolIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn initialize_pool_ix(
    keys: InitializePoolKeys,
    args: InitializePoolIxArgs,
) -> std::io::Result<Instruction> {
    initialize_pool_ix_with_program_id(crate::ID, keys, args)
}
pub fn initialize_pool_invoke_with_program_id(
    program_id: Pubkey,
    accounts: InitializePoolAccounts<'_, '_>,
    args: InitializePoolIxArgs,
) -> ProgramResult {
    let keys: InitializePoolKeys = accounts.into();
    let ix = initialize_pool_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn initialize_pool_invoke(
    accounts: InitializePoolAccounts<'_, '_>,
    args: InitializePoolIxArgs,
) -> ProgramResult {
    initialize_pool_invoke_with_program_id(crate::ID, accounts, args)
}
pub fn initialize_pool_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: InitializePoolAccounts<'_, '_>,
    args: InitializePoolIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: InitializePoolKeys = accounts.into();
    let ix = initialize_pool_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn initialize_pool_invoke_signed(
    accounts: InitializePoolAccounts<'_, '_>,
    args: InitializePoolIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    initialize_pool_invoke_signed_with_program_id(crate::ID, accounts, args, seeds)
}
pub fn initialize_pool_verify_account_keys(
    accounts: InitializePoolAccounts<'_, '_>,
    keys: InitializePoolKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.whirlpools_config.key, keys.whirlpools_config),
        (*accounts.token_mint_a.key, keys.token_mint_a),
        (*accounts.token_mint_b.key, keys.token_mint_b),
        (*accounts.funder.key, keys.funder),
        (*accounts.whirlpool.key, keys.whirlpool),
        (*accounts.token_vault_a.key, keys.token_vault_a),
        (*accounts.token_vault_b.key, keys.token_vault_b),
        (*accounts.fee_tier.key, keys.fee_tier),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.rent.key, keys.rent),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn initialize_pool_verify_writable_privileges<'me, 'info>(
    accounts: InitializePoolAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.funder,
        accounts.whirlpool,
        accounts.token_vault_a,
        accounts.token_vault_b,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn initialize_pool_verify_signer_privileges<'me, 'info>(
    accounts: InitializePoolAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [
        accounts.funder,
        accounts.token_vault_a,
        accounts.token_vault_b,
    ] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn initialize_pool_verify_account_privileges<'me, 'info>(
    accounts: InitializePoolAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    initialize_pool_verify_writable_privileges(accounts)?;
    initialize_pool_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const INITIALIZE_TICK_ARRAY_IX_ACCOUNTS_LEN: usize = 4;
#[derive(Copy, Clone, Debug)]
pub struct InitializeTickArrayAccounts<'me, 'info> {
    pub whirlpool: &'me AccountInfo<'info>,
    pub funder: &'me AccountInfo<'info>,
    pub tick_array: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InitializeTickArrayKeys {
    pub whirlpool: Pubkey,
    pub funder: Pubkey,
    pub tick_array: Pubkey,
    pub system_program: Pubkey,
}
impl From<InitializeTickArrayAccounts<'_, '_>> for InitializeTickArrayKeys {
    fn from(accounts: InitializeTickArrayAccounts) -> Self {
        Self {
            whirlpool: *accounts.whirlpool.key,
            funder: *accounts.funder.key,
            tick_array: *accounts.tick_array.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<InitializeTickArrayKeys>
for [AccountMeta; INITIALIZE_TICK_ARRAY_IX_ACCOUNTS_LEN] {
    fn from(keys: InitializeTickArrayKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.whirlpool,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.funder,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.tick_array,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.system_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; INITIALIZE_TICK_ARRAY_IX_ACCOUNTS_LEN]> for InitializeTickArrayKeys {
    fn from(pubkeys: [Pubkey; INITIALIZE_TICK_ARRAY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            whirlpool: pubkeys[0],
            funder: pubkeys[1],
            tick_array: pubkeys[2],
            system_program: pubkeys[3],
        }
    }
}
impl<'info> From<InitializeTickArrayAccounts<'_, 'info>>
for [AccountInfo<'info>; INITIALIZE_TICK_ARRAY_IX_ACCOUNTS_LEN] {
    fn from(accounts: InitializeTickArrayAccounts<'_, 'info>) -> Self {
        [
            accounts.whirlpool.clone(),
            accounts.funder.clone(),
            accounts.tick_array.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; INITIALIZE_TICK_ARRAY_IX_ACCOUNTS_LEN]>
for InitializeTickArrayAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; INITIALIZE_TICK_ARRAY_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            whirlpool: &arr[0],
            funder: &arr[1],
            tick_array: &arr[2],
            system_program: &arr[3],
        }
    }
}
pub const INITIALIZE_TICK_ARRAY_IX_DISCM: [u8; 8] = [
    11,
    188,
    193,
    214,
    141,
    91,
    149,
    184,
];
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InitializeTickArrayIxArgs {
    pub start_tick_index: i32,
}
#[derive(Clone, Debug, PartialEq)]
pub struct InitializeTickArrayIxData(pub InitializeTickArrayIxArgs);
impl From<InitializeTickArrayIxArgs> for InitializeTickArrayIxData {
    fn from(args: InitializeTickArrayIxArgs) -> Self {
        Self(args)
    }
}
impl InitializeTickArrayIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INITIALIZE_TICK_ARRAY_IX_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        INITIALIZE_TICK_ARRAY_IX_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(InitializeTickArrayIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INITIALIZE_TICK_ARRAY_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn initialize_tick_array_ix_with_program_id(
    program_id: Pubkey,
    keys: InitializeTickArrayKeys,
    args: InitializeTickArrayIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INITIALIZE_TICK_ARRAY_IX_ACCOUNTS_LEN] = keys.into();
    let data: InitializeTickArrayIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn initialize_tick_array_ix(
    keys: InitializeTickArrayKeys,
    args: InitializeTickArrayIxArgs,
) -> std::io::Result<Instruction> {
    initialize_tick_array_ix_with_program_id(crate::ID, keys, args)
}
pub fn initialize_tick_array_invoke_with_program_id(
    program_id: Pubkey,
    accounts: InitializeTickArrayAccounts<'_, '_>,
    args: InitializeTickArrayIxArgs,
) -> ProgramResult {
    let keys: InitializeTickArrayKeys = accounts.into();
    let ix = initialize_tick_array_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn initialize_tick_array_invoke(
    accounts: InitializeTickArrayAccounts<'_, '_>,
    args: InitializeTickArrayIxArgs,
) -> ProgramResult {
    initialize_tick_array_invoke_with_program_id(crate::ID, accounts, args)
}
pub fn initialize_tick_array_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: InitializeTickArrayAccounts<'_, '_>,
    args: InitializeTickArrayIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: InitializeTickArrayKeys = accounts.into();
    let ix = initialize_tick_array_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn initialize_tick_array_invoke_signed(
    accounts: InitializeTickArrayAccounts<'_, '_>,
    args: InitializeTickArrayIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    initialize_tick_array_invoke_signed_with_program_id(crate::ID, accounts, args, seeds)
}
pub fn initialize_tick_array_verify_account_keys(
    accounts: InitializeTickArrayAccounts<'_, '_>,
    keys: InitializeTickArrayKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.whirlpool.key, keys.whirlpool),
        (*accounts.funder.key, keys.funder),
        (*accounts.tick_array.key, keys.tick_array),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn initialize_tick_array_verify_writable_privileges<'me, 'info>(
    accounts: InitializeTickArrayAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.funder, accounts.tick_array] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn initialize_tick_array_verify_signer_privileges<'me, 'info>(
    accounts: InitializeTickArrayAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.funder] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn initialize_tick_array_verify_account_privileges<'me, 'info>(
    accounts: InitializeTickArrayAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    initialize_tick_array_verify_writable_privileges(accounts)?;
    initialize_tick_array_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const INITIALIZE_FEE_TIER_IX_ACCOUNTS_LEN: usize = 5;
#[derive(Copy, Clone, Debug)]
pub struct InitializeFeeTierAccounts<'me, 'info> {
    pub config: &'me AccountInfo<'info>,
    pub fee_tier: &'me AccountInfo<'info>,
    pub funder: &'me AccountInfo<'info>,
    pub fee_authority: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InitializeFeeTierKeys {
    pub config: Pubkey,
    pub fee_tier: Pubkey,
    pub funder: Pubkey,
    pub fee_authority: Pubkey,
    pub system_program: Pubkey,
}
impl From<InitializeFeeTierAccounts<'_, '_>> for InitializeFeeTierKeys {
    fn from(accounts: InitializeFeeTierAccounts) -> Self {
        Self {
            config: *accounts.config.key,
            fee_tier: *accounts.fee_tier.key,
            funder: *accounts.funder.key,
            fee_authority: *accounts.fee_authority.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<InitializeFeeTierKeys> for [AccountMeta; INITIALIZE_FEE_TIER_IX_ACCOUNTS_LEN] {
    fn from(keys: InitializeFeeTierKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.fee_tier,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.funder,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.fee_authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.system_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; INITIALIZE_FEE_TIER_IX_ACCOUNTS_LEN]> for InitializeFeeTierKeys {
    fn from(pubkeys: [Pubkey; INITIALIZE_FEE_TIER_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            config: pubkeys[0],
            fee_tier: pubkeys[1],
            funder: pubkeys[2],
            fee_authority: pubkeys[3],
            system_program: pubkeys[4],
        }
    }
}
impl<'info> From<InitializeFeeTierAccounts<'_, 'info>>
for [AccountInfo<'info>; INITIALIZE_FEE_TIER_IX_ACCOUNTS_LEN] {
    fn from(accounts: InitializeFeeTierAccounts<'_, 'info>) -> Self {
        [
            accounts.config.clone(),
            accounts.fee_tier.clone(),
            accounts.funder.clone(),
            accounts.fee_authority.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; INITIALIZE_FEE_TIER_IX_ACCOUNTS_LEN]>
for InitializeFeeTierAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; INITIALIZE_FEE_TIER_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            config: &arr[0],
            fee_tier: &arr[1],
            funder: &arr[2],
            fee_authority: &arr[3],
            system_program: &arr[4],
        }
    }
}
pub const INITIALIZE_FEE_TIER_IX_DISCM: [u8; 8] = [183, 74, 156, 160, 112, 2, 42, 30];
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InitializeFeeTierIxArgs {
    pub tick_spacing: u16,
    pub default_fee_rate: u16,
}
#[derive(Clone, Debug, PartialEq)]
pub struct InitializeFeeTierIxData(pub InitializeFeeTierIxArgs);
impl From<InitializeFeeTierIxArgs> for InitializeFeeTierIxData {
    fn from(args: InitializeFeeTierIxArgs) -> Self {
        Self(args)
    }
}
impl InitializeFeeTierIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INITIALIZE_FEE_TIER_IX_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        INITIALIZE_FEE_TIER_IX_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(InitializeFeeTierIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INITIALIZE_FEE_TIER_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn initialize_fee_tier_ix_with_program_id(
    program_id: Pubkey,
    keys: InitializeFeeTierKeys,
    args: InitializeFeeTierIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INITIALIZE_FEE_TIER_IX_ACCOUNTS_LEN] = keys.into();
    let data: InitializeFeeTierIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn initialize_fee_tier_ix(
    keys: InitializeFeeTierKeys,
    args: InitializeFeeTierIxArgs,
) -> std::io::Result<Instruction> {
    initialize_fee_tier_ix_with_program_id(crate::ID, keys, args)
}
pub fn initialize_fee_tier_invoke_with_program_id(
    program_id: Pubkey,
    accounts: InitializeFeeTierAccounts<'_, '_>,
    args: InitializeFeeTierIxArgs,
) -> ProgramResult {
    let keys: InitializeFeeTierKeys = accounts.into();
    let ix = initialize_fee_tier_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn initialize_fee_tier_invoke(
    accounts: InitializeFeeTierAccounts<'_, '_>,
    args: InitializeFeeTierIxArgs,
) -> ProgramResult {
    initialize_fee_tier_invoke_with_program_id(crate::ID, accounts, args)
}
pub fn initialize_fee_tier_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: InitializeFeeTierAccounts<'_, '_>,
    args: InitializeFeeTierIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: InitializeFeeTierKeys = accounts.into();
    let ix = initialize_fee_tier_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn initialize_fee_tier_invoke_signed(
    accounts: InitializeFeeTierAccounts<'_, '_>,
    args: InitializeFeeTierIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    initialize_fee_tier_invoke_signed_with_program_id(crate::ID, accounts, args, seeds)
}
pub fn initialize_fee_tier_verify_account_keys(
    accounts: InitializeFeeTierAccounts<'_, '_>,
    keys: InitializeFeeTierKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.config.key, keys.config),
        (*accounts.fee_tier.key, keys.fee_tier),
        (*accounts.funder.key, keys.funder),
        (*accounts.fee_authority.key, keys.fee_authority),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn initialize_fee_tier_verify_writable_privileges<'me, 'info>(
    accounts: InitializeFeeTierAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.fee_tier, accounts.funder] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn initialize_fee_tier_verify_signer_privileges<'me, 'info>(
    accounts: InitializeFeeTierAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.funder, accounts.fee_authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn initialize_fee_tier_verify_account_privileges<'me, 'info>(
    accounts: InitializeFeeTierAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    initialize_fee_tier_verify_writable_privileges(accounts)?;
    initialize_fee_tier_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const INITIALIZE_REWARD_IX_ACCOUNTS_LEN: usize = 8;
#[derive(Copy, Clone, Debug)]
pub struct InitializeRewardAccounts<'me, 'info> {
    pub reward_authority: &'me AccountInfo<'info>,
    pub funder: &'me AccountInfo<'info>,
    pub whirlpool: &'me AccountInfo<'info>,
    pub reward_mint: &'me AccountInfo<'info>,
    pub reward_vault: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub rent: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InitializeRewardKeys {
    pub reward_authority: Pubkey,
    pub funder: Pubkey,
    pub whirlpool: Pubkey,
    pub reward_mint: Pubkey,
    pub reward_vault: Pubkey,
    pub token_program: Pubkey,
    pub system_program: Pubkey,
    pub rent: Pubkey,
}
impl From<InitializeRewardAccounts<'_, '_>> for InitializeRewardKeys {
    fn from(accounts: InitializeRewardAccounts) -> Self {
        Self {
            reward_authority: *accounts.reward_authority.key,
            funder: *accounts.funder.key,
            whirlpool: *accounts.whirlpool.key,
            reward_mint: *accounts.reward_mint.key,
            reward_vault: *accounts.reward_vault.key,
            token_program: *accounts.token_program.key,
            system_program: *accounts.system_program.key,
            rent: *accounts.rent.key,
        }
    }
}
impl From<InitializeRewardKeys> for [AccountMeta; INITIALIZE_REWARD_IX_ACCOUNTS_LEN] {
    fn from(keys: InitializeRewardKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.reward_authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.funder,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.whirlpool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.reward_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.reward_vault,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.system_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.rent,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; INITIALIZE_REWARD_IX_ACCOUNTS_LEN]> for InitializeRewardKeys {
    fn from(pubkeys: [Pubkey; INITIALIZE_REWARD_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            reward_authority: pubkeys[0],
            funder: pubkeys[1],
            whirlpool: pubkeys[2],
            reward_mint: pubkeys[3],
            reward_vault: pubkeys[4],
            token_program: pubkeys[5],
            system_program: pubkeys[6],
            rent: pubkeys[7],
        }
    }
}
impl<'info> From<InitializeRewardAccounts<'_, 'info>>
for [AccountInfo<'info>; INITIALIZE_REWARD_IX_ACCOUNTS_LEN] {
    fn from(accounts: InitializeRewardAccounts<'_, 'info>) -> Self {
        [
            accounts.reward_authority.clone(),
            accounts.funder.clone(),
            accounts.whirlpool.clone(),
            accounts.reward_mint.clone(),
            accounts.reward_vault.clone(),
            accounts.token_program.clone(),
            accounts.system_program.clone(),
            accounts.rent.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; INITIALIZE_REWARD_IX_ACCOUNTS_LEN]>
for InitializeRewardAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; INITIALIZE_REWARD_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            reward_authority: &arr[0],
            funder: &arr[1],
            whirlpool: &arr[2],
            reward_mint: &arr[3],
            reward_vault: &arr[4],
            token_program: &arr[5],
            system_program: &arr[6],
            rent: &arr[7],
        }
    }
}
pub const INITIALIZE_REWARD_IX_DISCM: [u8; 8] = [95, 135, 192, 196, 242, 129, 230, 68];
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InitializeRewardIxArgs {
    pub reward_index: u8,
}
#[derive(Clone, Debug, PartialEq)]
pub struct InitializeRewardIxData(pub InitializeRewardIxArgs);
impl From<InitializeRewardIxArgs> for InitializeRewardIxData {
    fn from(args: InitializeRewardIxArgs) -> Self {
        Self(args)
    }
}
impl InitializeRewardIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INITIALIZE_REWARD_IX_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        INITIALIZE_REWARD_IX_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(InitializeRewardIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INITIALIZE_REWARD_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn initialize_reward_ix_with_program_id(
    program_id: Pubkey,
    keys: InitializeRewardKeys,
    args: InitializeRewardIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INITIALIZE_REWARD_IX_ACCOUNTS_LEN] = keys.into();
    let data: InitializeRewardIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn initialize_reward_ix(
    keys: InitializeRewardKeys,
    args: InitializeRewardIxArgs,
) -> std::io::Result<Instruction> {
    initialize_reward_ix_with_program_id(crate::ID, keys, args)
}
pub fn initialize_reward_invoke_with_program_id(
    program_id: Pubkey,
    accounts: InitializeRewardAccounts<'_, '_>,
    args: InitializeRewardIxArgs,
) -> ProgramResult {
    let keys: InitializeRewardKeys = accounts.into();
    let ix = initialize_reward_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn initialize_reward_invoke(
    accounts: InitializeRewardAccounts<'_, '_>,
    args: InitializeRewardIxArgs,
) -> ProgramResult {
    initialize_reward_invoke_with_program_id(crate::ID, accounts, args)
}
pub fn initialize_reward_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: InitializeRewardAccounts<'_, '_>,
    args: InitializeRewardIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: InitializeRewardKeys = accounts.into();
    let ix = initialize_reward_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn initialize_reward_invoke_signed(
    accounts: InitializeRewardAccounts<'_, '_>,
    args: InitializeRewardIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    initialize_reward_invoke_signed_with_program_id(crate::ID, accounts, args, seeds)
}
pub fn initialize_reward_verify_account_keys(
    accounts: InitializeRewardAccounts<'_, '_>,
    keys: InitializeRewardKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.reward_authority.key, keys.reward_authority),
        (*accounts.funder.key, keys.funder),
        (*accounts.whirlpool.key, keys.whirlpool),
        (*accounts.reward_mint.key, keys.reward_mint),
        (*accounts.reward_vault.key, keys.reward_vault),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.rent.key, keys.rent),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn initialize_reward_verify_writable_privileges<'me, 'info>(
    accounts: InitializeRewardAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.funder,
        accounts.whirlpool,
        accounts.reward_vault,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn initialize_reward_verify_signer_privileges<'me, 'info>(
    accounts: InitializeRewardAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [
        accounts.reward_authority,
        accounts.funder,
        accounts.reward_vault,
    ] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn initialize_reward_verify_account_privileges<'me, 'info>(
    accounts: InitializeRewardAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    initialize_reward_verify_writable_privileges(accounts)?;
    initialize_reward_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SET_REWARD_EMISSIONS_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct SetRewardEmissionsAccounts<'me, 'info> {
    pub whirlpool: &'me AccountInfo<'info>,
    pub reward_authority: &'me AccountInfo<'info>,
    pub reward_vault: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SetRewardEmissionsKeys {
    pub whirlpool: Pubkey,
    pub reward_authority: Pubkey,
    pub reward_vault: Pubkey,
}
impl From<SetRewardEmissionsAccounts<'_, '_>> for SetRewardEmissionsKeys {
    fn from(accounts: SetRewardEmissionsAccounts) -> Self {
        Self {
            whirlpool: *accounts.whirlpool.key,
            reward_authority: *accounts.reward_authority.key,
            reward_vault: *accounts.reward_vault.key,
        }
    }
}
impl From<SetRewardEmissionsKeys>
for [AccountMeta; SET_REWARD_EMISSIONS_IX_ACCOUNTS_LEN] {
    fn from(keys: SetRewardEmissionsKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.whirlpool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.reward_authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.reward_vault,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; SET_REWARD_EMISSIONS_IX_ACCOUNTS_LEN]> for SetRewardEmissionsKeys {
    fn from(pubkeys: [Pubkey; SET_REWARD_EMISSIONS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            whirlpool: pubkeys[0],
            reward_authority: pubkeys[1],
            reward_vault: pubkeys[2],
        }
    }
}
impl<'info> From<SetRewardEmissionsAccounts<'_, 'info>>
for [AccountInfo<'info>; SET_REWARD_EMISSIONS_IX_ACCOUNTS_LEN] {
    fn from(accounts: SetRewardEmissionsAccounts<'_, 'info>) -> Self {
        [
            accounts.whirlpool.clone(),
            accounts.reward_authority.clone(),
            accounts.reward_vault.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SET_REWARD_EMISSIONS_IX_ACCOUNTS_LEN]>
for SetRewardEmissionsAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; SET_REWARD_EMISSIONS_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            whirlpool: &arr[0],
            reward_authority: &arr[1],
            reward_vault: &arr[2],
        }
    }
}
pub const SET_REWARD_EMISSIONS_IX_DISCM: [u8; 8] = [13, 197, 86, 168, 109, 176, 27, 244];
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SetRewardEmissionsIxArgs {
    pub reward_index: u8,
    pub emissions_per_second_x64: u128,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SetRewardEmissionsIxData(pub SetRewardEmissionsIxArgs);
impl From<SetRewardEmissionsIxArgs> for SetRewardEmissionsIxData {
    fn from(args: SetRewardEmissionsIxArgs) -> Self {
        Self(args)
    }
}
impl SetRewardEmissionsIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SET_REWARD_EMISSIONS_IX_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        SET_REWARD_EMISSIONS_IX_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(SetRewardEmissionsIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SET_REWARD_EMISSIONS_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn set_reward_emissions_ix_with_program_id(
    program_id: Pubkey,
    keys: SetRewardEmissionsKeys,
    args: SetRewardEmissionsIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SET_REWARD_EMISSIONS_IX_ACCOUNTS_LEN] = keys.into();
    let data: SetRewardEmissionsIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn set_reward_emissions_ix(
    keys: SetRewardEmissionsKeys,
    args: SetRewardEmissionsIxArgs,
) -> std::io::Result<Instruction> {
    set_reward_emissions_ix_with_program_id(crate::ID, keys, args)
}
pub fn set_reward_emissions_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SetRewardEmissionsAccounts<'_, '_>,
    args: SetRewardEmissionsIxArgs,
) -> ProgramResult {
    let keys: SetRewardEmissionsKeys = accounts.into();
    let ix = set_reward_emissions_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn set_reward_emissions_invoke(
    accounts: SetRewardEmissionsAccounts<'_, '_>,
    args: SetRewardEmissionsIxArgs,
) -> ProgramResult {
    set_reward_emissions_invoke_with_program_id(crate::ID, accounts, args)
}
pub fn set_reward_emissions_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SetRewardEmissionsAccounts<'_, '_>,
    args: SetRewardEmissionsIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SetRewardEmissionsKeys = accounts.into();
    let ix = set_reward_emissions_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn set_reward_emissions_invoke_signed(
    accounts: SetRewardEmissionsAccounts<'_, '_>,
    args: SetRewardEmissionsIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    set_reward_emissions_invoke_signed_with_program_id(crate::ID, accounts, args, seeds)
}
pub fn set_reward_emissions_verify_account_keys(
    accounts: SetRewardEmissionsAccounts<'_, '_>,
    keys: SetRewardEmissionsKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.whirlpool.key, keys.whirlpool),
        (*accounts.reward_authority.key, keys.reward_authority),
        (*accounts.reward_vault.key, keys.reward_vault),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn set_reward_emissions_verify_writable_privileges<'me, 'info>(
    accounts: SetRewardEmissionsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.whirlpool] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn set_reward_emissions_verify_signer_privileges<'me, 'info>(
    accounts: SetRewardEmissionsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.reward_authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn set_reward_emissions_verify_account_privileges<'me, 'info>(
    accounts: SetRewardEmissionsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    set_reward_emissions_verify_writable_privileges(accounts)?;
    set_reward_emissions_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const OPEN_POSITION_IX_ACCOUNTS_LEN: usize = 10;
#[derive(Copy, Clone, Debug)]
pub struct OpenPositionAccounts<'me, 'info> {
    pub funder: &'me AccountInfo<'info>,
    pub owner: &'me AccountInfo<'info>,
    pub position: &'me AccountInfo<'info>,
    pub position_mint: &'me AccountInfo<'info>,
    pub position_token_account: &'me AccountInfo<'info>,
    pub whirlpool: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub rent: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct OpenPositionKeys {
    pub funder: Pubkey,
    pub owner: Pubkey,
    pub position: Pubkey,
    pub position_mint: Pubkey,
    pub position_token_account: Pubkey,
    pub whirlpool: Pubkey,
    pub token_program: Pubkey,
    pub system_program: Pubkey,
    pub rent: Pubkey,
    pub associated_token_program: Pubkey,
}
impl From<OpenPositionAccounts<'_, '_>> for OpenPositionKeys {
    fn from(accounts: OpenPositionAccounts) -> Self {
        Self {
            funder: *accounts.funder.key,
            owner: *accounts.owner.key,
            position: *accounts.position.key,
            position_mint: *accounts.position_mint.key,
            position_token_account: *accounts.position_token_account.key,
            whirlpool: *accounts.whirlpool.key,
            token_program: *accounts.token_program.key,
            system_program: *accounts.system_program.key,
            rent: *accounts.rent.key,
            associated_token_program: *accounts.associated_token_program.key,
        }
    }
}
impl From<OpenPositionKeys> for [AccountMeta; OPEN_POSITION_IX_ACCOUNTS_LEN] {
    fn from(keys: OpenPositionKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.funder,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.owner,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.position,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.position_mint,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.position_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.whirlpool,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.system_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.rent,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.associated_token_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; OPEN_POSITION_IX_ACCOUNTS_LEN]> for OpenPositionKeys {
    fn from(pubkeys: [Pubkey; OPEN_POSITION_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            funder: pubkeys[0],
            owner: pubkeys[1],
            position: pubkeys[2],
            position_mint: pubkeys[3],
            position_token_account: pubkeys[4],
            whirlpool: pubkeys[5],
            token_program: pubkeys[6],
            system_program: pubkeys[7],
            rent: pubkeys[8],
            associated_token_program: pubkeys[9],
        }
    }
}
impl<'info> From<OpenPositionAccounts<'_, 'info>>
for [AccountInfo<'info>; OPEN_POSITION_IX_ACCOUNTS_LEN] {
    fn from(accounts: OpenPositionAccounts<'_, 'info>) -> Self {
        [
            accounts.funder.clone(),
            accounts.owner.clone(),
            accounts.position.clone(),
            accounts.position_mint.clone(),
            accounts.position_token_account.clone(),
            accounts.whirlpool.clone(),
            accounts.token_program.clone(),
            accounts.system_program.clone(),
            accounts.rent.clone(),
            accounts.associated_token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; OPEN_POSITION_IX_ACCOUNTS_LEN]>
for OpenPositionAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; OPEN_POSITION_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            funder: &arr[0],
            owner: &arr[1],
            position: &arr[2],
            position_mint: &arr[3],
            position_token_account: &arr[4],
            whirlpool: &arr[5],
            token_program: &arr[6],
            system_program: &arr[7],
            rent: &arr[8],
            associated_token_program: &arr[9],
        }
    }
}
pub const OPEN_POSITION_IX_DISCM: [u8; 8] = [135, 128, 47, 77, 15, 152, 240, 49];
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct OpenPositionIxArgs {
    pub bumps: OpenPositionBumps,
    pub tick_lower_index: i32,
    pub tick_upper_index: i32,
}
#[derive(Clone, Debug, PartialEq)]
pub struct OpenPositionIxData(pub OpenPositionIxArgs);
impl From<OpenPositionIxArgs> for OpenPositionIxData {
    fn from(args: OpenPositionIxArgs) -> Self {
        Self(args)
    }
}
impl OpenPositionIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != OPEN_POSITION_IX_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        OPEN_POSITION_IX_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(OpenPositionIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&OPEN_POSITION_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn open_position_ix_with_program_id(
    program_id: Pubkey,
    keys: OpenPositionKeys,
    args: OpenPositionIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; OPEN_POSITION_IX_ACCOUNTS_LEN] = keys.into();
    let data: OpenPositionIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn open_position_ix(
    keys: OpenPositionKeys,
    args: OpenPositionIxArgs,
) -> std::io::Result<Instruction> {
    open_position_ix_with_program_id(crate::ID, keys, args)
}
pub fn open_position_invoke_with_program_id(
    program_id: Pubkey,
    accounts: OpenPositionAccounts<'_, '_>,
    args: OpenPositionIxArgs,
) -> ProgramResult {
    let keys: OpenPositionKeys = accounts.into();
    let ix = open_position_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn open_position_invoke(
    accounts: OpenPositionAccounts<'_, '_>,
    args: OpenPositionIxArgs,
) -> ProgramResult {
    open_position_invoke_with_program_id(crate::ID, accounts, args)
}
pub fn open_position_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: OpenPositionAccounts<'_, '_>,
    args: OpenPositionIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: OpenPositionKeys = accounts.into();
    let ix = open_position_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn open_position_invoke_signed(
    accounts: OpenPositionAccounts<'_, '_>,
    args: OpenPositionIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    open_position_invoke_signed_with_program_id(crate::ID, accounts, args, seeds)
}
pub fn open_position_verify_account_keys(
    accounts: OpenPositionAccounts<'_, '_>,
    keys: OpenPositionKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.funder.key, keys.funder),
        (*accounts.owner.key, keys.owner),
        (*accounts.position.key, keys.position),
        (*accounts.position_mint.key, keys.position_mint),
        (*accounts.position_token_account.key, keys.position_token_account),
        (*accounts.whirlpool.key, keys.whirlpool),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.rent.key, keys.rent),
        (*accounts.associated_token_program.key, keys.associated_token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn open_position_verify_writable_privileges<'me, 'info>(
    accounts: OpenPositionAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.funder,
        accounts.position,
        accounts.position_mint,
        accounts.position_token_account,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn open_position_verify_signer_privileges<'me, 'info>(
    accounts: OpenPositionAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.funder, accounts.position_mint] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn open_position_verify_account_privileges<'me, 'info>(
    accounts: OpenPositionAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    open_position_verify_writable_privileges(accounts)?;
    open_position_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const OPEN_POSITION_WITH_METADATA_IX_ACCOUNTS_LEN: usize = 13;
#[derive(Copy, Clone, Debug)]
pub struct OpenPositionWithMetadataAccounts<'me, 'info> {
    pub funder: &'me AccountInfo<'info>,
    pub owner: &'me AccountInfo<'info>,
    pub position: &'me AccountInfo<'info>,
    pub position_mint: &'me AccountInfo<'info>,
    pub position_metadata_account: &'me AccountInfo<'info>,
    pub position_token_account: &'me AccountInfo<'info>,
    pub whirlpool: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub rent: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
    pub metadata_program: &'me AccountInfo<'info>,
    pub metadata_update_auth: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct OpenPositionWithMetadataKeys {
    pub funder: Pubkey,
    pub owner: Pubkey,
    pub position: Pubkey,
    pub position_mint: Pubkey,
    pub position_metadata_account: Pubkey,
    pub position_token_account: Pubkey,
    pub whirlpool: Pubkey,
    pub token_program: Pubkey,
    pub system_program: Pubkey,
    pub rent: Pubkey,
    pub associated_token_program: Pubkey,
    pub metadata_program: Pubkey,
    pub metadata_update_auth: Pubkey,
}
impl From<OpenPositionWithMetadataAccounts<'_, '_>> for OpenPositionWithMetadataKeys {
    fn from(accounts: OpenPositionWithMetadataAccounts) -> Self {
        Self {
            funder: *accounts.funder.key,
            owner: *accounts.owner.key,
            position: *accounts.position.key,
            position_mint: *accounts.position_mint.key,
            position_metadata_account: *accounts.position_metadata_account.key,
            position_token_account: *accounts.position_token_account.key,
            whirlpool: *accounts.whirlpool.key,
            token_program: *accounts.token_program.key,
            system_program: *accounts.system_program.key,
            rent: *accounts.rent.key,
            associated_token_program: *accounts.associated_token_program.key,
            metadata_program: *accounts.metadata_program.key,
            metadata_update_auth: *accounts.metadata_update_auth.key,
        }
    }
}
impl From<OpenPositionWithMetadataKeys>
for [AccountMeta; OPEN_POSITION_WITH_METADATA_IX_ACCOUNTS_LEN] {
    fn from(keys: OpenPositionWithMetadataKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.funder,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.owner,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.position,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.position_mint,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.position_metadata_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.position_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.whirlpool,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.system_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.rent,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.associated_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.metadata_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.metadata_update_auth,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; OPEN_POSITION_WITH_METADATA_IX_ACCOUNTS_LEN]>
for OpenPositionWithMetadataKeys {
    fn from(pubkeys: [Pubkey; OPEN_POSITION_WITH_METADATA_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            funder: pubkeys[0],
            owner: pubkeys[1],
            position: pubkeys[2],
            position_mint: pubkeys[3],
            position_metadata_account: pubkeys[4],
            position_token_account: pubkeys[5],
            whirlpool: pubkeys[6],
            token_program: pubkeys[7],
            system_program: pubkeys[8],
            rent: pubkeys[9],
            associated_token_program: pubkeys[10],
            metadata_program: pubkeys[11],
            metadata_update_auth: pubkeys[12],
        }
    }
}
impl<'info> From<OpenPositionWithMetadataAccounts<'_, 'info>>
for [AccountInfo<'info>; OPEN_POSITION_WITH_METADATA_IX_ACCOUNTS_LEN] {
    fn from(accounts: OpenPositionWithMetadataAccounts<'_, 'info>) -> Self {
        [
            accounts.funder.clone(),
            accounts.owner.clone(),
            accounts.position.clone(),
            accounts.position_mint.clone(),
            accounts.position_metadata_account.clone(),
            accounts.position_token_account.clone(),
            accounts.whirlpool.clone(),
            accounts.token_program.clone(),
            accounts.system_program.clone(),
            accounts.rent.clone(),
            accounts.associated_token_program.clone(),
            accounts.metadata_program.clone(),
            accounts.metadata_update_auth.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; OPEN_POSITION_WITH_METADATA_IX_ACCOUNTS_LEN]>
for OpenPositionWithMetadataAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; OPEN_POSITION_WITH_METADATA_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            funder: &arr[0],
            owner: &arr[1],
            position: &arr[2],
            position_mint: &arr[3],
            position_metadata_account: &arr[4],
            position_token_account: &arr[5],
            whirlpool: &arr[6],
            token_program: &arr[7],
            system_program: &arr[8],
            rent: &arr[9],
            associated_token_program: &arr[10],
            metadata_program: &arr[11],
            metadata_update_auth: &arr[12],
        }
    }
}
pub const OPEN_POSITION_WITH_METADATA_IX_DISCM: [u8; 8] = [
    242,
    29,
    134,
    48,
    58,
    110,
    14,
    60,
];
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct OpenPositionWithMetadataIxArgs {
    pub bumps: OpenPositionWithMetadataBumps,
    pub tick_lower_index: i32,
    pub tick_upper_index: i32,
}
#[derive(Clone, Debug, PartialEq)]
pub struct OpenPositionWithMetadataIxData(pub OpenPositionWithMetadataIxArgs);
impl From<OpenPositionWithMetadataIxArgs> for OpenPositionWithMetadataIxData {
    fn from(args: OpenPositionWithMetadataIxArgs) -> Self {
        Self(args)
    }
}
impl OpenPositionWithMetadataIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != OPEN_POSITION_WITH_METADATA_IX_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        OPEN_POSITION_WITH_METADATA_IX_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(OpenPositionWithMetadataIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&OPEN_POSITION_WITH_METADATA_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn open_position_with_metadata_ix_with_program_id(
    program_id: Pubkey,
    keys: OpenPositionWithMetadataKeys,
    args: OpenPositionWithMetadataIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; OPEN_POSITION_WITH_METADATA_IX_ACCOUNTS_LEN] = keys.into();
    let data: OpenPositionWithMetadataIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn open_position_with_metadata_ix(
    keys: OpenPositionWithMetadataKeys,
    args: OpenPositionWithMetadataIxArgs,
) -> std::io::Result<Instruction> {
    open_position_with_metadata_ix_with_program_id(crate::ID, keys, args)
}
pub fn open_position_with_metadata_invoke_with_program_id(
    program_id: Pubkey,
    accounts: OpenPositionWithMetadataAccounts<'_, '_>,
    args: OpenPositionWithMetadataIxArgs,
) -> ProgramResult {
    let keys: OpenPositionWithMetadataKeys = accounts.into();
    let ix = open_position_with_metadata_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn open_position_with_metadata_invoke(
    accounts: OpenPositionWithMetadataAccounts<'_, '_>,
    args: OpenPositionWithMetadataIxArgs,
) -> ProgramResult {
    open_position_with_metadata_invoke_with_program_id(crate::ID, accounts, args)
}
pub fn open_position_with_metadata_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: OpenPositionWithMetadataAccounts<'_, '_>,
    args: OpenPositionWithMetadataIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: OpenPositionWithMetadataKeys = accounts.into();
    let ix = open_position_with_metadata_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn open_position_with_metadata_invoke_signed(
    accounts: OpenPositionWithMetadataAccounts<'_, '_>,
    args: OpenPositionWithMetadataIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    open_position_with_metadata_invoke_signed_with_program_id(
        crate::ID,
        accounts,
        args,
        seeds,
    )
}
pub fn open_position_with_metadata_verify_account_keys(
    accounts: OpenPositionWithMetadataAccounts<'_, '_>,
    keys: OpenPositionWithMetadataKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.funder.key, keys.funder),
        (*accounts.owner.key, keys.owner),
        (*accounts.position.key, keys.position),
        (*accounts.position_mint.key, keys.position_mint),
        (*accounts.position_metadata_account.key, keys.position_metadata_account),
        (*accounts.position_token_account.key, keys.position_token_account),
        (*accounts.whirlpool.key, keys.whirlpool),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.rent.key, keys.rent),
        (*accounts.associated_token_program.key, keys.associated_token_program),
        (*accounts.metadata_program.key, keys.metadata_program),
        (*accounts.metadata_update_auth.key, keys.metadata_update_auth),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn open_position_with_metadata_verify_writable_privileges<'me, 'info>(
    accounts: OpenPositionWithMetadataAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.funder,
        accounts.position,
        accounts.position_mint,
        accounts.position_metadata_account,
        accounts.position_token_account,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn open_position_with_metadata_verify_signer_privileges<'me, 'info>(
    accounts: OpenPositionWithMetadataAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.funder, accounts.position_mint] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn open_position_with_metadata_verify_account_privileges<'me, 'info>(
    accounts: OpenPositionWithMetadataAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    open_position_with_metadata_verify_writable_privileges(accounts)?;
    open_position_with_metadata_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const INCREASE_LIQUIDITY_IX_ACCOUNTS_LEN: usize = 11;
#[derive(Copy, Clone, Debug)]
pub struct IncreaseLiquidityAccounts<'me, 'info> {
    pub whirlpool: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub position_authority: &'me AccountInfo<'info>,
    pub position: &'me AccountInfo<'info>,
    pub position_token_account: &'me AccountInfo<'info>,
    pub token_owner_account_a: &'me AccountInfo<'info>,
    pub token_owner_account_b: &'me AccountInfo<'info>,
    pub token_vault_a: &'me AccountInfo<'info>,
    pub token_vault_b: &'me AccountInfo<'info>,
    pub tick_array_lower: &'me AccountInfo<'info>,
    pub tick_array_upper: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct IncreaseLiquidityKeys {
    pub whirlpool: Pubkey,
    pub token_program: Pubkey,
    pub position_authority: Pubkey,
    pub position: Pubkey,
    pub position_token_account: Pubkey,
    pub token_owner_account_a: Pubkey,
    pub token_owner_account_b: Pubkey,
    pub token_vault_a: Pubkey,
    pub token_vault_b: Pubkey,
    pub tick_array_lower: Pubkey,
    pub tick_array_upper: Pubkey,
}
impl From<IncreaseLiquidityAccounts<'_, '_>> for IncreaseLiquidityKeys {
    fn from(accounts: IncreaseLiquidityAccounts) -> Self {
        Self {
            whirlpool: *accounts.whirlpool.key,
            token_program: *accounts.token_program.key,
            position_authority: *accounts.position_authority.key,
            position: *accounts.position.key,
            position_token_account: *accounts.position_token_account.key,
            token_owner_account_a: *accounts.token_owner_account_a.key,
            token_owner_account_b: *accounts.token_owner_account_b.key,
            token_vault_a: *accounts.token_vault_a.key,
            token_vault_b: *accounts.token_vault_b.key,
            tick_array_lower: *accounts.tick_array_lower.key,
            tick_array_upper: *accounts.tick_array_upper.key,
        }
    }
}
impl From<IncreaseLiquidityKeys> for [AccountMeta; INCREASE_LIQUIDITY_IX_ACCOUNTS_LEN] {
    fn from(keys: IncreaseLiquidityKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.whirlpool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.position_authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.position,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.position_token_account,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_owner_account_a,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_owner_account_b,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_vault_a,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_vault_b,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.tick_array_lower,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.tick_array_upper,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; INCREASE_LIQUIDITY_IX_ACCOUNTS_LEN]> for IncreaseLiquidityKeys {
    fn from(pubkeys: [Pubkey; INCREASE_LIQUIDITY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            whirlpool: pubkeys[0],
            token_program: pubkeys[1],
            position_authority: pubkeys[2],
            position: pubkeys[3],
            position_token_account: pubkeys[4],
            token_owner_account_a: pubkeys[5],
            token_owner_account_b: pubkeys[6],
            token_vault_a: pubkeys[7],
            token_vault_b: pubkeys[8],
            tick_array_lower: pubkeys[9],
            tick_array_upper: pubkeys[10],
        }
    }
}
impl<'info> From<IncreaseLiquidityAccounts<'_, 'info>>
for [AccountInfo<'info>; INCREASE_LIQUIDITY_IX_ACCOUNTS_LEN] {
    fn from(accounts: IncreaseLiquidityAccounts<'_, 'info>) -> Self {
        [
            accounts.whirlpool.clone(),
            accounts.token_program.clone(),
            accounts.position_authority.clone(),
            accounts.position.clone(),
            accounts.position_token_account.clone(),
            accounts.token_owner_account_a.clone(),
            accounts.token_owner_account_b.clone(),
            accounts.token_vault_a.clone(),
            accounts.token_vault_b.clone(),
            accounts.tick_array_lower.clone(),
            accounts.tick_array_upper.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; INCREASE_LIQUIDITY_IX_ACCOUNTS_LEN]>
for IncreaseLiquidityAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; INCREASE_LIQUIDITY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            whirlpool: &arr[0],
            token_program: &arr[1],
            position_authority: &arr[2],
            position: &arr[3],
            position_token_account: &arr[4],
            token_owner_account_a: &arr[5],
            token_owner_account_b: &arr[6],
            token_vault_a: &arr[7],
            token_vault_b: &arr[8],
            tick_array_lower: &arr[9],
            tick_array_upper: &arr[10],
        }
    }
}
pub const INCREASE_LIQUIDITY_IX_DISCM: [u8; 8] = [46, 156, 243, 118, 13, 205, 251, 178];
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct IncreaseLiquidityIxArgs {
    pub liquidity_amount: u128,
    pub token_max_a: u64,
    pub token_max_b: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct IncreaseLiquidityIxData(pub IncreaseLiquidityIxArgs);
impl From<IncreaseLiquidityIxArgs> for IncreaseLiquidityIxData {
    fn from(args: IncreaseLiquidityIxArgs) -> Self {
        Self(args)
    }
}
impl IncreaseLiquidityIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INCREASE_LIQUIDITY_IX_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        INCREASE_LIQUIDITY_IX_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(IncreaseLiquidityIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INCREASE_LIQUIDITY_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn increase_liquidity_ix_with_program_id(
    program_id: Pubkey,
    keys: IncreaseLiquidityKeys,
    args: IncreaseLiquidityIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INCREASE_LIQUIDITY_IX_ACCOUNTS_LEN] = keys.into();
    let data: IncreaseLiquidityIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn increase_liquidity_ix(
    keys: IncreaseLiquidityKeys,
    args: IncreaseLiquidityIxArgs,
) -> std::io::Result<Instruction> {
    increase_liquidity_ix_with_program_id(crate::ID, keys, args)
}
pub fn increase_liquidity_invoke_with_program_id(
    program_id: Pubkey,
    accounts: IncreaseLiquidityAccounts<'_, '_>,
    args: IncreaseLiquidityIxArgs,
) -> ProgramResult {
    let keys: IncreaseLiquidityKeys = accounts.into();
    let ix = increase_liquidity_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn increase_liquidity_invoke(
    accounts: IncreaseLiquidityAccounts<'_, '_>,
    args: IncreaseLiquidityIxArgs,
) -> ProgramResult {
    increase_liquidity_invoke_with_program_id(crate::ID, accounts, args)
}
pub fn increase_liquidity_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: IncreaseLiquidityAccounts<'_, '_>,
    args: IncreaseLiquidityIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: IncreaseLiquidityKeys = accounts.into();
    let ix = increase_liquidity_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn increase_liquidity_invoke_signed(
    accounts: IncreaseLiquidityAccounts<'_, '_>,
    args: IncreaseLiquidityIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    increase_liquidity_invoke_signed_with_program_id(crate::ID, accounts, args, seeds)
}
pub fn increase_liquidity_verify_account_keys(
    accounts: IncreaseLiquidityAccounts<'_, '_>,
    keys: IncreaseLiquidityKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.whirlpool.key, keys.whirlpool),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.position_authority.key, keys.position_authority),
        (*accounts.position.key, keys.position),
        (*accounts.position_token_account.key, keys.position_token_account),
        (*accounts.token_owner_account_a.key, keys.token_owner_account_a),
        (*accounts.token_owner_account_b.key, keys.token_owner_account_b),
        (*accounts.token_vault_a.key, keys.token_vault_a),
        (*accounts.token_vault_b.key, keys.token_vault_b),
        (*accounts.tick_array_lower.key, keys.tick_array_lower),
        (*accounts.tick_array_upper.key, keys.tick_array_upper),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn increase_liquidity_verify_writable_privileges<'me, 'info>(
    accounts: IncreaseLiquidityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.whirlpool,
        accounts.position,
        accounts.token_owner_account_a,
        accounts.token_owner_account_b,
        accounts.token_vault_a,
        accounts.token_vault_b,
        accounts.tick_array_lower,
        accounts.tick_array_upper,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn increase_liquidity_verify_signer_privileges<'me, 'info>(
    accounts: IncreaseLiquidityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.position_authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn increase_liquidity_verify_account_privileges<'me, 'info>(
    accounts: IncreaseLiquidityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    increase_liquidity_verify_writable_privileges(accounts)?;
    increase_liquidity_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const DECREASE_LIQUIDITY_IX_ACCOUNTS_LEN: usize = 11;
#[derive(Copy, Clone, Debug)]
pub struct DecreaseLiquidityAccounts<'me, 'info> {
    pub whirlpool: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub position_authority: &'me AccountInfo<'info>,
    pub position: &'me AccountInfo<'info>,
    pub position_token_account: &'me AccountInfo<'info>,
    pub token_owner_account_a: &'me AccountInfo<'info>,
    pub token_owner_account_b: &'me AccountInfo<'info>,
    pub token_vault_a: &'me AccountInfo<'info>,
    pub token_vault_b: &'me AccountInfo<'info>,
    pub tick_array_lower: &'me AccountInfo<'info>,
    pub tick_array_upper: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DecreaseLiquidityKeys {
    pub whirlpool: Pubkey,
    pub token_program: Pubkey,
    pub position_authority: Pubkey,
    pub position: Pubkey,
    pub position_token_account: Pubkey,
    pub token_owner_account_a: Pubkey,
    pub token_owner_account_b: Pubkey,
    pub token_vault_a: Pubkey,
    pub token_vault_b: Pubkey,
    pub tick_array_lower: Pubkey,
    pub tick_array_upper: Pubkey,
}
impl From<DecreaseLiquidityAccounts<'_, '_>> for DecreaseLiquidityKeys {
    fn from(accounts: DecreaseLiquidityAccounts) -> Self {
        Self {
            whirlpool: *accounts.whirlpool.key,
            token_program: *accounts.token_program.key,
            position_authority: *accounts.position_authority.key,
            position: *accounts.position.key,
            position_token_account: *accounts.position_token_account.key,
            token_owner_account_a: *accounts.token_owner_account_a.key,
            token_owner_account_b: *accounts.token_owner_account_b.key,
            token_vault_a: *accounts.token_vault_a.key,
            token_vault_b: *accounts.token_vault_b.key,
            tick_array_lower: *accounts.tick_array_lower.key,
            tick_array_upper: *accounts.tick_array_upper.key,
        }
    }
}
impl From<DecreaseLiquidityKeys> for [AccountMeta; DECREASE_LIQUIDITY_IX_ACCOUNTS_LEN] {
    fn from(keys: DecreaseLiquidityKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.whirlpool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.position_authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.position,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.position_token_account,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_owner_account_a,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_owner_account_b,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_vault_a,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_vault_b,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.tick_array_lower,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.tick_array_upper,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; DECREASE_LIQUIDITY_IX_ACCOUNTS_LEN]> for DecreaseLiquidityKeys {
    fn from(pubkeys: [Pubkey; DECREASE_LIQUIDITY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            whirlpool: pubkeys[0],
            token_program: pubkeys[1],
            position_authority: pubkeys[2],
            position: pubkeys[3],
            position_token_account: pubkeys[4],
            token_owner_account_a: pubkeys[5],
            token_owner_account_b: pubkeys[6],
            token_vault_a: pubkeys[7],
            token_vault_b: pubkeys[8],
            tick_array_lower: pubkeys[9],
            tick_array_upper: pubkeys[10],
        }
    }
}
impl<'info> From<DecreaseLiquidityAccounts<'_, 'info>>
for [AccountInfo<'info>; DECREASE_LIQUIDITY_IX_ACCOUNTS_LEN] {
    fn from(accounts: DecreaseLiquidityAccounts<'_, 'info>) -> Self {
        [
            accounts.whirlpool.clone(),
            accounts.token_program.clone(),
            accounts.position_authority.clone(),
            accounts.position.clone(),
            accounts.position_token_account.clone(),
            accounts.token_owner_account_a.clone(),
            accounts.token_owner_account_b.clone(),
            accounts.token_vault_a.clone(),
            accounts.token_vault_b.clone(),
            accounts.tick_array_lower.clone(),
            accounts.tick_array_upper.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; DECREASE_LIQUIDITY_IX_ACCOUNTS_LEN]>
for DecreaseLiquidityAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; DECREASE_LIQUIDITY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            whirlpool: &arr[0],
            token_program: &arr[1],
            position_authority: &arr[2],
            position: &arr[3],
            position_token_account: &arr[4],
            token_owner_account_a: &arr[5],
            token_owner_account_b: &arr[6],
            token_vault_a: &arr[7],
            token_vault_b: &arr[8],
            tick_array_lower: &arr[9],
            tick_array_upper: &arr[10],
        }
    }
}
pub const DECREASE_LIQUIDITY_IX_DISCM: [u8; 8] = [160, 38, 208, 111, 104, 91, 44, 1];
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DecreaseLiquidityIxArgs {
    pub liquidity_amount: u128,
    pub token_min_a: u64,
    pub token_min_b: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct DecreaseLiquidityIxData(pub DecreaseLiquidityIxArgs);
impl From<DecreaseLiquidityIxArgs> for DecreaseLiquidityIxData {
    fn from(args: DecreaseLiquidityIxArgs) -> Self {
        Self(args)
    }
}
impl DecreaseLiquidityIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != DECREASE_LIQUIDITY_IX_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        DECREASE_LIQUIDITY_IX_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(DecreaseLiquidityIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&DECREASE_LIQUIDITY_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn decrease_liquidity_ix_with_program_id(
    program_id: Pubkey,
    keys: DecreaseLiquidityKeys,
    args: DecreaseLiquidityIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; DECREASE_LIQUIDITY_IX_ACCOUNTS_LEN] = keys.into();
    let data: DecreaseLiquidityIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn decrease_liquidity_ix(
    keys: DecreaseLiquidityKeys,
    args: DecreaseLiquidityIxArgs,
) -> std::io::Result<Instruction> {
    decrease_liquidity_ix_with_program_id(crate::ID, keys, args)
}
pub fn decrease_liquidity_invoke_with_program_id(
    program_id: Pubkey,
    accounts: DecreaseLiquidityAccounts<'_, '_>,
    args: DecreaseLiquidityIxArgs,
) -> ProgramResult {
    let keys: DecreaseLiquidityKeys = accounts.into();
    let ix = decrease_liquidity_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn decrease_liquidity_invoke(
    accounts: DecreaseLiquidityAccounts<'_, '_>,
    args: DecreaseLiquidityIxArgs,
) -> ProgramResult {
    decrease_liquidity_invoke_with_program_id(crate::ID, accounts, args)
}
pub fn decrease_liquidity_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: DecreaseLiquidityAccounts<'_, '_>,
    args: DecreaseLiquidityIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: DecreaseLiquidityKeys = accounts.into();
    let ix = decrease_liquidity_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn decrease_liquidity_invoke_signed(
    accounts: DecreaseLiquidityAccounts<'_, '_>,
    args: DecreaseLiquidityIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    decrease_liquidity_invoke_signed_with_program_id(crate::ID, accounts, args, seeds)
}
pub fn decrease_liquidity_verify_account_keys(
    accounts: DecreaseLiquidityAccounts<'_, '_>,
    keys: DecreaseLiquidityKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.whirlpool.key, keys.whirlpool),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.position_authority.key, keys.position_authority),
        (*accounts.position.key, keys.position),
        (*accounts.position_token_account.key, keys.position_token_account),
        (*accounts.token_owner_account_a.key, keys.token_owner_account_a),
        (*accounts.token_owner_account_b.key, keys.token_owner_account_b),
        (*accounts.token_vault_a.key, keys.token_vault_a),
        (*accounts.token_vault_b.key, keys.token_vault_b),
        (*accounts.tick_array_lower.key, keys.tick_array_lower),
        (*accounts.tick_array_upper.key, keys.tick_array_upper),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn decrease_liquidity_verify_writable_privileges<'me, 'info>(
    accounts: DecreaseLiquidityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.whirlpool,
        accounts.position,
        accounts.token_owner_account_a,
        accounts.token_owner_account_b,
        accounts.token_vault_a,
        accounts.token_vault_b,
        accounts.tick_array_lower,
        accounts.tick_array_upper,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn decrease_liquidity_verify_signer_privileges<'me, 'info>(
    accounts: DecreaseLiquidityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.position_authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn decrease_liquidity_verify_account_privileges<'me, 'info>(
    accounts: DecreaseLiquidityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    decrease_liquidity_verify_writable_privileges(accounts)?;
    decrease_liquidity_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const UPDATE_FEES_AND_REWARDS_IX_ACCOUNTS_LEN: usize = 4;
#[derive(Copy, Clone, Debug)]
pub struct UpdateFeesAndRewardsAccounts<'me, 'info> {
    pub whirlpool: &'me AccountInfo<'info>,
    pub position: &'me AccountInfo<'info>,
    pub tick_array_lower: &'me AccountInfo<'info>,
    pub tick_array_upper: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdateFeesAndRewardsKeys {
    pub whirlpool: Pubkey,
    pub position: Pubkey,
    pub tick_array_lower: Pubkey,
    pub tick_array_upper: Pubkey,
}
impl From<UpdateFeesAndRewardsAccounts<'_, '_>> for UpdateFeesAndRewardsKeys {
    fn from(accounts: UpdateFeesAndRewardsAccounts) -> Self {
        Self {
            whirlpool: *accounts.whirlpool.key,
            position: *accounts.position.key,
            tick_array_lower: *accounts.tick_array_lower.key,
            tick_array_upper: *accounts.tick_array_upper.key,
        }
    }
}
impl From<UpdateFeesAndRewardsKeys>
for [AccountMeta; UPDATE_FEES_AND_REWARDS_IX_ACCOUNTS_LEN] {
    fn from(keys: UpdateFeesAndRewardsKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.whirlpool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.position,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.tick_array_lower,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.tick_array_upper,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; UPDATE_FEES_AND_REWARDS_IX_ACCOUNTS_LEN]>
for UpdateFeesAndRewardsKeys {
    fn from(pubkeys: [Pubkey; UPDATE_FEES_AND_REWARDS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            whirlpool: pubkeys[0],
            position: pubkeys[1],
            tick_array_lower: pubkeys[2],
            tick_array_upper: pubkeys[3],
        }
    }
}
impl<'info> From<UpdateFeesAndRewardsAccounts<'_, 'info>>
for [AccountInfo<'info>; UPDATE_FEES_AND_REWARDS_IX_ACCOUNTS_LEN] {
    fn from(accounts: UpdateFeesAndRewardsAccounts<'_, 'info>) -> Self {
        [
            accounts.whirlpool.clone(),
            accounts.position.clone(),
            accounts.tick_array_lower.clone(),
            accounts.tick_array_upper.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; UPDATE_FEES_AND_REWARDS_IX_ACCOUNTS_LEN]>
for UpdateFeesAndRewardsAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; UPDATE_FEES_AND_REWARDS_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            whirlpool: &arr[0],
            position: &arr[1],
            tick_array_lower: &arr[2],
            tick_array_upper: &arr[3],
        }
    }
}
pub const UPDATE_FEES_AND_REWARDS_IX_DISCM: [u8; 8] = [
    154,
    230,
    250,
    13,
    236,
    209,
    75,
    223,
];
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateFeesAndRewardsIxData;
impl UpdateFeesAndRewardsIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_FEES_AND_REWARDS_IX_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        UPDATE_FEES_AND_REWARDS_IX_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_FEES_AND_REWARDS_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn update_fees_and_rewards_ix_with_program_id(
    program_id: Pubkey,
    keys: UpdateFeesAndRewardsKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; UPDATE_FEES_AND_REWARDS_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: UpdateFeesAndRewardsIxData.try_to_vec()?,
    })
}
pub fn update_fees_and_rewards_ix(
    keys: UpdateFeesAndRewardsKeys,
) -> std::io::Result<Instruction> {
    update_fees_and_rewards_ix_with_program_id(crate::ID, keys)
}
pub fn update_fees_and_rewards_invoke_with_program_id(
    program_id: Pubkey,
    accounts: UpdateFeesAndRewardsAccounts<'_, '_>,
) -> ProgramResult {
    let keys: UpdateFeesAndRewardsKeys = accounts.into();
    let ix = update_fees_and_rewards_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn update_fees_and_rewards_invoke(
    accounts: UpdateFeesAndRewardsAccounts<'_, '_>,
) -> ProgramResult {
    update_fees_and_rewards_invoke_with_program_id(crate::ID, accounts)
}
pub fn update_fees_and_rewards_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: UpdateFeesAndRewardsAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: UpdateFeesAndRewardsKeys = accounts.into();
    let ix = update_fees_and_rewards_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn update_fees_and_rewards_invoke_signed(
    accounts: UpdateFeesAndRewardsAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    update_fees_and_rewards_invoke_signed_with_program_id(crate::ID, accounts, seeds)
}
pub fn update_fees_and_rewards_verify_account_keys(
    accounts: UpdateFeesAndRewardsAccounts<'_, '_>,
    keys: UpdateFeesAndRewardsKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.whirlpool.key, keys.whirlpool),
        (*accounts.position.key, keys.position),
        (*accounts.tick_array_lower.key, keys.tick_array_lower),
        (*accounts.tick_array_upper.key, keys.tick_array_upper),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn update_fees_and_rewards_verify_writable_privileges<'me, 'info>(
    accounts: UpdateFeesAndRewardsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.whirlpool, accounts.position] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn update_fees_and_rewards_verify_account_privileges<'me, 'info>(
    accounts: UpdateFeesAndRewardsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    update_fees_and_rewards_verify_writable_privileges(accounts)?;
    Ok(())
}
pub const COLLECT_FEES_IX_ACCOUNTS_LEN: usize = 9;
#[derive(Copy, Clone, Debug)]
pub struct CollectFeesAccounts<'me, 'info> {
    pub whirlpool: &'me AccountInfo<'info>,
    pub position_authority: &'me AccountInfo<'info>,
    pub position: &'me AccountInfo<'info>,
    pub position_token_account: &'me AccountInfo<'info>,
    pub token_owner_account_a: &'me AccountInfo<'info>,
    pub token_vault_a: &'me AccountInfo<'info>,
    pub token_owner_account_b: &'me AccountInfo<'info>,
    pub token_vault_b: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CollectFeesKeys {
    pub whirlpool: Pubkey,
    pub position_authority: Pubkey,
    pub position: Pubkey,
    pub position_token_account: Pubkey,
    pub token_owner_account_a: Pubkey,
    pub token_vault_a: Pubkey,
    pub token_owner_account_b: Pubkey,
    pub token_vault_b: Pubkey,
    pub token_program: Pubkey,
}
impl From<CollectFeesAccounts<'_, '_>> for CollectFeesKeys {
    fn from(accounts: CollectFeesAccounts) -> Self {
        Self {
            whirlpool: *accounts.whirlpool.key,
            position_authority: *accounts.position_authority.key,
            position: *accounts.position.key,
            position_token_account: *accounts.position_token_account.key,
            token_owner_account_a: *accounts.token_owner_account_a.key,
            token_vault_a: *accounts.token_vault_a.key,
            token_owner_account_b: *accounts.token_owner_account_b.key,
            token_vault_b: *accounts.token_vault_b.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<CollectFeesKeys> for [AccountMeta; COLLECT_FEES_IX_ACCOUNTS_LEN] {
    fn from(keys: CollectFeesKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.whirlpool,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.position_authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.position,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.position_token_account,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_owner_account_a,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_vault_a,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_owner_account_b,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_vault_b,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; COLLECT_FEES_IX_ACCOUNTS_LEN]> for CollectFeesKeys {
    fn from(pubkeys: [Pubkey; COLLECT_FEES_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            whirlpool: pubkeys[0],
            position_authority: pubkeys[1],
            position: pubkeys[2],
            position_token_account: pubkeys[3],
            token_owner_account_a: pubkeys[4],
            token_vault_a: pubkeys[5],
            token_owner_account_b: pubkeys[6],
            token_vault_b: pubkeys[7],
            token_program: pubkeys[8],
        }
    }
}
impl<'info> From<CollectFeesAccounts<'_, 'info>>
for [AccountInfo<'info>; COLLECT_FEES_IX_ACCOUNTS_LEN] {
    fn from(accounts: CollectFeesAccounts<'_, 'info>) -> Self {
        [
            accounts.whirlpool.clone(),
            accounts.position_authority.clone(),
            accounts.position.clone(),
            accounts.position_token_account.clone(),
            accounts.token_owner_account_a.clone(),
            accounts.token_vault_a.clone(),
            accounts.token_owner_account_b.clone(),
            accounts.token_vault_b.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; COLLECT_FEES_IX_ACCOUNTS_LEN]>
for CollectFeesAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; COLLECT_FEES_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            whirlpool: &arr[0],
            position_authority: &arr[1],
            position: &arr[2],
            position_token_account: &arr[3],
            token_owner_account_a: &arr[4],
            token_vault_a: &arr[5],
            token_owner_account_b: &arr[6],
            token_vault_b: &arr[7],
            token_program: &arr[8],
        }
    }
}
pub const COLLECT_FEES_IX_DISCM: [u8; 8] = [164, 152, 207, 99, 30, 186, 19, 182];
#[derive(Clone, Debug, PartialEq)]
pub struct CollectFeesIxData;
impl CollectFeesIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != COLLECT_FEES_IX_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        COLLECT_FEES_IX_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&COLLECT_FEES_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn collect_fees_ix_with_program_id(
    program_id: Pubkey,
    keys: CollectFeesKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; COLLECT_FEES_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: CollectFeesIxData.try_to_vec()?,
    })
}
pub fn collect_fees_ix(keys: CollectFeesKeys) -> std::io::Result<Instruction> {
    collect_fees_ix_with_program_id(crate::ID, keys)
}
pub fn collect_fees_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CollectFeesAccounts<'_, '_>,
) -> ProgramResult {
    let keys: CollectFeesKeys = accounts.into();
    let ix = collect_fees_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn collect_fees_invoke(accounts: CollectFeesAccounts<'_, '_>) -> ProgramResult {
    collect_fees_invoke_with_program_id(crate::ID, accounts)
}
pub fn collect_fees_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CollectFeesAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CollectFeesKeys = accounts.into();
    let ix = collect_fees_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn collect_fees_invoke_signed(
    accounts: CollectFeesAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    collect_fees_invoke_signed_with_program_id(crate::ID, accounts, seeds)
}
pub fn collect_fees_verify_account_keys(
    accounts: CollectFeesAccounts<'_, '_>,
    keys: CollectFeesKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.whirlpool.key, keys.whirlpool),
        (*accounts.position_authority.key, keys.position_authority),
        (*accounts.position.key, keys.position),
        (*accounts.position_token_account.key, keys.position_token_account),
        (*accounts.token_owner_account_a.key, keys.token_owner_account_a),
        (*accounts.token_vault_a.key, keys.token_vault_a),
        (*accounts.token_owner_account_b.key, keys.token_owner_account_b),
        (*accounts.token_vault_b.key, keys.token_vault_b),
        (*accounts.token_program.key, keys.token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn collect_fees_verify_writable_privileges<'me, 'info>(
    accounts: CollectFeesAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.position,
        accounts.token_owner_account_a,
        accounts.token_vault_a,
        accounts.token_owner_account_b,
        accounts.token_vault_b,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn collect_fees_verify_signer_privileges<'me, 'info>(
    accounts: CollectFeesAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.position_authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn collect_fees_verify_account_privileges<'me, 'info>(
    accounts: CollectFeesAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    collect_fees_verify_writable_privileges(accounts)?;
    collect_fees_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const COLLECT_REWARD_IX_ACCOUNTS_LEN: usize = 7;
#[derive(Copy, Clone, Debug)]
pub struct CollectRewardAccounts<'me, 'info> {
    pub whirlpool: &'me AccountInfo<'info>,
    pub position_authority: &'me AccountInfo<'info>,
    pub position: &'me AccountInfo<'info>,
    pub position_token_account: &'me AccountInfo<'info>,
    pub reward_owner_account: &'me AccountInfo<'info>,
    pub reward_vault: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CollectRewardKeys {
    pub whirlpool: Pubkey,
    pub position_authority: Pubkey,
    pub position: Pubkey,
    pub position_token_account: Pubkey,
    pub reward_owner_account: Pubkey,
    pub reward_vault: Pubkey,
    pub token_program: Pubkey,
}
impl From<CollectRewardAccounts<'_, '_>> for CollectRewardKeys {
    fn from(accounts: CollectRewardAccounts) -> Self {
        Self {
            whirlpool: *accounts.whirlpool.key,
            position_authority: *accounts.position_authority.key,
            position: *accounts.position.key,
            position_token_account: *accounts.position_token_account.key,
            reward_owner_account: *accounts.reward_owner_account.key,
            reward_vault: *accounts.reward_vault.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<CollectRewardKeys> for [AccountMeta; COLLECT_REWARD_IX_ACCOUNTS_LEN] {
    fn from(keys: CollectRewardKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.whirlpool,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.position_authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.position,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.position_token_account,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.reward_owner_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.reward_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; COLLECT_REWARD_IX_ACCOUNTS_LEN]> for CollectRewardKeys {
    fn from(pubkeys: [Pubkey; COLLECT_REWARD_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            whirlpool: pubkeys[0],
            position_authority: pubkeys[1],
            position: pubkeys[2],
            position_token_account: pubkeys[3],
            reward_owner_account: pubkeys[4],
            reward_vault: pubkeys[5],
            token_program: pubkeys[6],
        }
    }
}
impl<'info> From<CollectRewardAccounts<'_, 'info>>
for [AccountInfo<'info>; COLLECT_REWARD_IX_ACCOUNTS_LEN] {
    fn from(accounts: CollectRewardAccounts<'_, 'info>) -> Self {
        [
            accounts.whirlpool.clone(),
            accounts.position_authority.clone(),
            accounts.position.clone(),
            accounts.position_token_account.clone(),
            accounts.reward_owner_account.clone(),
            accounts.reward_vault.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; COLLECT_REWARD_IX_ACCOUNTS_LEN]>
for CollectRewardAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; COLLECT_REWARD_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            whirlpool: &arr[0],
            position_authority: &arr[1],
            position: &arr[2],
            position_token_account: &arr[3],
            reward_owner_account: &arr[4],
            reward_vault: &arr[5],
            token_program: &arr[6],
        }
    }
}
pub const COLLECT_REWARD_IX_DISCM: [u8; 8] = [70, 5, 132, 87, 86, 235, 177, 34];
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CollectRewardIxArgs {
    pub reward_index: u8,
}
#[derive(Clone, Debug, PartialEq)]
pub struct CollectRewardIxData(pub CollectRewardIxArgs);
impl From<CollectRewardIxArgs> for CollectRewardIxData {
    fn from(args: CollectRewardIxArgs) -> Self {
        Self(args)
    }
}
impl CollectRewardIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != COLLECT_REWARD_IX_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        COLLECT_REWARD_IX_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(CollectRewardIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&COLLECT_REWARD_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn collect_reward_ix_with_program_id(
    program_id: Pubkey,
    keys: CollectRewardKeys,
    args: CollectRewardIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; COLLECT_REWARD_IX_ACCOUNTS_LEN] = keys.into();
    let data: CollectRewardIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn collect_reward_ix(
    keys: CollectRewardKeys,
    args: CollectRewardIxArgs,
) -> std::io::Result<Instruction> {
    collect_reward_ix_with_program_id(crate::ID, keys, args)
}
pub fn collect_reward_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CollectRewardAccounts<'_, '_>,
    args: CollectRewardIxArgs,
) -> ProgramResult {
    let keys: CollectRewardKeys = accounts.into();
    let ix = collect_reward_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn collect_reward_invoke(
    accounts: CollectRewardAccounts<'_, '_>,
    args: CollectRewardIxArgs,
) -> ProgramResult {
    collect_reward_invoke_with_program_id(crate::ID, accounts, args)
}
pub fn collect_reward_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CollectRewardAccounts<'_, '_>,
    args: CollectRewardIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CollectRewardKeys = accounts.into();
    let ix = collect_reward_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn collect_reward_invoke_signed(
    accounts: CollectRewardAccounts<'_, '_>,
    args: CollectRewardIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    collect_reward_invoke_signed_with_program_id(crate::ID, accounts, args, seeds)
}
pub fn collect_reward_verify_account_keys(
    accounts: CollectRewardAccounts<'_, '_>,
    keys: CollectRewardKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.whirlpool.key, keys.whirlpool),
        (*accounts.position_authority.key, keys.position_authority),
        (*accounts.position.key, keys.position),
        (*accounts.position_token_account.key, keys.position_token_account),
        (*accounts.reward_owner_account.key, keys.reward_owner_account),
        (*accounts.reward_vault.key, keys.reward_vault),
        (*accounts.token_program.key, keys.token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn collect_reward_verify_writable_privileges<'me, 'info>(
    accounts: CollectRewardAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.position,
        accounts.reward_owner_account,
        accounts.reward_vault,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn collect_reward_verify_signer_privileges<'me, 'info>(
    accounts: CollectRewardAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.position_authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn collect_reward_verify_account_privileges<'me, 'info>(
    accounts: CollectRewardAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    collect_reward_verify_writable_privileges(accounts)?;
    collect_reward_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const COLLECT_PROTOCOL_FEES_IX_ACCOUNTS_LEN: usize = 8;
#[derive(Copy, Clone, Debug)]
pub struct CollectProtocolFeesAccounts<'me, 'info> {
    pub whirlpools_config: &'me AccountInfo<'info>,
    pub whirlpool: &'me AccountInfo<'info>,
    pub collect_protocol_fees_authority: &'me AccountInfo<'info>,
    pub token_vault_a: &'me AccountInfo<'info>,
    pub token_vault_b: &'me AccountInfo<'info>,
    pub token_destination_a: &'me AccountInfo<'info>,
    pub token_destination_b: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CollectProtocolFeesKeys {
    pub whirlpools_config: Pubkey,
    pub whirlpool: Pubkey,
    pub collect_protocol_fees_authority: Pubkey,
    pub token_vault_a: Pubkey,
    pub token_vault_b: Pubkey,
    pub token_destination_a: Pubkey,
    pub token_destination_b: Pubkey,
    pub token_program: Pubkey,
}
impl From<CollectProtocolFeesAccounts<'_, '_>> for CollectProtocolFeesKeys {
    fn from(accounts: CollectProtocolFeesAccounts) -> Self {
        Self {
            whirlpools_config: *accounts.whirlpools_config.key,
            whirlpool: *accounts.whirlpool.key,
            collect_protocol_fees_authority: *accounts
                .collect_protocol_fees_authority
                .key,
            token_vault_a: *accounts.token_vault_a.key,
            token_vault_b: *accounts.token_vault_b.key,
            token_destination_a: *accounts.token_destination_a.key,
            token_destination_b: *accounts.token_destination_b.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<CollectProtocolFeesKeys>
for [AccountMeta; COLLECT_PROTOCOL_FEES_IX_ACCOUNTS_LEN] {
    fn from(keys: CollectProtocolFeesKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.whirlpools_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.whirlpool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.collect_protocol_fees_authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_vault_a,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_vault_b,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_destination_a,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_destination_b,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; COLLECT_PROTOCOL_FEES_IX_ACCOUNTS_LEN]> for CollectProtocolFeesKeys {
    fn from(pubkeys: [Pubkey; COLLECT_PROTOCOL_FEES_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            whirlpools_config: pubkeys[0],
            whirlpool: pubkeys[1],
            collect_protocol_fees_authority: pubkeys[2],
            token_vault_a: pubkeys[3],
            token_vault_b: pubkeys[4],
            token_destination_a: pubkeys[5],
            token_destination_b: pubkeys[6],
            token_program: pubkeys[7],
        }
    }
}
impl<'info> From<CollectProtocolFeesAccounts<'_, 'info>>
for [AccountInfo<'info>; COLLECT_PROTOCOL_FEES_IX_ACCOUNTS_LEN] {
    fn from(accounts: CollectProtocolFeesAccounts<'_, 'info>) -> Self {
        [
            accounts.whirlpools_config.clone(),
            accounts.whirlpool.clone(),
            accounts.collect_protocol_fees_authority.clone(),
            accounts.token_vault_a.clone(),
            accounts.token_vault_b.clone(),
            accounts.token_destination_a.clone(),
            accounts.token_destination_b.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; COLLECT_PROTOCOL_FEES_IX_ACCOUNTS_LEN]>
for CollectProtocolFeesAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; COLLECT_PROTOCOL_FEES_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            whirlpools_config: &arr[0],
            whirlpool: &arr[1],
            collect_protocol_fees_authority: &arr[2],
            token_vault_a: &arr[3],
            token_vault_b: &arr[4],
            token_destination_a: &arr[5],
            token_destination_b: &arr[6],
            token_program: &arr[7],
        }
    }
}
pub const COLLECT_PROTOCOL_FEES_IX_DISCM: [u8; 8] = [22, 67, 23, 98, 150, 178, 70, 220];
#[derive(Clone, Debug, PartialEq)]
pub struct CollectProtocolFeesIxData;
impl CollectProtocolFeesIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != COLLECT_PROTOCOL_FEES_IX_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        COLLECT_PROTOCOL_FEES_IX_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&COLLECT_PROTOCOL_FEES_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn collect_protocol_fees_ix_with_program_id(
    program_id: Pubkey,
    keys: CollectProtocolFeesKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; COLLECT_PROTOCOL_FEES_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: CollectProtocolFeesIxData.try_to_vec()?,
    })
}
pub fn collect_protocol_fees_ix(
    keys: CollectProtocolFeesKeys,
) -> std::io::Result<Instruction> {
    collect_protocol_fees_ix_with_program_id(crate::ID, keys)
}
pub fn collect_protocol_fees_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CollectProtocolFeesAccounts<'_, '_>,
) -> ProgramResult {
    let keys: CollectProtocolFeesKeys = accounts.into();
    let ix = collect_protocol_fees_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn collect_protocol_fees_invoke(
    accounts: CollectProtocolFeesAccounts<'_, '_>,
) -> ProgramResult {
    collect_protocol_fees_invoke_with_program_id(crate::ID, accounts)
}
pub fn collect_protocol_fees_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CollectProtocolFeesAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CollectProtocolFeesKeys = accounts.into();
    let ix = collect_protocol_fees_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn collect_protocol_fees_invoke_signed(
    accounts: CollectProtocolFeesAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    collect_protocol_fees_invoke_signed_with_program_id(crate::ID, accounts, seeds)
}
pub fn collect_protocol_fees_verify_account_keys(
    accounts: CollectProtocolFeesAccounts<'_, '_>,
    keys: CollectProtocolFeesKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.whirlpools_config.key, keys.whirlpools_config),
        (*accounts.whirlpool.key, keys.whirlpool),
        (
            *accounts.collect_protocol_fees_authority.key,
            keys.collect_protocol_fees_authority,
        ),
        (*accounts.token_vault_a.key, keys.token_vault_a),
        (*accounts.token_vault_b.key, keys.token_vault_b),
        (*accounts.token_destination_a.key, keys.token_destination_a),
        (*accounts.token_destination_b.key, keys.token_destination_b),
        (*accounts.token_program.key, keys.token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn collect_protocol_fees_verify_writable_privileges<'me, 'info>(
    accounts: CollectProtocolFeesAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.whirlpool,
        accounts.token_vault_a,
        accounts.token_vault_b,
        accounts.token_destination_a,
        accounts.token_destination_b,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn collect_protocol_fees_verify_signer_privileges<'me, 'info>(
    accounts: CollectProtocolFeesAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.collect_protocol_fees_authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn collect_protocol_fees_verify_account_privileges<'me, 'info>(
    accounts: CollectProtocolFeesAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    collect_protocol_fees_verify_writable_privileges(accounts)?;
    collect_protocol_fees_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SWAP_IX_ACCOUNTS_LEN: usize = 11;
#[derive(Copy, Clone, Debug)]
pub struct SwapAccounts<'me, 'info> {
    pub token_program: &'me AccountInfo<'info>,
    pub token_authority: &'me AccountInfo<'info>,
    pub whirlpool: &'me AccountInfo<'info>,
    pub token_owner_account_a: &'me AccountInfo<'info>,
    pub token_vault_a: &'me AccountInfo<'info>,
    pub token_owner_account_b: &'me AccountInfo<'info>,
    pub token_vault_b: &'me AccountInfo<'info>,
    pub tick_array0: &'me AccountInfo<'info>,
    pub tick_array1: &'me AccountInfo<'info>,
    pub tick_array2: &'me AccountInfo<'info>,
    pub oracle: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SwapKeys {
    pub token_program: Pubkey,
    pub token_authority: Pubkey,
    pub whirlpool: Pubkey,
    pub token_owner_account_a: Pubkey,
    pub token_vault_a: Pubkey,
    pub token_owner_account_b: Pubkey,
    pub token_vault_b: Pubkey,
    pub tick_array0: Pubkey,
    pub tick_array1: Pubkey,
    pub tick_array2: Pubkey,
    pub oracle: Pubkey,
}
impl From<SwapAccounts<'_, '_>> for SwapKeys {
    fn from(accounts: SwapAccounts) -> Self {
        Self {
            token_program: *accounts.token_program.key,
            token_authority: *accounts.token_authority.key,
            whirlpool: *accounts.whirlpool.key,
            token_owner_account_a: *accounts.token_owner_account_a.key,
            token_vault_a: *accounts.token_vault_a.key,
            token_owner_account_b: *accounts.token_owner_account_b.key,
            token_vault_b: *accounts.token_vault_b.key,
            tick_array0: *accounts.tick_array0.key,
            tick_array1: *accounts.tick_array1.key,
            tick_array2: *accounts.tick_array2.key,
            oracle: *accounts.oracle.key,
        }
    }
}
impl From<SwapKeys> for [AccountMeta; SWAP_IX_ACCOUNTS_LEN] {
    fn from(keys: SwapKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.whirlpool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_owner_account_a,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_vault_a,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_owner_account_b,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_vault_b,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.tick_array0,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.tick_array1,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.tick_array2,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.oracle,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; SWAP_IX_ACCOUNTS_LEN]> for SwapKeys {
    fn from(pubkeys: [Pubkey; SWAP_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            token_program: pubkeys[0],
            token_authority: pubkeys[1],
            whirlpool: pubkeys[2],
            token_owner_account_a: pubkeys[3],
            token_vault_a: pubkeys[4],
            token_owner_account_b: pubkeys[5],
            token_vault_b: pubkeys[6],
            tick_array0: pubkeys[7],
            tick_array1: pubkeys[8],
            tick_array2: pubkeys[9],
            oracle: pubkeys[10],
        }
    }
}
impl<'info> From<SwapAccounts<'_, 'info>>
for [AccountInfo<'info>; SWAP_IX_ACCOUNTS_LEN] {
    fn from(accounts: SwapAccounts<'_, 'info>) -> Self {
        [
            accounts.token_program.clone(),
            accounts.token_authority.clone(),
            accounts.whirlpool.clone(),
            accounts.token_owner_account_a.clone(),
            accounts.token_vault_a.clone(),
            accounts.token_owner_account_b.clone(),
            accounts.token_vault_b.clone(),
            accounts.tick_array0.clone(),
            accounts.tick_array1.clone(),
            accounts.tick_array2.clone(),
            accounts.oracle.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SWAP_IX_ACCOUNTS_LEN]>
for SwapAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; SWAP_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            token_program: &arr[0],
            token_authority: &arr[1],
            whirlpool: &arr[2],
            token_owner_account_a: &arr[3],
            token_vault_a: &arr[4],
            token_owner_account_b: &arr[5],
            token_vault_b: &arr[6],
            tick_array0: &arr[7],
            tick_array1: &arr[8],
            tick_array2: &arr[9],
            oracle: &arr[10],
        }
    }
}
pub const SWAP_IX_DISCM: [u8; 8] = [248, 198, 158, 145, 225, 117, 135, 200];
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SwapIxArgs {
    pub amount: u64,
    pub other_amount_threshold: u64,
    pub sqrt_price_limit: u128,
    pub amount_specified_is_input: bool,
    pub a_to_b: bool,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SwapIxData(pub SwapIxArgs);
impl From<SwapIxArgs> for SwapIxData {
    fn from(args: SwapIxArgs) -> Self {
        Self(args)
    }
}
impl SwapIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SWAP_IX_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        SWAP_IX_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(SwapIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SWAP_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn swap_ix_with_program_id(
    program_id: Pubkey,
    keys: SwapKeys,
    args: SwapIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SWAP_IX_ACCOUNTS_LEN] = keys.into();
    let data: SwapIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn swap_ix(keys: SwapKeys, args: SwapIxArgs) -> std::io::Result<Instruction> {
    swap_ix_with_program_id(crate::ID, keys, args)
}
pub fn swap_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SwapAccounts<'_, '_>,
    args: SwapIxArgs,
) -> ProgramResult {
    let keys: SwapKeys = accounts.into();
    let ix = swap_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn swap_invoke(accounts: SwapAccounts<'_, '_>, args: SwapIxArgs) -> ProgramResult {
    swap_invoke_with_program_id(crate::ID, accounts, args)
}
pub fn swap_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SwapAccounts<'_, '_>,
    args: SwapIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SwapKeys = accounts.into();
    let ix = swap_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn swap_invoke_signed(
    accounts: SwapAccounts<'_, '_>,
    args: SwapIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    swap_invoke_signed_with_program_id(crate::ID, accounts, args, seeds)
}
pub fn swap_verify_account_keys(
    accounts: SwapAccounts<'_, '_>,
    keys: SwapKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.token_program.key, keys.token_program),
        (*accounts.token_authority.key, keys.token_authority),
        (*accounts.whirlpool.key, keys.whirlpool),
        (*accounts.token_owner_account_a.key, keys.token_owner_account_a),
        (*accounts.token_vault_a.key, keys.token_vault_a),
        (*accounts.token_owner_account_b.key, keys.token_owner_account_b),
        (*accounts.token_vault_b.key, keys.token_vault_b),
        (*accounts.tick_array0.key, keys.tick_array0),
        (*accounts.tick_array1.key, keys.tick_array1),
        (*accounts.tick_array2.key, keys.tick_array2),
        (*accounts.oracle.key, keys.oracle),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn swap_verify_writable_privileges<'me, 'info>(
    accounts: SwapAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.whirlpool,
        accounts.token_owner_account_a,
        accounts.token_vault_a,
        accounts.token_owner_account_b,
        accounts.token_vault_b,
        accounts.tick_array0,
        accounts.tick_array1,
        accounts.tick_array2,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn swap_verify_signer_privileges<'me, 'info>(
    accounts: SwapAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.token_authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn swap_verify_account_privileges<'me, 'info>(
    accounts: SwapAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    swap_verify_writable_privileges(accounts)?;
    swap_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CLOSE_POSITION_IX_ACCOUNTS_LEN: usize = 6;
#[derive(Copy, Clone, Debug)]
pub struct ClosePositionAccounts<'me, 'info> {
    pub position_authority: &'me AccountInfo<'info>,
    pub receiver: &'me AccountInfo<'info>,
    pub position: &'me AccountInfo<'info>,
    pub position_mint: &'me AccountInfo<'info>,
    pub position_token_account: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ClosePositionKeys {
    pub position_authority: Pubkey,
    pub receiver: Pubkey,
    pub position: Pubkey,
    pub position_mint: Pubkey,
    pub position_token_account: Pubkey,
    pub token_program: Pubkey,
}
impl From<ClosePositionAccounts<'_, '_>> for ClosePositionKeys {
    fn from(accounts: ClosePositionAccounts) -> Self {
        Self {
            position_authority: *accounts.position_authority.key,
            receiver: *accounts.receiver.key,
            position: *accounts.position.key,
            position_mint: *accounts.position_mint.key,
            position_token_account: *accounts.position_token_account.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<ClosePositionKeys> for [AccountMeta; CLOSE_POSITION_IX_ACCOUNTS_LEN] {
    fn from(keys: ClosePositionKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.position_authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.receiver,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.position,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.position_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.position_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; CLOSE_POSITION_IX_ACCOUNTS_LEN]> for ClosePositionKeys {
    fn from(pubkeys: [Pubkey; CLOSE_POSITION_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            position_authority: pubkeys[0],
            receiver: pubkeys[1],
            position: pubkeys[2],
            position_mint: pubkeys[3],
            position_token_account: pubkeys[4],
            token_program: pubkeys[5],
        }
    }
}
impl<'info> From<ClosePositionAccounts<'_, 'info>>
for [AccountInfo<'info>; CLOSE_POSITION_IX_ACCOUNTS_LEN] {
    fn from(accounts: ClosePositionAccounts<'_, 'info>) -> Self {
        [
            accounts.position_authority.clone(),
            accounts.receiver.clone(),
            accounts.position.clone(),
            accounts.position_mint.clone(),
            accounts.position_token_account.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CLOSE_POSITION_IX_ACCOUNTS_LEN]>
for ClosePositionAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; CLOSE_POSITION_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            position_authority: &arr[0],
            receiver: &arr[1],
            position: &arr[2],
            position_mint: &arr[3],
            position_token_account: &arr[4],
            token_program: &arr[5],
        }
    }
}
pub const CLOSE_POSITION_IX_DISCM: [u8; 8] = [123, 134, 81, 0, 49, 68, 98, 98];
#[derive(Clone, Debug, PartialEq)]
pub struct ClosePositionIxData;
impl ClosePositionIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CLOSE_POSITION_IX_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        CLOSE_POSITION_IX_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CLOSE_POSITION_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn close_position_ix_with_program_id(
    program_id: Pubkey,
    keys: ClosePositionKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CLOSE_POSITION_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: ClosePositionIxData.try_to_vec()?,
    })
}
pub fn close_position_ix(keys: ClosePositionKeys) -> std::io::Result<Instruction> {
    close_position_ix_with_program_id(crate::ID, keys)
}
pub fn close_position_invoke_with_program_id(
    program_id: Pubkey,
    accounts: ClosePositionAccounts<'_, '_>,
) -> ProgramResult {
    let keys: ClosePositionKeys = accounts.into();
    let ix = close_position_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn close_position_invoke(accounts: ClosePositionAccounts<'_, '_>) -> ProgramResult {
    close_position_invoke_with_program_id(crate::ID, accounts)
}
pub fn close_position_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: ClosePositionAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: ClosePositionKeys = accounts.into();
    let ix = close_position_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn close_position_invoke_signed(
    accounts: ClosePositionAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    close_position_invoke_signed_with_program_id(crate::ID, accounts, seeds)
}
pub fn close_position_verify_account_keys(
    accounts: ClosePositionAccounts<'_, '_>,
    keys: ClosePositionKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.position_authority.key, keys.position_authority),
        (*accounts.receiver.key, keys.receiver),
        (*accounts.position.key, keys.position),
        (*accounts.position_mint.key, keys.position_mint),
        (*accounts.position_token_account.key, keys.position_token_account),
        (*accounts.token_program.key, keys.token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn close_position_verify_writable_privileges<'me, 'info>(
    accounts: ClosePositionAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.receiver,
        accounts.position,
        accounts.position_mint,
        accounts.position_token_account,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn close_position_verify_signer_privileges<'me, 'info>(
    accounts: ClosePositionAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.position_authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn close_position_verify_account_privileges<'me, 'info>(
    accounts: ClosePositionAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    close_position_verify_writable_privileges(accounts)?;
    close_position_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SET_DEFAULT_FEE_RATE_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct SetDefaultFeeRateAccounts<'me, 'info> {
    pub whirlpools_config: &'me AccountInfo<'info>,
    pub fee_tier: &'me AccountInfo<'info>,
    pub fee_authority: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SetDefaultFeeRateKeys {
    pub whirlpools_config: Pubkey,
    pub fee_tier: Pubkey,
    pub fee_authority: Pubkey,
}
impl From<SetDefaultFeeRateAccounts<'_, '_>> for SetDefaultFeeRateKeys {
    fn from(accounts: SetDefaultFeeRateAccounts) -> Self {
        Self {
            whirlpools_config: *accounts.whirlpools_config.key,
            fee_tier: *accounts.fee_tier.key,
            fee_authority: *accounts.fee_authority.key,
        }
    }
}
impl From<SetDefaultFeeRateKeys>
for [AccountMeta; SET_DEFAULT_FEE_RATE_IX_ACCOUNTS_LEN] {
    fn from(keys: SetDefaultFeeRateKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.whirlpools_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.fee_tier,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.fee_authority,
                is_signer: true,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; SET_DEFAULT_FEE_RATE_IX_ACCOUNTS_LEN]> for SetDefaultFeeRateKeys {
    fn from(pubkeys: [Pubkey; SET_DEFAULT_FEE_RATE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            whirlpools_config: pubkeys[0],
            fee_tier: pubkeys[1],
            fee_authority: pubkeys[2],
        }
    }
}
impl<'info> From<SetDefaultFeeRateAccounts<'_, 'info>>
for [AccountInfo<'info>; SET_DEFAULT_FEE_RATE_IX_ACCOUNTS_LEN] {
    fn from(accounts: SetDefaultFeeRateAccounts<'_, 'info>) -> Self {
        [
            accounts.whirlpools_config.clone(),
            accounts.fee_tier.clone(),
            accounts.fee_authority.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SET_DEFAULT_FEE_RATE_IX_ACCOUNTS_LEN]>
for SetDefaultFeeRateAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; SET_DEFAULT_FEE_RATE_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            whirlpools_config: &arr[0],
            fee_tier: &arr[1],
            fee_authority: &arr[2],
        }
    }
}
pub const SET_DEFAULT_FEE_RATE_IX_DISCM: [u8; 8] = [
    118,
    215,
    214,
    157,
    182,
    229,
    208,
    228,
];
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SetDefaultFeeRateIxArgs {
    pub default_fee_rate: u16,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SetDefaultFeeRateIxData(pub SetDefaultFeeRateIxArgs);
impl From<SetDefaultFeeRateIxArgs> for SetDefaultFeeRateIxData {
    fn from(args: SetDefaultFeeRateIxArgs) -> Self {
        Self(args)
    }
}
impl SetDefaultFeeRateIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SET_DEFAULT_FEE_RATE_IX_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        SET_DEFAULT_FEE_RATE_IX_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(SetDefaultFeeRateIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SET_DEFAULT_FEE_RATE_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn set_default_fee_rate_ix_with_program_id(
    program_id: Pubkey,
    keys: SetDefaultFeeRateKeys,
    args: SetDefaultFeeRateIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SET_DEFAULT_FEE_RATE_IX_ACCOUNTS_LEN] = keys.into();
    let data: SetDefaultFeeRateIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn set_default_fee_rate_ix(
    keys: SetDefaultFeeRateKeys,
    args: SetDefaultFeeRateIxArgs,
) -> std::io::Result<Instruction> {
    set_default_fee_rate_ix_with_program_id(crate::ID, keys, args)
}
pub fn set_default_fee_rate_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SetDefaultFeeRateAccounts<'_, '_>,
    args: SetDefaultFeeRateIxArgs,
) -> ProgramResult {
    let keys: SetDefaultFeeRateKeys = accounts.into();
    let ix = set_default_fee_rate_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn set_default_fee_rate_invoke(
    accounts: SetDefaultFeeRateAccounts<'_, '_>,
    args: SetDefaultFeeRateIxArgs,
) -> ProgramResult {
    set_default_fee_rate_invoke_with_program_id(crate::ID, accounts, args)
}
pub fn set_default_fee_rate_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SetDefaultFeeRateAccounts<'_, '_>,
    args: SetDefaultFeeRateIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SetDefaultFeeRateKeys = accounts.into();
    let ix = set_default_fee_rate_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn set_default_fee_rate_invoke_signed(
    accounts: SetDefaultFeeRateAccounts<'_, '_>,
    args: SetDefaultFeeRateIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    set_default_fee_rate_invoke_signed_with_program_id(crate::ID, accounts, args, seeds)
}
pub fn set_default_fee_rate_verify_account_keys(
    accounts: SetDefaultFeeRateAccounts<'_, '_>,
    keys: SetDefaultFeeRateKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.whirlpools_config.key, keys.whirlpools_config),
        (*accounts.fee_tier.key, keys.fee_tier),
        (*accounts.fee_authority.key, keys.fee_authority),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn set_default_fee_rate_verify_writable_privileges<'me, 'info>(
    accounts: SetDefaultFeeRateAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.fee_tier] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn set_default_fee_rate_verify_signer_privileges<'me, 'info>(
    accounts: SetDefaultFeeRateAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.fee_authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn set_default_fee_rate_verify_account_privileges<'me, 'info>(
    accounts: SetDefaultFeeRateAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    set_default_fee_rate_verify_writable_privileges(accounts)?;
    set_default_fee_rate_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SET_DEFAULT_PROTOCOL_FEE_RATE_IX_ACCOUNTS_LEN: usize = 2;
#[derive(Copy, Clone, Debug)]
pub struct SetDefaultProtocolFeeRateAccounts<'me, 'info> {
    pub whirlpools_config: &'me AccountInfo<'info>,
    pub fee_authority: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SetDefaultProtocolFeeRateKeys {
    pub whirlpools_config: Pubkey,
    pub fee_authority: Pubkey,
}
impl From<SetDefaultProtocolFeeRateAccounts<'_, '_>> for SetDefaultProtocolFeeRateKeys {
    fn from(accounts: SetDefaultProtocolFeeRateAccounts) -> Self {
        Self {
            whirlpools_config: *accounts.whirlpools_config.key,
            fee_authority: *accounts.fee_authority.key,
        }
    }
}
impl From<SetDefaultProtocolFeeRateKeys>
for [AccountMeta; SET_DEFAULT_PROTOCOL_FEE_RATE_IX_ACCOUNTS_LEN] {
    fn from(keys: SetDefaultProtocolFeeRateKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.whirlpools_config,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.fee_authority,
                is_signer: true,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; SET_DEFAULT_PROTOCOL_FEE_RATE_IX_ACCOUNTS_LEN]>
for SetDefaultProtocolFeeRateKeys {
    fn from(pubkeys: [Pubkey; SET_DEFAULT_PROTOCOL_FEE_RATE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            whirlpools_config: pubkeys[0],
            fee_authority: pubkeys[1],
        }
    }
}
impl<'info> From<SetDefaultProtocolFeeRateAccounts<'_, 'info>>
for [AccountInfo<'info>; SET_DEFAULT_PROTOCOL_FEE_RATE_IX_ACCOUNTS_LEN] {
    fn from(accounts: SetDefaultProtocolFeeRateAccounts<'_, 'info>) -> Self {
        [accounts.whirlpools_config.clone(), accounts.fee_authority.clone()]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; SET_DEFAULT_PROTOCOL_FEE_RATE_IX_ACCOUNTS_LEN]>
for SetDefaultProtocolFeeRateAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; SET_DEFAULT_PROTOCOL_FEE_RATE_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            whirlpools_config: &arr[0],
            fee_authority: &arr[1],
        }
    }
}
pub const SET_DEFAULT_PROTOCOL_FEE_RATE_IX_DISCM: [u8; 8] = [
    107,
    205,
    249,
    226,
    151,
    35,
    86,
    0,
];
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SetDefaultProtocolFeeRateIxArgs {
    pub default_protocol_fee_rate: u16,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SetDefaultProtocolFeeRateIxData(pub SetDefaultProtocolFeeRateIxArgs);
impl From<SetDefaultProtocolFeeRateIxArgs> for SetDefaultProtocolFeeRateIxData {
    fn from(args: SetDefaultProtocolFeeRateIxArgs) -> Self {
        Self(args)
    }
}
impl SetDefaultProtocolFeeRateIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SET_DEFAULT_PROTOCOL_FEE_RATE_IX_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        SET_DEFAULT_PROTOCOL_FEE_RATE_IX_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(SetDefaultProtocolFeeRateIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SET_DEFAULT_PROTOCOL_FEE_RATE_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn set_default_protocol_fee_rate_ix_with_program_id(
    program_id: Pubkey,
    keys: SetDefaultProtocolFeeRateKeys,
    args: SetDefaultProtocolFeeRateIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SET_DEFAULT_PROTOCOL_FEE_RATE_IX_ACCOUNTS_LEN] = keys
        .into();
    let data: SetDefaultProtocolFeeRateIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn set_default_protocol_fee_rate_ix(
    keys: SetDefaultProtocolFeeRateKeys,
    args: SetDefaultProtocolFeeRateIxArgs,
) -> std::io::Result<Instruction> {
    set_default_protocol_fee_rate_ix_with_program_id(crate::ID, keys, args)
}
pub fn set_default_protocol_fee_rate_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SetDefaultProtocolFeeRateAccounts<'_, '_>,
    args: SetDefaultProtocolFeeRateIxArgs,
) -> ProgramResult {
    let keys: SetDefaultProtocolFeeRateKeys = accounts.into();
    let ix = set_default_protocol_fee_rate_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn set_default_protocol_fee_rate_invoke(
    accounts: SetDefaultProtocolFeeRateAccounts<'_, '_>,
    args: SetDefaultProtocolFeeRateIxArgs,
) -> ProgramResult {
    set_default_protocol_fee_rate_invoke_with_program_id(crate::ID, accounts, args)
}
pub fn set_default_protocol_fee_rate_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SetDefaultProtocolFeeRateAccounts<'_, '_>,
    args: SetDefaultProtocolFeeRateIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SetDefaultProtocolFeeRateKeys = accounts.into();
    let ix = set_default_protocol_fee_rate_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn set_default_protocol_fee_rate_invoke_signed(
    accounts: SetDefaultProtocolFeeRateAccounts<'_, '_>,
    args: SetDefaultProtocolFeeRateIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    set_default_protocol_fee_rate_invoke_signed_with_program_id(
        crate::ID,
        accounts,
        args,
        seeds,
    )
}
pub fn set_default_protocol_fee_rate_verify_account_keys(
    accounts: SetDefaultProtocolFeeRateAccounts<'_, '_>,
    keys: SetDefaultProtocolFeeRateKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.whirlpools_config.key, keys.whirlpools_config),
        (*accounts.fee_authority.key, keys.fee_authority),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn set_default_protocol_fee_rate_verify_writable_privileges<'me, 'info>(
    accounts: SetDefaultProtocolFeeRateAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.whirlpools_config] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn set_default_protocol_fee_rate_verify_signer_privileges<'me, 'info>(
    accounts: SetDefaultProtocolFeeRateAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.fee_authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn set_default_protocol_fee_rate_verify_account_privileges<'me, 'info>(
    accounts: SetDefaultProtocolFeeRateAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    set_default_protocol_fee_rate_verify_writable_privileges(accounts)?;
    set_default_protocol_fee_rate_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SET_FEE_RATE_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct SetFeeRateAccounts<'me, 'info> {
    pub whirlpools_config: &'me AccountInfo<'info>,
    pub whirlpool: &'me AccountInfo<'info>,
    pub fee_authority: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SetFeeRateKeys {
    pub whirlpools_config: Pubkey,
    pub whirlpool: Pubkey,
    pub fee_authority: Pubkey,
}
impl From<SetFeeRateAccounts<'_, '_>> for SetFeeRateKeys {
    fn from(accounts: SetFeeRateAccounts) -> Self {
        Self {
            whirlpools_config: *accounts.whirlpools_config.key,
            whirlpool: *accounts.whirlpool.key,
            fee_authority: *accounts.fee_authority.key,
        }
    }
}
impl From<SetFeeRateKeys> for [AccountMeta; SET_FEE_RATE_IX_ACCOUNTS_LEN] {
    fn from(keys: SetFeeRateKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.whirlpools_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.whirlpool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.fee_authority,
                is_signer: true,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; SET_FEE_RATE_IX_ACCOUNTS_LEN]> for SetFeeRateKeys {
    fn from(pubkeys: [Pubkey; SET_FEE_RATE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            whirlpools_config: pubkeys[0],
            whirlpool: pubkeys[1],
            fee_authority: pubkeys[2],
        }
    }
}
impl<'info> From<SetFeeRateAccounts<'_, 'info>>
for [AccountInfo<'info>; SET_FEE_RATE_IX_ACCOUNTS_LEN] {
    fn from(accounts: SetFeeRateAccounts<'_, 'info>) -> Self {
        [
            accounts.whirlpools_config.clone(),
            accounts.whirlpool.clone(),
            accounts.fee_authority.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SET_FEE_RATE_IX_ACCOUNTS_LEN]>
for SetFeeRateAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; SET_FEE_RATE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            whirlpools_config: &arr[0],
            whirlpool: &arr[1],
            fee_authority: &arr[2],
        }
    }
}
pub const SET_FEE_RATE_IX_DISCM: [u8; 8] = [53, 243, 137, 65, 8, 140, 158, 6];
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SetFeeRateIxArgs {
    pub fee_rate: u16,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SetFeeRateIxData(pub SetFeeRateIxArgs);
impl From<SetFeeRateIxArgs> for SetFeeRateIxData {
    fn from(args: SetFeeRateIxArgs) -> Self {
        Self(args)
    }
}
impl SetFeeRateIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SET_FEE_RATE_IX_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        SET_FEE_RATE_IX_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(SetFeeRateIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SET_FEE_RATE_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn set_fee_rate_ix_with_program_id(
    program_id: Pubkey,
    keys: SetFeeRateKeys,
    args: SetFeeRateIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SET_FEE_RATE_IX_ACCOUNTS_LEN] = keys.into();
    let data: SetFeeRateIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn set_fee_rate_ix(
    keys: SetFeeRateKeys,
    args: SetFeeRateIxArgs,
) -> std::io::Result<Instruction> {
    set_fee_rate_ix_with_program_id(crate::ID, keys, args)
}
pub fn set_fee_rate_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SetFeeRateAccounts<'_, '_>,
    args: SetFeeRateIxArgs,
) -> ProgramResult {
    let keys: SetFeeRateKeys = accounts.into();
    let ix = set_fee_rate_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn set_fee_rate_invoke(
    accounts: SetFeeRateAccounts<'_, '_>,
    args: SetFeeRateIxArgs,
) -> ProgramResult {
    set_fee_rate_invoke_with_program_id(crate::ID, accounts, args)
}
pub fn set_fee_rate_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SetFeeRateAccounts<'_, '_>,
    args: SetFeeRateIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SetFeeRateKeys = accounts.into();
    let ix = set_fee_rate_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn set_fee_rate_invoke_signed(
    accounts: SetFeeRateAccounts<'_, '_>,
    args: SetFeeRateIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    set_fee_rate_invoke_signed_with_program_id(crate::ID, accounts, args, seeds)
}
pub fn set_fee_rate_verify_account_keys(
    accounts: SetFeeRateAccounts<'_, '_>,
    keys: SetFeeRateKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.whirlpools_config.key, keys.whirlpools_config),
        (*accounts.whirlpool.key, keys.whirlpool),
        (*accounts.fee_authority.key, keys.fee_authority),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn set_fee_rate_verify_writable_privileges<'me, 'info>(
    accounts: SetFeeRateAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.whirlpool] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn set_fee_rate_verify_signer_privileges<'me, 'info>(
    accounts: SetFeeRateAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.fee_authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn set_fee_rate_verify_account_privileges<'me, 'info>(
    accounts: SetFeeRateAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    set_fee_rate_verify_writable_privileges(accounts)?;
    set_fee_rate_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SET_PROTOCOL_FEE_RATE_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct SetProtocolFeeRateAccounts<'me, 'info> {
    pub whirlpools_config: &'me AccountInfo<'info>,
    pub whirlpool: &'me AccountInfo<'info>,
    pub fee_authority: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SetProtocolFeeRateKeys {
    pub whirlpools_config: Pubkey,
    pub whirlpool: Pubkey,
    pub fee_authority: Pubkey,
}
impl From<SetProtocolFeeRateAccounts<'_, '_>> for SetProtocolFeeRateKeys {
    fn from(accounts: SetProtocolFeeRateAccounts) -> Self {
        Self {
            whirlpools_config: *accounts.whirlpools_config.key,
            whirlpool: *accounts.whirlpool.key,
            fee_authority: *accounts.fee_authority.key,
        }
    }
}
impl From<SetProtocolFeeRateKeys>
for [AccountMeta; SET_PROTOCOL_FEE_RATE_IX_ACCOUNTS_LEN] {
    fn from(keys: SetProtocolFeeRateKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.whirlpools_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.whirlpool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.fee_authority,
                is_signer: true,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; SET_PROTOCOL_FEE_RATE_IX_ACCOUNTS_LEN]> for SetProtocolFeeRateKeys {
    fn from(pubkeys: [Pubkey; SET_PROTOCOL_FEE_RATE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            whirlpools_config: pubkeys[0],
            whirlpool: pubkeys[1],
            fee_authority: pubkeys[2],
        }
    }
}
impl<'info> From<SetProtocolFeeRateAccounts<'_, 'info>>
for [AccountInfo<'info>; SET_PROTOCOL_FEE_RATE_IX_ACCOUNTS_LEN] {
    fn from(accounts: SetProtocolFeeRateAccounts<'_, 'info>) -> Self {
        [
            accounts.whirlpools_config.clone(),
            accounts.whirlpool.clone(),
            accounts.fee_authority.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SET_PROTOCOL_FEE_RATE_IX_ACCOUNTS_LEN]>
for SetProtocolFeeRateAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; SET_PROTOCOL_FEE_RATE_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            whirlpools_config: &arr[0],
            whirlpool: &arr[1],
            fee_authority: &arr[2],
        }
    }
}
pub const SET_PROTOCOL_FEE_RATE_IX_DISCM: [u8; 8] = [95, 7, 4, 50, 154, 79, 156, 131];
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SetProtocolFeeRateIxArgs {
    pub protocol_fee_rate: u16,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SetProtocolFeeRateIxData(pub SetProtocolFeeRateIxArgs);
impl From<SetProtocolFeeRateIxArgs> for SetProtocolFeeRateIxData {
    fn from(args: SetProtocolFeeRateIxArgs) -> Self {
        Self(args)
    }
}
impl SetProtocolFeeRateIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SET_PROTOCOL_FEE_RATE_IX_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        SET_PROTOCOL_FEE_RATE_IX_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(SetProtocolFeeRateIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SET_PROTOCOL_FEE_RATE_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn set_protocol_fee_rate_ix_with_program_id(
    program_id: Pubkey,
    keys: SetProtocolFeeRateKeys,
    args: SetProtocolFeeRateIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SET_PROTOCOL_FEE_RATE_IX_ACCOUNTS_LEN] = keys.into();
    let data: SetProtocolFeeRateIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn set_protocol_fee_rate_ix(
    keys: SetProtocolFeeRateKeys,
    args: SetProtocolFeeRateIxArgs,
) -> std::io::Result<Instruction> {
    set_protocol_fee_rate_ix_with_program_id(crate::ID, keys, args)
}
pub fn set_protocol_fee_rate_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SetProtocolFeeRateAccounts<'_, '_>,
    args: SetProtocolFeeRateIxArgs,
) -> ProgramResult {
    let keys: SetProtocolFeeRateKeys = accounts.into();
    let ix = set_protocol_fee_rate_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn set_protocol_fee_rate_invoke(
    accounts: SetProtocolFeeRateAccounts<'_, '_>,
    args: SetProtocolFeeRateIxArgs,
) -> ProgramResult {
    set_protocol_fee_rate_invoke_with_program_id(crate::ID, accounts, args)
}
pub fn set_protocol_fee_rate_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SetProtocolFeeRateAccounts<'_, '_>,
    args: SetProtocolFeeRateIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SetProtocolFeeRateKeys = accounts.into();
    let ix = set_protocol_fee_rate_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn set_protocol_fee_rate_invoke_signed(
    accounts: SetProtocolFeeRateAccounts<'_, '_>,
    args: SetProtocolFeeRateIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    set_protocol_fee_rate_invoke_signed_with_program_id(crate::ID, accounts, args, seeds)
}
pub fn set_protocol_fee_rate_verify_account_keys(
    accounts: SetProtocolFeeRateAccounts<'_, '_>,
    keys: SetProtocolFeeRateKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.whirlpools_config.key, keys.whirlpools_config),
        (*accounts.whirlpool.key, keys.whirlpool),
        (*accounts.fee_authority.key, keys.fee_authority),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn set_protocol_fee_rate_verify_writable_privileges<'me, 'info>(
    accounts: SetProtocolFeeRateAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.whirlpool] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn set_protocol_fee_rate_verify_signer_privileges<'me, 'info>(
    accounts: SetProtocolFeeRateAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.fee_authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn set_protocol_fee_rate_verify_account_privileges<'me, 'info>(
    accounts: SetProtocolFeeRateAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    set_protocol_fee_rate_verify_writable_privileges(accounts)?;
    set_protocol_fee_rate_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SET_FEE_AUTHORITY_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct SetFeeAuthorityAccounts<'me, 'info> {
    pub whirlpools_config: &'me AccountInfo<'info>,
    pub fee_authority: &'me AccountInfo<'info>,
    pub new_fee_authority: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SetFeeAuthorityKeys {
    pub whirlpools_config: Pubkey,
    pub fee_authority: Pubkey,
    pub new_fee_authority: Pubkey,
}
impl From<SetFeeAuthorityAccounts<'_, '_>> for SetFeeAuthorityKeys {
    fn from(accounts: SetFeeAuthorityAccounts) -> Self {
        Self {
            whirlpools_config: *accounts.whirlpools_config.key,
            fee_authority: *accounts.fee_authority.key,
            new_fee_authority: *accounts.new_fee_authority.key,
        }
    }
}
impl From<SetFeeAuthorityKeys> for [AccountMeta; SET_FEE_AUTHORITY_IX_ACCOUNTS_LEN] {
    fn from(keys: SetFeeAuthorityKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.whirlpools_config,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.fee_authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.new_fee_authority,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; SET_FEE_AUTHORITY_IX_ACCOUNTS_LEN]> for SetFeeAuthorityKeys {
    fn from(pubkeys: [Pubkey; SET_FEE_AUTHORITY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            whirlpools_config: pubkeys[0],
            fee_authority: pubkeys[1],
            new_fee_authority: pubkeys[2],
        }
    }
}
impl<'info> From<SetFeeAuthorityAccounts<'_, 'info>>
for [AccountInfo<'info>; SET_FEE_AUTHORITY_IX_ACCOUNTS_LEN] {
    fn from(accounts: SetFeeAuthorityAccounts<'_, 'info>) -> Self {
        [
            accounts.whirlpools_config.clone(),
            accounts.fee_authority.clone(),
            accounts.new_fee_authority.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SET_FEE_AUTHORITY_IX_ACCOUNTS_LEN]>
for SetFeeAuthorityAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; SET_FEE_AUTHORITY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            whirlpools_config: &arr[0],
            fee_authority: &arr[1],
            new_fee_authority: &arr[2],
        }
    }
}
pub const SET_FEE_AUTHORITY_IX_DISCM: [u8; 8] = [31, 1, 50, 87, 237, 101, 97, 132];
#[derive(Clone, Debug, PartialEq)]
pub struct SetFeeAuthorityIxData;
impl SetFeeAuthorityIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SET_FEE_AUTHORITY_IX_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        SET_FEE_AUTHORITY_IX_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SET_FEE_AUTHORITY_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn set_fee_authority_ix_with_program_id(
    program_id: Pubkey,
    keys: SetFeeAuthorityKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SET_FEE_AUTHORITY_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: SetFeeAuthorityIxData.try_to_vec()?,
    })
}
pub fn set_fee_authority_ix(keys: SetFeeAuthorityKeys) -> std::io::Result<Instruction> {
    set_fee_authority_ix_with_program_id(crate::ID, keys)
}
pub fn set_fee_authority_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SetFeeAuthorityAccounts<'_, '_>,
) -> ProgramResult {
    let keys: SetFeeAuthorityKeys = accounts.into();
    let ix = set_fee_authority_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn set_fee_authority_invoke(
    accounts: SetFeeAuthorityAccounts<'_, '_>,
) -> ProgramResult {
    set_fee_authority_invoke_with_program_id(crate::ID, accounts)
}
pub fn set_fee_authority_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SetFeeAuthorityAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SetFeeAuthorityKeys = accounts.into();
    let ix = set_fee_authority_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn set_fee_authority_invoke_signed(
    accounts: SetFeeAuthorityAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    set_fee_authority_invoke_signed_with_program_id(crate::ID, accounts, seeds)
}
pub fn set_fee_authority_verify_account_keys(
    accounts: SetFeeAuthorityAccounts<'_, '_>,
    keys: SetFeeAuthorityKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.whirlpools_config.key, keys.whirlpools_config),
        (*accounts.fee_authority.key, keys.fee_authority),
        (*accounts.new_fee_authority.key, keys.new_fee_authority),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn set_fee_authority_verify_writable_privileges<'me, 'info>(
    accounts: SetFeeAuthorityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.whirlpools_config] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn set_fee_authority_verify_signer_privileges<'me, 'info>(
    accounts: SetFeeAuthorityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.fee_authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn set_fee_authority_verify_account_privileges<'me, 'info>(
    accounts: SetFeeAuthorityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    set_fee_authority_verify_writable_privileges(accounts)?;
    set_fee_authority_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SET_COLLECT_PROTOCOL_FEES_AUTHORITY_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct SetCollectProtocolFeesAuthorityAccounts<'me, 'info> {
    pub whirlpools_config: &'me AccountInfo<'info>,
    pub collect_protocol_fees_authority: &'me AccountInfo<'info>,
    pub new_collect_protocol_fees_authority: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SetCollectProtocolFeesAuthorityKeys {
    pub whirlpools_config: Pubkey,
    pub collect_protocol_fees_authority: Pubkey,
    pub new_collect_protocol_fees_authority: Pubkey,
}
impl From<SetCollectProtocolFeesAuthorityAccounts<'_, '_>>
for SetCollectProtocolFeesAuthorityKeys {
    fn from(accounts: SetCollectProtocolFeesAuthorityAccounts) -> Self {
        Self {
            whirlpools_config: *accounts.whirlpools_config.key,
            collect_protocol_fees_authority: *accounts
                .collect_protocol_fees_authority
                .key,
            new_collect_protocol_fees_authority: *accounts
                .new_collect_protocol_fees_authority
                .key,
        }
    }
}
impl From<SetCollectProtocolFeesAuthorityKeys>
for [AccountMeta; SET_COLLECT_PROTOCOL_FEES_AUTHORITY_IX_ACCOUNTS_LEN] {
    fn from(keys: SetCollectProtocolFeesAuthorityKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.whirlpools_config,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.collect_protocol_fees_authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.new_collect_protocol_fees_authority,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; SET_COLLECT_PROTOCOL_FEES_AUTHORITY_IX_ACCOUNTS_LEN]>
for SetCollectProtocolFeesAuthorityKeys {
    fn from(
        pubkeys: [Pubkey; SET_COLLECT_PROTOCOL_FEES_AUTHORITY_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            whirlpools_config: pubkeys[0],
            collect_protocol_fees_authority: pubkeys[1],
            new_collect_protocol_fees_authority: pubkeys[2],
        }
    }
}
impl<'info> From<SetCollectProtocolFeesAuthorityAccounts<'_, 'info>>
for [AccountInfo<'info>; SET_COLLECT_PROTOCOL_FEES_AUTHORITY_IX_ACCOUNTS_LEN] {
    fn from(accounts: SetCollectProtocolFeesAuthorityAccounts<'_, 'info>) -> Self {
        [
            accounts.whirlpools_config.clone(),
            accounts.collect_protocol_fees_authority.clone(),
            accounts.new_collect_protocol_fees_authority.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; SET_COLLECT_PROTOCOL_FEES_AUTHORITY_IX_ACCOUNTS_LEN]>
for SetCollectProtocolFeesAuthorityAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<
            'info,
        >; SET_COLLECT_PROTOCOL_FEES_AUTHORITY_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            whirlpools_config: &arr[0],
            collect_protocol_fees_authority: &arr[1],
            new_collect_protocol_fees_authority: &arr[2],
        }
    }
}
pub const SET_COLLECT_PROTOCOL_FEES_AUTHORITY_IX_DISCM: [u8; 8] = [
    34,
    150,
    93,
    244,
    139,
    225,
    233,
    67,
];
#[derive(Clone, Debug, PartialEq)]
pub struct SetCollectProtocolFeesAuthorityIxData;
impl SetCollectProtocolFeesAuthorityIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SET_COLLECT_PROTOCOL_FEES_AUTHORITY_IX_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        SET_COLLECT_PROTOCOL_FEES_AUTHORITY_IX_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SET_COLLECT_PROTOCOL_FEES_AUTHORITY_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn set_collect_protocol_fees_authority_ix_with_program_id(
    program_id: Pubkey,
    keys: SetCollectProtocolFeesAuthorityKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SET_COLLECT_PROTOCOL_FEES_AUTHORITY_IX_ACCOUNTS_LEN] = keys
        .into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: SetCollectProtocolFeesAuthorityIxData.try_to_vec()?,
    })
}
pub fn set_collect_protocol_fees_authority_ix(
    keys: SetCollectProtocolFeesAuthorityKeys,
) -> std::io::Result<Instruction> {
    set_collect_protocol_fees_authority_ix_with_program_id(crate::ID, keys)
}
pub fn set_collect_protocol_fees_authority_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SetCollectProtocolFeesAuthorityAccounts<'_, '_>,
) -> ProgramResult {
    let keys: SetCollectProtocolFeesAuthorityKeys = accounts.into();
    let ix = set_collect_protocol_fees_authority_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn set_collect_protocol_fees_authority_invoke(
    accounts: SetCollectProtocolFeesAuthorityAccounts<'_, '_>,
) -> ProgramResult {
    set_collect_protocol_fees_authority_invoke_with_program_id(crate::ID, accounts)
}
pub fn set_collect_protocol_fees_authority_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SetCollectProtocolFeesAuthorityAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SetCollectProtocolFeesAuthorityKeys = accounts.into();
    let ix = set_collect_protocol_fees_authority_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn set_collect_protocol_fees_authority_invoke_signed(
    accounts: SetCollectProtocolFeesAuthorityAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    set_collect_protocol_fees_authority_invoke_signed_with_program_id(
        crate::ID,
        accounts,
        seeds,
    )
}
pub fn set_collect_protocol_fees_authority_verify_account_keys(
    accounts: SetCollectProtocolFeesAuthorityAccounts<'_, '_>,
    keys: SetCollectProtocolFeesAuthorityKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.whirlpools_config.key, keys.whirlpools_config),
        (
            *accounts.collect_protocol_fees_authority.key,
            keys.collect_protocol_fees_authority,
        ),
        (
            *accounts.new_collect_protocol_fees_authority.key,
            keys.new_collect_protocol_fees_authority,
        ),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn set_collect_protocol_fees_authority_verify_writable_privileges<'me, 'info>(
    accounts: SetCollectProtocolFeesAuthorityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.whirlpools_config] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn set_collect_protocol_fees_authority_verify_signer_privileges<'me, 'info>(
    accounts: SetCollectProtocolFeesAuthorityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.collect_protocol_fees_authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn set_collect_protocol_fees_authority_verify_account_privileges<'me, 'info>(
    accounts: SetCollectProtocolFeesAuthorityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    set_collect_protocol_fees_authority_verify_writable_privileges(accounts)?;
    set_collect_protocol_fees_authority_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SET_REWARD_AUTHORITY_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct SetRewardAuthorityAccounts<'me, 'info> {
    pub whirlpool: &'me AccountInfo<'info>,
    pub reward_authority: &'me AccountInfo<'info>,
    pub new_reward_authority: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SetRewardAuthorityKeys {
    pub whirlpool: Pubkey,
    pub reward_authority: Pubkey,
    pub new_reward_authority: Pubkey,
}
impl From<SetRewardAuthorityAccounts<'_, '_>> for SetRewardAuthorityKeys {
    fn from(accounts: SetRewardAuthorityAccounts) -> Self {
        Self {
            whirlpool: *accounts.whirlpool.key,
            reward_authority: *accounts.reward_authority.key,
            new_reward_authority: *accounts.new_reward_authority.key,
        }
    }
}
impl From<SetRewardAuthorityKeys>
for [AccountMeta; SET_REWARD_AUTHORITY_IX_ACCOUNTS_LEN] {
    fn from(keys: SetRewardAuthorityKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.whirlpool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.reward_authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.new_reward_authority,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; SET_REWARD_AUTHORITY_IX_ACCOUNTS_LEN]> for SetRewardAuthorityKeys {
    fn from(pubkeys: [Pubkey; SET_REWARD_AUTHORITY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            whirlpool: pubkeys[0],
            reward_authority: pubkeys[1],
            new_reward_authority: pubkeys[2],
        }
    }
}
impl<'info> From<SetRewardAuthorityAccounts<'_, 'info>>
for [AccountInfo<'info>; SET_REWARD_AUTHORITY_IX_ACCOUNTS_LEN] {
    fn from(accounts: SetRewardAuthorityAccounts<'_, 'info>) -> Self {
        [
            accounts.whirlpool.clone(),
            accounts.reward_authority.clone(),
            accounts.new_reward_authority.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SET_REWARD_AUTHORITY_IX_ACCOUNTS_LEN]>
for SetRewardAuthorityAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; SET_REWARD_AUTHORITY_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            whirlpool: &arr[0],
            reward_authority: &arr[1],
            new_reward_authority: &arr[2],
        }
    }
}
pub const SET_REWARD_AUTHORITY_IX_DISCM: [u8; 8] = [34, 39, 183, 252, 83, 28, 85, 127];
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SetRewardAuthorityIxArgs {
    pub reward_index: u8,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SetRewardAuthorityIxData(pub SetRewardAuthorityIxArgs);
impl From<SetRewardAuthorityIxArgs> for SetRewardAuthorityIxData {
    fn from(args: SetRewardAuthorityIxArgs) -> Self {
        Self(args)
    }
}
impl SetRewardAuthorityIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SET_REWARD_AUTHORITY_IX_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        SET_REWARD_AUTHORITY_IX_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(SetRewardAuthorityIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SET_REWARD_AUTHORITY_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn set_reward_authority_ix_with_program_id(
    program_id: Pubkey,
    keys: SetRewardAuthorityKeys,
    args: SetRewardAuthorityIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SET_REWARD_AUTHORITY_IX_ACCOUNTS_LEN] = keys.into();
    let data: SetRewardAuthorityIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn set_reward_authority_ix(
    keys: SetRewardAuthorityKeys,
    args: SetRewardAuthorityIxArgs,
) -> std::io::Result<Instruction> {
    set_reward_authority_ix_with_program_id(crate::ID, keys, args)
}
pub fn set_reward_authority_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SetRewardAuthorityAccounts<'_, '_>,
    args: SetRewardAuthorityIxArgs,
) -> ProgramResult {
    let keys: SetRewardAuthorityKeys = accounts.into();
    let ix = set_reward_authority_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn set_reward_authority_invoke(
    accounts: SetRewardAuthorityAccounts<'_, '_>,
    args: SetRewardAuthorityIxArgs,
) -> ProgramResult {
    set_reward_authority_invoke_with_program_id(crate::ID, accounts, args)
}
pub fn set_reward_authority_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SetRewardAuthorityAccounts<'_, '_>,
    args: SetRewardAuthorityIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SetRewardAuthorityKeys = accounts.into();
    let ix = set_reward_authority_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn set_reward_authority_invoke_signed(
    accounts: SetRewardAuthorityAccounts<'_, '_>,
    args: SetRewardAuthorityIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    set_reward_authority_invoke_signed_with_program_id(crate::ID, accounts, args, seeds)
}
pub fn set_reward_authority_verify_account_keys(
    accounts: SetRewardAuthorityAccounts<'_, '_>,
    keys: SetRewardAuthorityKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.whirlpool.key, keys.whirlpool),
        (*accounts.reward_authority.key, keys.reward_authority),
        (*accounts.new_reward_authority.key, keys.new_reward_authority),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn set_reward_authority_verify_writable_privileges<'me, 'info>(
    accounts: SetRewardAuthorityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.whirlpool] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn set_reward_authority_verify_signer_privileges<'me, 'info>(
    accounts: SetRewardAuthorityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.reward_authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn set_reward_authority_verify_account_privileges<'me, 'info>(
    accounts: SetRewardAuthorityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    set_reward_authority_verify_writable_privileges(accounts)?;
    set_reward_authority_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SET_REWARD_AUTHORITY_BY_SUPER_AUTHORITY_IX_ACCOUNTS_LEN: usize = 4;
#[derive(Copy, Clone, Debug)]
pub struct SetRewardAuthorityBySuperAuthorityAccounts<'me, 'info> {
    pub whirlpools_config: &'me AccountInfo<'info>,
    pub whirlpool: &'me AccountInfo<'info>,
    pub reward_emissions_super_authority: &'me AccountInfo<'info>,
    pub new_reward_authority: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SetRewardAuthorityBySuperAuthorityKeys {
    pub whirlpools_config: Pubkey,
    pub whirlpool: Pubkey,
    pub reward_emissions_super_authority: Pubkey,
    pub new_reward_authority: Pubkey,
}
impl From<SetRewardAuthorityBySuperAuthorityAccounts<'_, '_>>
for SetRewardAuthorityBySuperAuthorityKeys {
    fn from(accounts: SetRewardAuthorityBySuperAuthorityAccounts) -> Self {
        Self {
            whirlpools_config: *accounts.whirlpools_config.key,
            whirlpool: *accounts.whirlpool.key,
            reward_emissions_super_authority: *accounts
                .reward_emissions_super_authority
                .key,
            new_reward_authority: *accounts.new_reward_authority.key,
        }
    }
}
impl From<SetRewardAuthorityBySuperAuthorityKeys>
for [AccountMeta; SET_REWARD_AUTHORITY_BY_SUPER_AUTHORITY_IX_ACCOUNTS_LEN] {
    fn from(keys: SetRewardAuthorityBySuperAuthorityKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.whirlpools_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.whirlpool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.reward_emissions_super_authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.new_reward_authority,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; SET_REWARD_AUTHORITY_BY_SUPER_AUTHORITY_IX_ACCOUNTS_LEN]>
for SetRewardAuthorityBySuperAuthorityKeys {
    fn from(
        pubkeys: [Pubkey; SET_REWARD_AUTHORITY_BY_SUPER_AUTHORITY_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            whirlpools_config: pubkeys[0],
            whirlpool: pubkeys[1],
            reward_emissions_super_authority: pubkeys[2],
            new_reward_authority: pubkeys[3],
        }
    }
}
impl<'info> From<SetRewardAuthorityBySuperAuthorityAccounts<'_, 'info>>
for [AccountInfo<'info>; SET_REWARD_AUTHORITY_BY_SUPER_AUTHORITY_IX_ACCOUNTS_LEN] {
    fn from(accounts: SetRewardAuthorityBySuperAuthorityAccounts<'_, 'info>) -> Self {
        [
            accounts.whirlpools_config.clone(),
            accounts.whirlpool.clone(),
            accounts.reward_emissions_super_authority.clone(),
            accounts.new_reward_authority.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<
    &'me [AccountInfo<'info>; SET_REWARD_AUTHORITY_BY_SUPER_AUTHORITY_IX_ACCOUNTS_LEN],
> for SetRewardAuthorityBySuperAuthorityAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<
            'info,
        >; SET_REWARD_AUTHORITY_BY_SUPER_AUTHORITY_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            whirlpools_config: &arr[0],
            whirlpool: &arr[1],
            reward_emissions_super_authority: &arr[2],
            new_reward_authority: &arr[3],
        }
    }
}
pub const SET_REWARD_AUTHORITY_BY_SUPER_AUTHORITY_IX_DISCM: [u8; 8] = [
    240,
    154,
    201,
    198,
    148,
    93,
    56,
    25,
];
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SetRewardAuthorityBySuperAuthorityIxArgs {
    pub reward_index: u8,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SetRewardAuthorityBySuperAuthorityIxData(
    pub SetRewardAuthorityBySuperAuthorityIxArgs,
);
impl From<SetRewardAuthorityBySuperAuthorityIxArgs>
for SetRewardAuthorityBySuperAuthorityIxData {
    fn from(args: SetRewardAuthorityBySuperAuthorityIxArgs) -> Self {
        Self(args)
    }
}
impl SetRewardAuthorityBySuperAuthorityIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SET_REWARD_AUTHORITY_BY_SUPER_AUTHORITY_IX_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        SET_REWARD_AUTHORITY_BY_SUPER_AUTHORITY_IX_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(SetRewardAuthorityBySuperAuthorityIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SET_REWARD_AUTHORITY_BY_SUPER_AUTHORITY_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn set_reward_authority_by_super_authority_ix_with_program_id(
    program_id: Pubkey,
    keys: SetRewardAuthorityBySuperAuthorityKeys,
    args: SetRewardAuthorityBySuperAuthorityIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SET_REWARD_AUTHORITY_BY_SUPER_AUTHORITY_IX_ACCOUNTS_LEN] = keys
        .into();
    let data: SetRewardAuthorityBySuperAuthorityIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn set_reward_authority_by_super_authority_ix(
    keys: SetRewardAuthorityBySuperAuthorityKeys,
    args: SetRewardAuthorityBySuperAuthorityIxArgs,
) -> std::io::Result<Instruction> {
    set_reward_authority_by_super_authority_ix_with_program_id(crate::ID, keys, args)
}
pub fn set_reward_authority_by_super_authority_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SetRewardAuthorityBySuperAuthorityAccounts<'_, '_>,
    args: SetRewardAuthorityBySuperAuthorityIxArgs,
) -> ProgramResult {
    let keys: SetRewardAuthorityBySuperAuthorityKeys = accounts.into();
    let ix = set_reward_authority_by_super_authority_ix_with_program_id(
        program_id,
        keys,
        args,
    )?;
    invoke_instruction(&ix, accounts)
}
pub fn set_reward_authority_by_super_authority_invoke(
    accounts: SetRewardAuthorityBySuperAuthorityAccounts<'_, '_>,
    args: SetRewardAuthorityBySuperAuthorityIxArgs,
) -> ProgramResult {
    set_reward_authority_by_super_authority_invoke_with_program_id(
        crate::ID,
        accounts,
        args,
    )
}
pub fn set_reward_authority_by_super_authority_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SetRewardAuthorityBySuperAuthorityAccounts<'_, '_>,
    args: SetRewardAuthorityBySuperAuthorityIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SetRewardAuthorityBySuperAuthorityKeys = accounts.into();
    let ix = set_reward_authority_by_super_authority_ix_with_program_id(
        program_id,
        keys,
        args,
    )?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn set_reward_authority_by_super_authority_invoke_signed(
    accounts: SetRewardAuthorityBySuperAuthorityAccounts<'_, '_>,
    args: SetRewardAuthorityBySuperAuthorityIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    set_reward_authority_by_super_authority_invoke_signed_with_program_id(
        crate::ID,
        accounts,
        args,
        seeds,
    )
}
pub fn set_reward_authority_by_super_authority_verify_account_keys(
    accounts: SetRewardAuthorityBySuperAuthorityAccounts<'_, '_>,
    keys: SetRewardAuthorityBySuperAuthorityKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.whirlpools_config.key, keys.whirlpools_config),
        (*accounts.whirlpool.key, keys.whirlpool),
        (
            *accounts.reward_emissions_super_authority.key,
            keys.reward_emissions_super_authority,
        ),
        (*accounts.new_reward_authority.key, keys.new_reward_authority),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn set_reward_authority_by_super_authority_verify_writable_privileges<'me, 'info>(
    accounts: SetRewardAuthorityBySuperAuthorityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.whirlpool] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn set_reward_authority_by_super_authority_verify_signer_privileges<'me, 'info>(
    accounts: SetRewardAuthorityBySuperAuthorityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.reward_emissions_super_authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn set_reward_authority_by_super_authority_verify_account_privileges<'me, 'info>(
    accounts: SetRewardAuthorityBySuperAuthorityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    set_reward_authority_by_super_authority_verify_writable_privileges(accounts)?;
    set_reward_authority_by_super_authority_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SET_REWARD_EMISSIONS_SUPER_AUTHORITY_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct SetRewardEmissionsSuperAuthorityAccounts<'me, 'info> {
    pub whirlpools_config: &'me AccountInfo<'info>,
    pub reward_emissions_super_authority: &'me AccountInfo<'info>,
    pub new_reward_emissions_super_authority: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SetRewardEmissionsSuperAuthorityKeys {
    pub whirlpools_config: Pubkey,
    pub reward_emissions_super_authority: Pubkey,
    pub new_reward_emissions_super_authority: Pubkey,
}
impl From<SetRewardEmissionsSuperAuthorityAccounts<'_, '_>>
for SetRewardEmissionsSuperAuthorityKeys {
    fn from(accounts: SetRewardEmissionsSuperAuthorityAccounts) -> Self {
        Self {
            whirlpools_config: *accounts.whirlpools_config.key,
            reward_emissions_super_authority: *accounts
                .reward_emissions_super_authority
                .key,
            new_reward_emissions_super_authority: *accounts
                .new_reward_emissions_super_authority
                .key,
        }
    }
}
impl From<SetRewardEmissionsSuperAuthorityKeys>
for [AccountMeta; SET_REWARD_EMISSIONS_SUPER_AUTHORITY_IX_ACCOUNTS_LEN] {
    fn from(keys: SetRewardEmissionsSuperAuthorityKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.whirlpools_config,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.reward_emissions_super_authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.new_reward_emissions_super_authority,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; SET_REWARD_EMISSIONS_SUPER_AUTHORITY_IX_ACCOUNTS_LEN]>
for SetRewardEmissionsSuperAuthorityKeys {
    fn from(
        pubkeys: [Pubkey; SET_REWARD_EMISSIONS_SUPER_AUTHORITY_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            whirlpools_config: pubkeys[0],
            reward_emissions_super_authority: pubkeys[1],
            new_reward_emissions_super_authority: pubkeys[2],
        }
    }
}
impl<'info> From<SetRewardEmissionsSuperAuthorityAccounts<'_, 'info>>
for [AccountInfo<'info>; SET_REWARD_EMISSIONS_SUPER_AUTHORITY_IX_ACCOUNTS_LEN] {
    fn from(accounts: SetRewardEmissionsSuperAuthorityAccounts<'_, 'info>) -> Self {
        [
            accounts.whirlpools_config.clone(),
            accounts.reward_emissions_super_authority.clone(),
            accounts.new_reward_emissions_super_authority.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; SET_REWARD_EMISSIONS_SUPER_AUTHORITY_IX_ACCOUNTS_LEN]>
for SetRewardEmissionsSuperAuthorityAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<
            'info,
        >; SET_REWARD_EMISSIONS_SUPER_AUTHORITY_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            whirlpools_config: &arr[0],
            reward_emissions_super_authority: &arr[1],
            new_reward_emissions_super_authority: &arr[2],
        }
    }
}
pub const SET_REWARD_EMISSIONS_SUPER_AUTHORITY_IX_DISCM: [u8; 8] = [
    207,
    5,
    200,
    209,
    122,
    56,
    82,
    183,
];
#[derive(Clone, Debug, PartialEq)]
pub struct SetRewardEmissionsSuperAuthorityIxData;
impl SetRewardEmissionsSuperAuthorityIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SET_REWARD_EMISSIONS_SUPER_AUTHORITY_IX_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        SET_REWARD_EMISSIONS_SUPER_AUTHORITY_IX_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SET_REWARD_EMISSIONS_SUPER_AUTHORITY_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn set_reward_emissions_super_authority_ix_with_program_id(
    program_id: Pubkey,
    keys: SetRewardEmissionsSuperAuthorityKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SET_REWARD_EMISSIONS_SUPER_AUTHORITY_IX_ACCOUNTS_LEN] = keys
        .into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: SetRewardEmissionsSuperAuthorityIxData.try_to_vec()?,
    })
}
pub fn set_reward_emissions_super_authority_ix(
    keys: SetRewardEmissionsSuperAuthorityKeys,
) -> std::io::Result<Instruction> {
    set_reward_emissions_super_authority_ix_with_program_id(crate::ID, keys)
}
pub fn set_reward_emissions_super_authority_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SetRewardEmissionsSuperAuthorityAccounts<'_, '_>,
) -> ProgramResult {
    let keys: SetRewardEmissionsSuperAuthorityKeys = accounts.into();
    let ix = set_reward_emissions_super_authority_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn set_reward_emissions_super_authority_invoke(
    accounts: SetRewardEmissionsSuperAuthorityAccounts<'_, '_>,
) -> ProgramResult {
    set_reward_emissions_super_authority_invoke_with_program_id(crate::ID, accounts)
}
pub fn set_reward_emissions_super_authority_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SetRewardEmissionsSuperAuthorityAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SetRewardEmissionsSuperAuthorityKeys = accounts.into();
    let ix = set_reward_emissions_super_authority_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn set_reward_emissions_super_authority_invoke_signed(
    accounts: SetRewardEmissionsSuperAuthorityAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    set_reward_emissions_super_authority_invoke_signed_with_program_id(
        crate::ID,
        accounts,
        seeds,
    )
}
pub fn set_reward_emissions_super_authority_verify_account_keys(
    accounts: SetRewardEmissionsSuperAuthorityAccounts<'_, '_>,
    keys: SetRewardEmissionsSuperAuthorityKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.whirlpools_config.key, keys.whirlpools_config),
        (
            *accounts.reward_emissions_super_authority.key,
            keys.reward_emissions_super_authority,
        ),
        (
            *accounts.new_reward_emissions_super_authority.key,
            keys.new_reward_emissions_super_authority,
        ),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn set_reward_emissions_super_authority_verify_writable_privileges<'me, 'info>(
    accounts: SetRewardEmissionsSuperAuthorityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.whirlpools_config] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn set_reward_emissions_super_authority_verify_signer_privileges<'me, 'info>(
    accounts: SetRewardEmissionsSuperAuthorityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.reward_emissions_super_authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn set_reward_emissions_super_authority_verify_account_privileges<'me, 'info>(
    accounts: SetRewardEmissionsSuperAuthorityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    set_reward_emissions_super_authority_verify_writable_privileges(accounts)?;
    set_reward_emissions_super_authority_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const TWO_HOP_SWAP_IX_ACCOUNTS_LEN: usize = 20;
#[derive(Copy, Clone, Debug)]
pub struct TwoHopSwapAccounts<'me, 'info> {
    pub token_program: &'me AccountInfo<'info>,
    pub token_authority: &'me AccountInfo<'info>,
    pub whirlpool_one: &'me AccountInfo<'info>,
    pub whirlpool_two: &'me AccountInfo<'info>,
    pub token_owner_account_one_a: &'me AccountInfo<'info>,
    pub token_vault_one_a: &'me AccountInfo<'info>,
    pub token_owner_account_one_b: &'me AccountInfo<'info>,
    pub token_vault_one_b: &'me AccountInfo<'info>,
    pub token_owner_account_two_a: &'me AccountInfo<'info>,
    pub token_vault_two_a: &'me AccountInfo<'info>,
    pub token_owner_account_two_b: &'me AccountInfo<'info>,
    pub token_vault_two_b: &'me AccountInfo<'info>,
    pub tick_array_one0: &'me AccountInfo<'info>,
    pub tick_array_one1: &'me AccountInfo<'info>,
    pub tick_array_one2: &'me AccountInfo<'info>,
    pub tick_array_two0: &'me AccountInfo<'info>,
    pub tick_array_two1: &'me AccountInfo<'info>,
    pub tick_array_two2: &'me AccountInfo<'info>,
    pub oracle_one: &'me AccountInfo<'info>,
    pub oracle_two: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct TwoHopSwapKeys {
    pub token_program: Pubkey,
    pub token_authority: Pubkey,
    pub whirlpool_one: Pubkey,
    pub whirlpool_two: Pubkey,
    pub token_owner_account_one_a: Pubkey,
    pub token_vault_one_a: Pubkey,
    pub token_owner_account_one_b: Pubkey,
    pub token_vault_one_b: Pubkey,
    pub token_owner_account_two_a: Pubkey,
    pub token_vault_two_a: Pubkey,
    pub token_owner_account_two_b: Pubkey,
    pub token_vault_two_b: Pubkey,
    pub tick_array_one0: Pubkey,
    pub tick_array_one1: Pubkey,
    pub tick_array_one2: Pubkey,
    pub tick_array_two0: Pubkey,
    pub tick_array_two1: Pubkey,
    pub tick_array_two2: Pubkey,
    pub oracle_one: Pubkey,
    pub oracle_two: Pubkey,
}
impl From<TwoHopSwapAccounts<'_, '_>> for TwoHopSwapKeys {
    fn from(accounts: TwoHopSwapAccounts) -> Self {
        Self {
            token_program: *accounts.token_program.key,
            token_authority: *accounts.token_authority.key,
            whirlpool_one: *accounts.whirlpool_one.key,
            whirlpool_two: *accounts.whirlpool_two.key,
            token_owner_account_one_a: *accounts.token_owner_account_one_a.key,
            token_vault_one_a: *accounts.token_vault_one_a.key,
            token_owner_account_one_b: *accounts.token_owner_account_one_b.key,
            token_vault_one_b: *accounts.token_vault_one_b.key,
            token_owner_account_two_a: *accounts.token_owner_account_two_a.key,
            token_vault_two_a: *accounts.token_vault_two_a.key,
            token_owner_account_two_b: *accounts.token_owner_account_two_b.key,
            token_vault_two_b: *accounts.token_vault_two_b.key,
            tick_array_one0: *accounts.tick_array_one0.key,
            tick_array_one1: *accounts.tick_array_one1.key,
            tick_array_one2: *accounts.tick_array_one2.key,
            tick_array_two0: *accounts.tick_array_two0.key,
            tick_array_two1: *accounts.tick_array_two1.key,
            tick_array_two2: *accounts.tick_array_two2.key,
            oracle_one: *accounts.oracle_one.key,
            oracle_two: *accounts.oracle_two.key,
        }
    }
}
impl From<TwoHopSwapKeys> for [AccountMeta; TWO_HOP_SWAP_IX_ACCOUNTS_LEN] {
    fn from(keys: TwoHopSwapKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.whirlpool_one,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.whirlpool_two,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_owner_account_one_a,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_vault_one_a,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_owner_account_one_b,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_vault_one_b,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_owner_account_two_a,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_vault_two_a,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_owner_account_two_b,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_vault_two_b,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.tick_array_one0,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.tick_array_one1,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.tick_array_one2,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.tick_array_two0,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.tick_array_two1,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.tick_array_two2,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.oracle_one,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.oracle_two,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; TWO_HOP_SWAP_IX_ACCOUNTS_LEN]> for TwoHopSwapKeys {
    fn from(pubkeys: [Pubkey; TWO_HOP_SWAP_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            token_program: pubkeys[0],
            token_authority: pubkeys[1],
            whirlpool_one: pubkeys[2],
            whirlpool_two: pubkeys[3],
            token_owner_account_one_a: pubkeys[4],
            token_vault_one_a: pubkeys[5],
            token_owner_account_one_b: pubkeys[6],
            token_vault_one_b: pubkeys[7],
            token_owner_account_two_a: pubkeys[8],
            token_vault_two_a: pubkeys[9],
            token_owner_account_two_b: pubkeys[10],
            token_vault_two_b: pubkeys[11],
            tick_array_one0: pubkeys[12],
            tick_array_one1: pubkeys[13],
            tick_array_one2: pubkeys[14],
            tick_array_two0: pubkeys[15],
            tick_array_two1: pubkeys[16],
            tick_array_two2: pubkeys[17],
            oracle_one: pubkeys[18],
            oracle_two: pubkeys[19],
        }
    }
}
impl<'info> From<TwoHopSwapAccounts<'_, 'info>>
for [AccountInfo<'info>; TWO_HOP_SWAP_IX_ACCOUNTS_LEN] {
    fn from(accounts: TwoHopSwapAccounts<'_, 'info>) -> Self {
        [
            accounts.token_program.clone(),
            accounts.token_authority.clone(),
            accounts.whirlpool_one.clone(),
            accounts.whirlpool_two.clone(),
            accounts.token_owner_account_one_a.clone(),
            accounts.token_vault_one_a.clone(),
            accounts.token_owner_account_one_b.clone(),
            accounts.token_vault_one_b.clone(),
            accounts.token_owner_account_two_a.clone(),
            accounts.token_vault_two_a.clone(),
            accounts.token_owner_account_two_b.clone(),
            accounts.token_vault_two_b.clone(),
            accounts.tick_array_one0.clone(),
            accounts.tick_array_one1.clone(),
            accounts.tick_array_one2.clone(),
            accounts.tick_array_two0.clone(),
            accounts.tick_array_two1.clone(),
            accounts.tick_array_two2.clone(),
            accounts.oracle_one.clone(),
            accounts.oracle_two.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; TWO_HOP_SWAP_IX_ACCOUNTS_LEN]>
for TwoHopSwapAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; TWO_HOP_SWAP_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            token_program: &arr[0],
            token_authority: &arr[1],
            whirlpool_one: &arr[2],
            whirlpool_two: &arr[3],
            token_owner_account_one_a: &arr[4],
            token_vault_one_a: &arr[5],
            token_owner_account_one_b: &arr[6],
            token_vault_one_b: &arr[7],
            token_owner_account_two_a: &arr[8],
            token_vault_two_a: &arr[9],
            token_owner_account_two_b: &arr[10],
            token_vault_two_b: &arr[11],
            tick_array_one0: &arr[12],
            tick_array_one1: &arr[13],
            tick_array_one2: &arr[14],
            tick_array_two0: &arr[15],
            tick_array_two1: &arr[16],
            tick_array_two2: &arr[17],
            oracle_one: &arr[18],
            oracle_two: &arr[19],
        }
    }
}
pub const TWO_HOP_SWAP_IX_DISCM: [u8; 8] = [195, 96, 237, 108, 68, 162, 219, 230];
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TwoHopSwapIxArgs {
    pub amount: u64,
    pub other_amount_threshold: u64,
    pub amount_specified_is_input: bool,
    pub a_to_b_one: bool,
    pub a_to_b_two: bool,
    pub sqrt_price_limit_one: u128,
    pub sqrt_price_limit_two: u128,
}
#[derive(Clone, Debug, PartialEq)]
pub struct TwoHopSwapIxData(pub TwoHopSwapIxArgs);
impl From<TwoHopSwapIxArgs> for TwoHopSwapIxData {
    fn from(args: TwoHopSwapIxArgs) -> Self {
        Self(args)
    }
}
impl TwoHopSwapIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != TWO_HOP_SWAP_IX_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        TWO_HOP_SWAP_IX_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(TwoHopSwapIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&TWO_HOP_SWAP_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn two_hop_swap_ix_with_program_id(
    program_id: Pubkey,
    keys: TwoHopSwapKeys,
    args: TwoHopSwapIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; TWO_HOP_SWAP_IX_ACCOUNTS_LEN] = keys.into();
    let data: TwoHopSwapIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn two_hop_swap_ix(
    keys: TwoHopSwapKeys,
    args: TwoHopSwapIxArgs,
) -> std::io::Result<Instruction> {
    two_hop_swap_ix_with_program_id(crate::ID, keys, args)
}
pub fn two_hop_swap_invoke_with_program_id(
    program_id: Pubkey,
    accounts: TwoHopSwapAccounts<'_, '_>,
    args: TwoHopSwapIxArgs,
) -> ProgramResult {
    let keys: TwoHopSwapKeys = accounts.into();
    let ix = two_hop_swap_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn two_hop_swap_invoke(
    accounts: TwoHopSwapAccounts<'_, '_>,
    args: TwoHopSwapIxArgs,
) -> ProgramResult {
    two_hop_swap_invoke_with_program_id(crate::ID, accounts, args)
}
pub fn two_hop_swap_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: TwoHopSwapAccounts<'_, '_>,
    args: TwoHopSwapIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: TwoHopSwapKeys = accounts.into();
    let ix = two_hop_swap_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn two_hop_swap_invoke_signed(
    accounts: TwoHopSwapAccounts<'_, '_>,
    args: TwoHopSwapIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    two_hop_swap_invoke_signed_with_program_id(crate::ID, accounts, args, seeds)
}
pub fn two_hop_swap_verify_account_keys(
    accounts: TwoHopSwapAccounts<'_, '_>,
    keys: TwoHopSwapKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.token_program.key, keys.token_program),
        (*accounts.token_authority.key, keys.token_authority),
        (*accounts.whirlpool_one.key, keys.whirlpool_one),
        (*accounts.whirlpool_two.key, keys.whirlpool_two),
        (*accounts.token_owner_account_one_a.key, keys.token_owner_account_one_a),
        (*accounts.token_vault_one_a.key, keys.token_vault_one_a),
        (*accounts.token_owner_account_one_b.key, keys.token_owner_account_one_b),
        (*accounts.token_vault_one_b.key, keys.token_vault_one_b),
        (*accounts.token_owner_account_two_a.key, keys.token_owner_account_two_a),
        (*accounts.token_vault_two_a.key, keys.token_vault_two_a),
        (*accounts.token_owner_account_two_b.key, keys.token_owner_account_two_b),
        (*accounts.token_vault_two_b.key, keys.token_vault_two_b),
        (*accounts.tick_array_one0.key, keys.tick_array_one0),
        (*accounts.tick_array_one1.key, keys.tick_array_one1),
        (*accounts.tick_array_one2.key, keys.tick_array_one2),
        (*accounts.tick_array_two0.key, keys.tick_array_two0),
        (*accounts.tick_array_two1.key, keys.tick_array_two1),
        (*accounts.tick_array_two2.key, keys.tick_array_two2),
        (*accounts.oracle_one.key, keys.oracle_one),
        (*accounts.oracle_two.key, keys.oracle_two),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn two_hop_swap_verify_writable_privileges<'me, 'info>(
    accounts: TwoHopSwapAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.whirlpool_one,
        accounts.whirlpool_two,
        accounts.token_owner_account_one_a,
        accounts.token_vault_one_a,
        accounts.token_owner_account_one_b,
        accounts.token_vault_one_b,
        accounts.token_owner_account_two_a,
        accounts.token_vault_two_a,
        accounts.token_owner_account_two_b,
        accounts.token_vault_two_b,
        accounts.tick_array_one0,
        accounts.tick_array_one1,
        accounts.tick_array_one2,
        accounts.tick_array_two0,
        accounts.tick_array_two1,
        accounts.tick_array_two2,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn two_hop_swap_verify_signer_privileges<'me, 'info>(
    accounts: TwoHopSwapAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.token_authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn two_hop_swap_verify_account_privileges<'me, 'info>(
    accounts: TwoHopSwapAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    two_hop_swap_verify_writable_privileges(accounts)?;
    two_hop_swap_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const INITIALIZE_POSITION_BUNDLE_IX_ACCOUNTS_LEN: usize = 9;
#[derive(Copy, Clone, Debug)]
pub struct InitializePositionBundleAccounts<'me, 'info> {
    pub position_bundle: &'me AccountInfo<'info>,
    pub position_bundle_mint: &'me AccountInfo<'info>,
    pub position_bundle_token_account: &'me AccountInfo<'info>,
    pub position_bundle_owner: &'me AccountInfo<'info>,
    pub funder: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub rent: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InitializePositionBundleKeys {
    pub position_bundle: Pubkey,
    pub position_bundle_mint: Pubkey,
    pub position_bundle_token_account: Pubkey,
    pub position_bundle_owner: Pubkey,
    pub funder: Pubkey,
    pub token_program: Pubkey,
    pub system_program: Pubkey,
    pub rent: Pubkey,
    pub associated_token_program: Pubkey,
}
impl From<InitializePositionBundleAccounts<'_, '_>> for InitializePositionBundleKeys {
    fn from(accounts: InitializePositionBundleAccounts) -> Self {
        Self {
            position_bundle: *accounts.position_bundle.key,
            position_bundle_mint: *accounts.position_bundle_mint.key,
            position_bundle_token_account: *accounts.position_bundle_token_account.key,
            position_bundle_owner: *accounts.position_bundle_owner.key,
            funder: *accounts.funder.key,
            token_program: *accounts.token_program.key,
            system_program: *accounts.system_program.key,
            rent: *accounts.rent.key,
            associated_token_program: *accounts.associated_token_program.key,
        }
    }
}
impl From<InitializePositionBundleKeys>
for [AccountMeta; INITIALIZE_POSITION_BUNDLE_IX_ACCOUNTS_LEN] {
    fn from(keys: InitializePositionBundleKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.position_bundle,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.position_bundle_mint,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.position_bundle_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.position_bundle_owner,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.funder,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.system_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.rent,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.associated_token_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; INITIALIZE_POSITION_BUNDLE_IX_ACCOUNTS_LEN]>
for InitializePositionBundleKeys {
    fn from(pubkeys: [Pubkey; INITIALIZE_POSITION_BUNDLE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            position_bundle: pubkeys[0],
            position_bundle_mint: pubkeys[1],
            position_bundle_token_account: pubkeys[2],
            position_bundle_owner: pubkeys[3],
            funder: pubkeys[4],
            token_program: pubkeys[5],
            system_program: pubkeys[6],
            rent: pubkeys[7],
            associated_token_program: pubkeys[8],
        }
    }
}
impl<'info> From<InitializePositionBundleAccounts<'_, 'info>>
for [AccountInfo<'info>; INITIALIZE_POSITION_BUNDLE_IX_ACCOUNTS_LEN] {
    fn from(accounts: InitializePositionBundleAccounts<'_, 'info>) -> Self {
        [
            accounts.position_bundle.clone(),
            accounts.position_bundle_mint.clone(),
            accounts.position_bundle_token_account.clone(),
            accounts.position_bundle_owner.clone(),
            accounts.funder.clone(),
            accounts.token_program.clone(),
            accounts.system_program.clone(),
            accounts.rent.clone(),
            accounts.associated_token_program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; INITIALIZE_POSITION_BUNDLE_IX_ACCOUNTS_LEN]>
for InitializePositionBundleAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; INITIALIZE_POSITION_BUNDLE_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            position_bundle: &arr[0],
            position_bundle_mint: &arr[1],
            position_bundle_token_account: &arr[2],
            position_bundle_owner: &arr[3],
            funder: &arr[4],
            token_program: &arr[5],
            system_program: &arr[6],
            rent: &arr[7],
            associated_token_program: &arr[8],
        }
    }
}
pub const INITIALIZE_POSITION_BUNDLE_IX_DISCM: [u8; 8] = [
    117,
    45,
    241,
    149,
    24,
    18,
    194,
    65,
];
#[derive(Clone, Debug, PartialEq)]
pub struct InitializePositionBundleIxData;
impl InitializePositionBundleIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INITIALIZE_POSITION_BUNDLE_IX_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        INITIALIZE_POSITION_BUNDLE_IX_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INITIALIZE_POSITION_BUNDLE_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn initialize_position_bundle_ix_with_program_id(
    program_id: Pubkey,
    keys: InitializePositionBundleKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INITIALIZE_POSITION_BUNDLE_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: InitializePositionBundleIxData.try_to_vec()?,
    })
}
pub fn initialize_position_bundle_ix(
    keys: InitializePositionBundleKeys,
) -> std::io::Result<Instruction> {
    initialize_position_bundle_ix_with_program_id(crate::ID, keys)
}
pub fn initialize_position_bundle_invoke_with_program_id(
    program_id: Pubkey,
    accounts: InitializePositionBundleAccounts<'_, '_>,
) -> ProgramResult {
    let keys: InitializePositionBundleKeys = accounts.into();
    let ix = initialize_position_bundle_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn initialize_position_bundle_invoke(
    accounts: InitializePositionBundleAccounts<'_, '_>,
) -> ProgramResult {
    initialize_position_bundle_invoke_with_program_id(crate::ID, accounts)
}
pub fn initialize_position_bundle_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: InitializePositionBundleAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: InitializePositionBundleKeys = accounts.into();
    let ix = initialize_position_bundle_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn initialize_position_bundle_invoke_signed(
    accounts: InitializePositionBundleAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    initialize_position_bundle_invoke_signed_with_program_id(crate::ID, accounts, seeds)
}
pub fn initialize_position_bundle_verify_account_keys(
    accounts: InitializePositionBundleAccounts<'_, '_>,
    keys: InitializePositionBundleKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.position_bundle.key, keys.position_bundle),
        (*accounts.position_bundle_mint.key, keys.position_bundle_mint),
        (
            *accounts.position_bundle_token_account.key,
            keys.position_bundle_token_account,
        ),
        (*accounts.position_bundle_owner.key, keys.position_bundle_owner),
        (*accounts.funder.key, keys.funder),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.rent.key, keys.rent),
        (*accounts.associated_token_program.key, keys.associated_token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn initialize_position_bundle_verify_writable_privileges<'me, 'info>(
    accounts: InitializePositionBundleAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.position_bundle,
        accounts.position_bundle_mint,
        accounts.position_bundle_token_account,
        accounts.funder,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn initialize_position_bundle_verify_signer_privileges<'me, 'info>(
    accounts: InitializePositionBundleAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.position_bundle_mint, accounts.funder] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn initialize_position_bundle_verify_account_privileges<'me, 'info>(
    accounts: InitializePositionBundleAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    initialize_position_bundle_verify_writable_privileges(accounts)?;
    initialize_position_bundle_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const INITIALIZE_POSITION_BUNDLE_WITH_METADATA_IX_ACCOUNTS_LEN: usize = 12;
#[derive(Copy, Clone, Debug)]
pub struct InitializePositionBundleWithMetadataAccounts<'me, 'info> {
    pub position_bundle: &'me AccountInfo<'info>,
    pub position_bundle_mint: &'me AccountInfo<'info>,
    pub position_bundle_metadata: &'me AccountInfo<'info>,
    pub position_bundle_token_account: &'me AccountInfo<'info>,
    pub position_bundle_owner: &'me AccountInfo<'info>,
    pub funder: &'me AccountInfo<'info>,
    pub metadata_update_auth: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub rent: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
    pub metadata_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InitializePositionBundleWithMetadataKeys {
    pub position_bundle: Pubkey,
    pub position_bundle_mint: Pubkey,
    pub position_bundle_metadata: Pubkey,
    pub position_bundle_token_account: Pubkey,
    pub position_bundle_owner: Pubkey,
    pub funder: Pubkey,
    pub metadata_update_auth: Pubkey,
    pub token_program: Pubkey,
    pub system_program: Pubkey,
    pub rent: Pubkey,
    pub associated_token_program: Pubkey,
    pub metadata_program: Pubkey,
}
impl From<InitializePositionBundleWithMetadataAccounts<'_, '_>>
for InitializePositionBundleWithMetadataKeys {
    fn from(accounts: InitializePositionBundleWithMetadataAccounts) -> Self {
        Self {
            position_bundle: *accounts.position_bundle.key,
            position_bundle_mint: *accounts.position_bundle_mint.key,
            position_bundle_metadata: *accounts.position_bundle_metadata.key,
            position_bundle_token_account: *accounts.position_bundle_token_account.key,
            position_bundle_owner: *accounts.position_bundle_owner.key,
            funder: *accounts.funder.key,
            metadata_update_auth: *accounts.metadata_update_auth.key,
            token_program: *accounts.token_program.key,
            system_program: *accounts.system_program.key,
            rent: *accounts.rent.key,
            associated_token_program: *accounts.associated_token_program.key,
            metadata_program: *accounts.metadata_program.key,
        }
    }
}
impl From<InitializePositionBundleWithMetadataKeys>
for [AccountMeta; INITIALIZE_POSITION_BUNDLE_WITH_METADATA_IX_ACCOUNTS_LEN] {
    fn from(keys: InitializePositionBundleWithMetadataKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.position_bundle,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.position_bundle_mint,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.position_bundle_metadata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.position_bundle_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.position_bundle_owner,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.funder,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.metadata_update_auth,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.system_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.rent,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.associated_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.metadata_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; INITIALIZE_POSITION_BUNDLE_WITH_METADATA_IX_ACCOUNTS_LEN]>
for InitializePositionBundleWithMetadataKeys {
    fn from(
        pubkeys: [Pubkey; INITIALIZE_POSITION_BUNDLE_WITH_METADATA_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            position_bundle: pubkeys[0],
            position_bundle_mint: pubkeys[1],
            position_bundle_metadata: pubkeys[2],
            position_bundle_token_account: pubkeys[3],
            position_bundle_owner: pubkeys[4],
            funder: pubkeys[5],
            metadata_update_auth: pubkeys[6],
            token_program: pubkeys[7],
            system_program: pubkeys[8],
            rent: pubkeys[9],
            associated_token_program: pubkeys[10],
            metadata_program: pubkeys[11],
        }
    }
}
impl<'info> From<InitializePositionBundleWithMetadataAccounts<'_, 'info>>
for [AccountInfo<'info>; INITIALIZE_POSITION_BUNDLE_WITH_METADATA_IX_ACCOUNTS_LEN] {
    fn from(accounts: InitializePositionBundleWithMetadataAccounts<'_, 'info>) -> Self {
        [
            accounts.position_bundle.clone(),
            accounts.position_bundle_mint.clone(),
            accounts.position_bundle_metadata.clone(),
            accounts.position_bundle_token_account.clone(),
            accounts.position_bundle_owner.clone(),
            accounts.funder.clone(),
            accounts.metadata_update_auth.clone(),
            accounts.token_program.clone(),
            accounts.system_program.clone(),
            accounts.rent.clone(),
            accounts.associated_token_program.clone(),
            accounts.metadata_program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<
    &'me [AccountInfo<'info>; INITIALIZE_POSITION_BUNDLE_WITH_METADATA_IX_ACCOUNTS_LEN],
> for InitializePositionBundleWithMetadataAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<
            'info,
        >; INITIALIZE_POSITION_BUNDLE_WITH_METADATA_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            position_bundle: &arr[0],
            position_bundle_mint: &arr[1],
            position_bundle_metadata: &arr[2],
            position_bundle_token_account: &arr[3],
            position_bundle_owner: &arr[4],
            funder: &arr[5],
            metadata_update_auth: &arr[6],
            token_program: &arr[7],
            system_program: &arr[8],
            rent: &arr[9],
            associated_token_program: &arr[10],
            metadata_program: &arr[11],
        }
    }
}
pub const INITIALIZE_POSITION_BUNDLE_WITH_METADATA_IX_DISCM: [u8; 8] = [
    93,
    124,
    16,
    179,
    249,
    131,
    115,
    245,
];
#[derive(Clone, Debug, PartialEq)]
pub struct InitializePositionBundleWithMetadataIxData;
impl InitializePositionBundleWithMetadataIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INITIALIZE_POSITION_BUNDLE_WITH_METADATA_IX_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        INITIALIZE_POSITION_BUNDLE_WITH_METADATA_IX_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INITIALIZE_POSITION_BUNDLE_WITH_METADATA_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn initialize_position_bundle_with_metadata_ix_with_program_id(
    program_id: Pubkey,
    keys: InitializePositionBundleWithMetadataKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INITIALIZE_POSITION_BUNDLE_WITH_METADATA_IX_ACCOUNTS_LEN] = keys
        .into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: InitializePositionBundleWithMetadataIxData.try_to_vec()?,
    })
}
pub fn initialize_position_bundle_with_metadata_ix(
    keys: InitializePositionBundleWithMetadataKeys,
) -> std::io::Result<Instruction> {
    initialize_position_bundle_with_metadata_ix_with_program_id(crate::ID, keys)
}
pub fn initialize_position_bundle_with_metadata_invoke_with_program_id(
    program_id: Pubkey,
    accounts: InitializePositionBundleWithMetadataAccounts<'_, '_>,
) -> ProgramResult {
    let keys: InitializePositionBundleWithMetadataKeys = accounts.into();
    let ix = initialize_position_bundle_with_metadata_ix_with_program_id(
        program_id,
        keys,
    )?;
    invoke_instruction(&ix, accounts)
}
pub fn initialize_position_bundle_with_metadata_invoke(
    accounts: InitializePositionBundleWithMetadataAccounts<'_, '_>,
) -> ProgramResult {
    initialize_position_bundle_with_metadata_invoke_with_program_id(crate::ID, accounts)
}
pub fn initialize_position_bundle_with_metadata_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: InitializePositionBundleWithMetadataAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: InitializePositionBundleWithMetadataKeys = accounts.into();
    let ix = initialize_position_bundle_with_metadata_ix_with_program_id(
        program_id,
        keys,
    )?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn initialize_position_bundle_with_metadata_invoke_signed(
    accounts: InitializePositionBundleWithMetadataAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    initialize_position_bundle_with_metadata_invoke_signed_with_program_id(
        crate::ID,
        accounts,
        seeds,
    )
}
pub fn initialize_position_bundle_with_metadata_verify_account_keys(
    accounts: InitializePositionBundleWithMetadataAccounts<'_, '_>,
    keys: InitializePositionBundleWithMetadataKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.position_bundle.key, keys.position_bundle),
        (*accounts.position_bundle_mint.key, keys.position_bundle_mint),
        (*accounts.position_bundle_metadata.key, keys.position_bundle_metadata),
        (
            *accounts.position_bundle_token_account.key,
            keys.position_bundle_token_account,
        ),
        (*accounts.position_bundle_owner.key, keys.position_bundle_owner),
        (*accounts.funder.key, keys.funder),
        (*accounts.metadata_update_auth.key, keys.metadata_update_auth),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.rent.key, keys.rent),
        (*accounts.associated_token_program.key, keys.associated_token_program),
        (*accounts.metadata_program.key, keys.metadata_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn initialize_position_bundle_with_metadata_verify_writable_privileges<'me, 'info>(
    accounts: InitializePositionBundleWithMetadataAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.position_bundle,
        accounts.position_bundle_mint,
        accounts.position_bundle_metadata,
        accounts.position_bundle_token_account,
        accounts.funder,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn initialize_position_bundle_with_metadata_verify_signer_privileges<'me, 'info>(
    accounts: InitializePositionBundleWithMetadataAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.position_bundle_mint, accounts.funder] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn initialize_position_bundle_with_metadata_verify_account_privileges<'me, 'info>(
    accounts: InitializePositionBundleWithMetadataAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    initialize_position_bundle_with_metadata_verify_writable_privileges(accounts)?;
    initialize_position_bundle_with_metadata_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const DELETE_POSITION_BUNDLE_IX_ACCOUNTS_LEN: usize = 6;
#[derive(Copy, Clone, Debug)]
pub struct DeletePositionBundleAccounts<'me, 'info> {
    pub position_bundle: &'me AccountInfo<'info>,
    pub position_bundle_mint: &'me AccountInfo<'info>,
    pub position_bundle_token_account: &'me AccountInfo<'info>,
    pub position_bundle_owner: &'me AccountInfo<'info>,
    pub receiver: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DeletePositionBundleKeys {
    pub position_bundle: Pubkey,
    pub position_bundle_mint: Pubkey,
    pub position_bundle_token_account: Pubkey,
    pub position_bundle_owner: Pubkey,
    pub receiver: Pubkey,
    pub token_program: Pubkey,
}
impl From<DeletePositionBundleAccounts<'_, '_>> for DeletePositionBundleKeys {
    fn from(accounts: DeletePositionBundleAccounts) -> Self {
        Self {
            position_bundle: *accounts.position_bundle.key,
            position_bundle_mint: *accounts.position_bundle_mint.key,
            position_bundle_token_account: *accounts.position_bundle_token_account.key,
            position_bundle_owner: *accounts.position_bundle_owner.key,
            receiver: *accounts.receiver.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<DeletePositionBundleKeys>
for [AccountMeta; DELETE_POSITION_BUNDLE_IX_ACCOUNTS_LEN] {
    fn from(keys: DeletePositionBundleKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.position_bundle,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.position_bundle_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.position_bundle_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.position_bundle_owner,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.receiver,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; DELETE_POSITION_BUNDLE_IX_ACCOUNTS_LEN]>
for DeletePositionBundleKeys {
    fn from(pubkeys: [Pubkey; DELETE_POSITION_BUNDLE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            position_bundle: pubkeys[0],
            position_bundle_mint: pubkeys[1],
            position_bundle_token_account: pubkeys[2],
            position_bundle_owner: pubkeys[3],
            receiver: pubkeys[4],
            token_program: pubkeys[5],
        }
    }
}
impl<'info> From<DeletePositionBundleAccounts<'_, 'info>>
for [AccountInfo<'info>; DELETE_POSITION_BUNDLE_IX_ACCOUNTS_LEN] {
    fn from(accounts: DeletePositionBundleAccounts<'_, 'info>) -> Self {
        [
            accounts.position_bundle.clone(),
            accounts.position_bundle_mint.clone(),
            accounts.position_bundle_token_account.clone(),
            accounts.position_bundle_owner.clone(),
            accounts.receiver.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; DELETE_POSITION_BUNDLE_IX_ACCOUNTS_LEN]>
for DeletePositionBundleAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; DELETE_POSITION_BUNDLE_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            position_bundle: &arr[0],
            position_bundle_mint: &arr[1],
            position_bundle_token_account: &arr[2],
            position_bundle_owner: &arr[3],
            receiver: &arr[4],
            token_program: &arr[5],
        }
    }
}
pub const DELETE_POSITION_BUNDLE_IX_DISCM: [u8; 8] = [
    100,
    25,
    99,
    2,
    217,
    239,
    124,
    173,
];
#[derive(Clone, Debug, PartialEq)]
pub struct DeletePositionBundleIxData;
impl DeletePositionBundleIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != DELETE_POSITION_BUNDLE_IX_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        DELETE_POSITION_BUNDLE_IX_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&DELETE_POSITION_BUNDLE_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn delete_position_bundle_ix_with_program_id(
    program_id: Pubkey,
    keys: DeletePositionBundleKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; DELETE_POSITION_BUNDLE_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: DeletePositionBundleIxData.try_to_vec()?,
    })
}
pub fn delete_position_bundle_ix(
    keys: DeletePositionBundleKeys,
) -> std::io::Result<Instruction> {
    delete_position_bundle_ix_with_program_id(crate::ID, keys)
}
pub fn delete_position_bundle_invoke_with_program_id(
    program_id: Pubkey,
    accounts: DeletePositionBundleAccounts<'_, '_>,
) -> ProgramResult {
    let keys: DeletePositionBundleKeys = accounts.into();
    let ix = delete_position_bundle_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn delete_position_bundle_invoke(
    accounts: DeletePositionBundleAccounts<'_, '_>,
) -> ProgramResult {
    delete_position_bundle_invoke_with_program_id(crate::ID, accounts)
}
pub fn delete_position_bundle_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: DeletePositionBundleAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: DeletePositionBundleKeys = accounts.into();
    let ix = delete_position_bundle_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn delete_position_bundle_invoke_signed(
    accounts: DeletePositionBundleAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    delete_position_bundle_invoke_signed_with_program_id(crate::ID, accounts, seeds)
}
pub fn delete_position_bundle_verify_account_keys(
    accounts: DeletePositionBundleAccounts<'_, '_>,
    keys: DeletePositionBundleKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.position_bundle.key, keys.position_bundle),
        (*accounts.position_bundle_mint.key, keys.position_bundle_mint),
        (
            *accounts.position_bundle_token_account.key,
            keys.position_bundle_token_account,
        ),
        (*accounts.position_bundle_owner.key, keys.position_bundle_owner),
        (*accounts.receiver.key, keys.receiver),
        (*accounts.token_program.key, keys.token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn delete_position_bundle_verify_writable_privileges<'me, 'info>(
    accounts: DeletePositionBundleAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.position_bundle,
        accounts.position_bundle_mint,
        accounts.position_bundle_token_account,
        accounts.receiver,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn delete_position_bundle_verify_signer_privileges<'me, 'info>(
    accounts: DeletePositionBundleAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.position_bundle_owner] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn delete_position_bundle_verify_account_privileges<'me, 'info>(
    accounts: DeletePositionBundleAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    delete_position_bundle_verify_writable_privileges(accounts)?;
    delete_position_bundle_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const OPEN_BUNDLED_POSITION_IX_ACCOUNTS_LEN: usize = 8;
#[derive(Copy, Clone, Debug)]
pub struct OpenBundledPositionAccounts<'me, 'info> {
    pub bundled_position: &'me AccountInfo<'info>,
    pub position_bundle: &'me AccountInfo<'info>,
    pub position_bundle_token_account: &'me AccountInfo<'info>,
    pub position_bundle_authority: &'me AccountInfo<'info>,
    pub whirlpool: &'me AccountInfo<'info>,
    pub funder: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub rent: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct OpenBundledPositionKeys {
    pub bundled_position: Pubkey,
    pub position_bundle: Pubkey,
    pub position_bundle_token_account: Pubkey,
    pub position_bundle_authority: Pubkey,
    pub whirlpool: Pubkey,
    pub funder: Pubkey,
    pub system_program: Pubkey,
    pub rent: Pubkey,
}
impl From<OpenBundledPositionAccounts<'_, '_>> for OpenBundledPositionKeys {
    fn from(accounts: OpenBundledPositionAccounts) -> Self {
        Self {
            bundled_position: *accounts.bundled_position.key,
            position_bundle: *accounts.position_bundle.key,
            position_bundle_token_account: *accounts.position_bundle_token_account.key,
            position_bundle_authority: *accounts.position_bundle_authority.key,
            whirlpool: *accounts.whirlpool.key,
            funder: *accounts.funder.key,
            system_program: *accounts.system_program.key,
            rent: *accounts.rent.key,
        }
    }
}
impl From<OpenBundledPositionKeys>
for [AccountMeta; OPEN_BUNDLED_POSITION_IX_ACCOUNTS_LEN] {
    fn from(keys: OpenBundledPositionKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.bundled_position,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.position_bundle,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.position_bundle_token_account,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.position_bundle_authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.whirlpool,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.funder,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.system_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.rent,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; OPEN_BUNDLED_POSITION_IX_ACCOUNTS_LEN]> for OpenBundledPositionKeys {
    fn from(pubkeys: [Pubkey; OPEN_BUNDLED_POSITION_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            bundled_position: pubkeys[0],
            position_bundle: pubkeys[1],
            position_bundle_token_account: pubkeys[2],
            position_bundle_authority: pubkeys[3],
            whirlpool: pubkeys[4],
            funder: pubkeys[5],
            system_program: pubkeys[6],
            rent: pubkeys[7],
        }
    }
}
impl<'info> From<OpenBundledPositionAccounts<'_, 'info>>
for [AccountInfo<'info>; OPEN_BUNDLED_POSITION_IX_ACCOUNTS_LEN] {
    fn from(accounts: OpenBundledPositionAccounts<'_, 'info>) -> Self {
        [
            accounts.bundled_position.clone(),
            accounts.position_bundle.clone(),
            accounts.position_bundle_token_account.clone(),
            accounts.position_bundle_authority.clone(),
            accounts.whirlpool.clone(),
            accounts.funder.clone(),
            accounts.system_program.clone(),
            accounts.rent.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; OPEN_BUNDLED_POSITION_IX_ACCOUNTS_LEN]>
for OpenBundledPositionAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; OPEN_BUNDLED_POSITION_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            bundled_position: &arr[0],
            position_bundle: &arr[1],
            position_bundle_token_account: &arr[2],
            position_bundle_authority: &arr[3],
            whirlpool: &arr[4],
            funder: &arr[5],
            system_program: &arr[6],
            rent: &arr[7],
        }
    }
}
pub const OPEN_BUNDLED_POSITION_IX_DISCM: [u8; 8] = [
    169,
    113,
    126,
    171,
    213,
    172,
    212,
    49,
];
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct OpenBundledPositionIxArgs {
    pub bundle_index: u16,
    pub tick_lower_index: i32,
    pub tick_upper_index: i32,
}
#[derive(Clone, Debug, PartialEq)]
pub struct OpenBundledPositionIxData(pub OpenBundledPositionIxArgs);
impl From<OpenBundledPositionIxArgs> for OpenBundledPositionIxData {
    fn from(args: OpenBundledPositionIxArgs) -> Self {
        Self(args)
    }
}
impl OpenBundledPositionIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != OPEN_BUNDLED_POSITION_IX_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        OPEN_BUNDLED_POSITION_IX_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(OpenBundledPositionIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&OPEN_BUNDLED_POSITION_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn open_bundled_position_ix_with_program_id(
    program_id: Pubkey,
    keys: OpenBundledPositionKeys,
    args: OpenBundledPositionIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; OPEN_BUNDLED_POSITION_IX_ACCOUNTS_LEN] = keys.into();
    let data: OpenBundledPositionIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn open_bundled_position_ix(
    keys: OpenBundledPositionKeys,
    args: OpenBundledPositionIxArgs,
) -> std::io::Result<Instruction> {
    open_bundled_position_ix_with_program_id(crate::ID, keys, args)
}
pub fn open_bundled_position_invoke_with_program_id(
    program_id: Pubkey,
    accounts: OpenBundledPositionAccounts<'_, '_>,
    args: OpenBundledPositionIxArgs,
) -> ProgramResult {
    let keys: OpenBundledPositionKeys = accounts.into();
    let ix = open_bundled_position_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn open_bundled_position_invoke(
    accounts: OpenBundledPositionAccounts<'_, '_>,
    args: OpenBundledPositionIxArgs,
) -> ProgramResult {
    open_bundled_position_invoke_with_program_id(crate::ID, accounts, args)
}
pub fn open_bundled_position_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: OpenBundledPositionAccounts<'_, '_>,
    args: OpenBundledPositionIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: OpenBundledPositionKeys = accounts.into();
    let ix = open_bundled_position_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn open_bundled_position_invoke_signed(
    accounts: OpenBundledPositionAccounts<'_, '_>,
    args: OpenBundledPositionIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    open_bundled_position_invoke_signed_with_program_id(crate::ID, accounts, args, seeds)
}
pub fn open_bundled_position_verify_account_keys(
    accounts: OpenBundledPositionAccounts<'_, '_>,
    keys: OpenBundledPositionKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.bundled_position.key, keys.bundled_position),
        (*accounts.position_bundle.key, keys.position_bundle),
        (
            *accounts.position_bundle_token_account.key,
            keys.position_bundle_token_account,
        ),
        (*accounts.position_bundle_authority.key, keys.position_bundle_authority),
        (*accounts.whirlpool.key, keys.whirlpool),
        (*accounts.funder.key, keys.funder),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.rent.key, keys.rent),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn open_bundled_position_verify_writable_privileges<'me, 'info>(
    accounts: OpenBundledPositionAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.bundled_position,
        accounts.position_bundle,
        accounts.funder,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn open_bundled_position_verify_signer_privileges<'me, 'info>(
    accounts: OpenBundledPositionAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.position_bundle_authority, accounts.funder] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn open_bundled_position_verify_account_privileges<'me, 'info>(
    accounts: OpenBundledPositionAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    open_bundled_position_verify_writable_privileges(accounts)?;
    open_bundled_position_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CLOSE_BUNDLED_POSITION_IX_ACCOUNTS_LEN: usize = 5;
#[derive(Copy, Clone, Debug)]
pub struct CloseBundledPositionAccounts<'me, 'info> {
    pub bundled_position: &'me AccountInfo<'info>,
    pub position_bundle: &'me AccountInfo<'info>,
    pub position_bundle_token_account: &'me AccountInfo<'info>,
    pub position_bundle_authority: &'me AccountInfo<'info>,
    pub receiver: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CloseBundledPositionKeys {
    pub bundled_position: Pubkey,
    pub position_bundle: Pubkey,
    pub position_bundle_token_account: Pubkey,
    pub position_bundle_authority: Pubkey,
    pub receiver: Pubkey,
}
impl From<CloseBundledPositionAccounts<'_, '_>> for CloseBundledPositionKeys {
    fn from(accounts: CloseBundledPositionAccounts) -> Self {
        Self {
            bundled_position: *accounts.bundled_position.key,
            position_bundle: *accounts.position_bundle.key,
            position_bundle_token_account: *accounts.position_bundle_token_account.key,
            position_bundle_authority: *accounts.position_bundle_authority.key,
            receiver: *accounts.receiver.key,
        }
    }
}
impl From<CloseBundledPositionKeys>
for [AccountMeta; CLOSE_BUNDLED_POSITION_IX_ACCOUNTS_LEN] {
    fn from(keys: CloseBundledPositionKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.bundled_position,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.position_bundle,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.position_bundle_token_account,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.position_bundle_authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.receiver,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; CLOSE_BUNDLED_POSITION_IX_ACCOUNTS_LEN]>
for CloseBundledPositionKeys {
    fn from(pubkeys: [Pubkey; CLOSE_BUNDLED_POSITION_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            bundled_position: pubkeys[0],
            position_bundle: pubkeys[1],
            position_bundle_token_account: pubkeys[2],
            position_bundle_authority: pubkeys[3],
            receiver: pubkeys[4],
        }
    }
}
impl<'info> From<CloseBundledPositionAccounts<'_, 'info>>
for [AccountInfo<'info>; CLOSE_BUNDLED_POSITION_IX_ACCOUNTS_LEN] {
    fn from(accounts: CloseBundledPositionAccounts<'_, 'info>) -> Self {
        [
            accounts.bundled_position.clone(),
            accounts.position_bundle.clone(),
            accounts.position_bundle_token_account.clone(),
            accounts.position_bundle_authority.clone(),
            accounts.receiver.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CLOSE_BUNDLED_POSITION_IX_ACCOUNTS_LEN]>
for CloseBundledPositionAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; CLOSE_BUNDLED_POSITION_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            bundled_position: &arr[0],
            position_bundle: &arr[1],
            position_bundle_token_account: &arr[2],
            position_bundle_authority: &arr[3],
            receiver: &arr[4],
        }
    }
}
pub const CLOSE_BUNDLED_POSITION_IX_DISCM: [u8; 8] = [41, 36, 216, 245, 27, 85, 103, 67];
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CloseBundledPositionIxArgs {
    pub bundle_index: u16,
}
#[derive(Clone, Debug, PartialEq)]
pub struct CloseBundledPositionIxData(pub CloseBundledPositionIxArgs);
impl From<CloseBundledPositionIxArgs> for CloseBundledPositionIxData {
    fn from(args: CloseBundledPositionIxArgs) -> Self {
        Self(args)
    }
}
impl CloseBundledPositionIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CLOSE_BUNDLED_POSITION_IX_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        CLOSE_BUNDLED_POSITION_IX_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(CloseBundledPositionIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CLOSE_BUNDLED_POSITION_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn close_bundled_position_ix_with_program_id(
    program_id: Pubkey,
    keys: CloseBundledPositionKeys,
    args: CloseBundledPositionIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CLOSE_BUNDLED_POSITION_IX_ACCOUNTS_LEN] = keys.into();
    let data: CloseBundledPositionIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn close_bundled_position_ix(
    keys: CloseBundledPositionKeys,
    args: CloseBundledPositionIxArgs,
) -> std::io::Result<Instruction> {
    close_bundled_position_ix_with_program_id(crate::ID, keys, args)
}
pub fn close_bundled_position_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CloseBundledPositionAccounts<'_, '_>,
    args: CloseBundledPositionIxArgs,
) -> ProgramResult {
    let keys: CloseBundledPositionKeys = accounts.into();
    let ix = close_bundled_position_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn close_bundled_position_invoke(
    accounts: CloseBundledPositionAccounts<'_, '_>,
    args: CloseBundledPositionIxArgs,
) -> ProgramResult {
    close_bundled_position_invoke_with_program_id(crate::ID, accounts, args)
}
pub fn close_bundled_position_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CloseBundledPositionAccounts<'_, '_>,
    args: CloseBundledPositionIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CloseBundledPositionKeys = accounts.into();
    let ix = close_bundled_position_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn close_bundled_position_invoke_signed(
    accounts: CloseBundledPositionAccounts<'_, '_>,
    args: CloseBundledPositionIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    close_bundled_position_invoke_signed_with_program_id(
        crate::ID,
        accounts,
        args,
        seeds,
    )
}
pub fn close_bundled_position_verify_account_keys(
    accounts: CloseBundledPositionAccounts<'_, '_>,
    keys: CloseBundledPositionKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.bundled_position.key, keys.bundled_position),
        (*accounts.position_bundle.key, keys.position_bundle),
        (
            *accounts.position_bundle_token_account.key,
            keys.position_bundle_token_account,
        ),
        (*accounts.position_bundle_authority.key, keys.position_bundle_authority),
        (*accounts.receiver.key, keys.receiver),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn close_bundled_position_verify_writable_privileges<'me, 'info>(
    accounts: CloseBundledPositionAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.bundled_position,
        accounts.position_bundle,
        accounts.receiver,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn close_bundled_position_verify_signer_privileges<'me, 'info>(
    accounts: CloseBundledPositionAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.position_bundle_authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn close_bundled_position_verify_account_privileges<'me, 'info>(
    accounts: CloseBundledPositionAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    close_bundled_position_verify_writable_privileges(accounts)?;
    close_bundled_position_verify_signer_privileges(accounts)?;
    Ok(())
}
