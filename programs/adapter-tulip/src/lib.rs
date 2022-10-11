use anchor_lang::prelude::*;
use anchor_lang::solana_program::{
    hash::hash,
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    program::invoke
};
use anchor_spl::token::{TokenAccount, Mint};

pub mod farms;
pub mod vaults;
pub mod common;
use crate::vaults::instructions::{new_withdraw_multi_deposit_optimizer_vault_ix, new_withdraw_deposit_tracking_ix, new_issue_shares_ix};
use crate::vaults::accounts::{vault_base::VaultBaseV1};

declare_id!("ADPT9nhC1asRcEB13FKymLTatqWGCuZHDznGgnakWKxW");

#[program]
pub mod adapter_tulip {
    use super::*;

    /// deposits `amount` of the underlying tokens in exchange for a corresponding
    /// amount of shares. these shares are locked within the deposit tracking account
    /// for 15 minutes, after which they can be removed from the deposit tracking account
    /// if desired. generaly speaking this should only be done if you want to
    /// use the tokenized shares elsewhere (ie friktion volts), otherwise
    /// its best to leave them within the deposit tracking account otherwise
    /// so that you can measure your accrued rewards automatically.
    pub fn deposit<'a, 'b, 'c, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, Action<'info>>,
    ) -> Result<()> {
        // Deriving Keys
        let authority = ctx.remaining_accounts[0].clone();
        let vault = ctx.remaining_accounts[1].clone();
        let deposit_tracking_account = ctx.remaining_accounts[2].clone();
        let deposit_tracking_hold_account = ctx.remaining_accounts[3].clone();
        let shares_mint = ctx.remaining_accounts[4].clone();
        let deposit_tracking_pda = ctx.remaining_accounts[5].clone();
        let vault_pda = ctx.remaining_accounts[6].clone();
        let vault_underlying_account = ctx.remaining_accounts[7].clone();
        let receiving_shares_account = ctx.remaining_accounts[3].clone();
        let depositing_underlying_account = ctx.remaining_accounts[8].clone();

        let mut share_token_account = Account::<TokenAccount>::try_from(&deposit_tracking_hold_account)?;
        let share_token_amount_before = share_token_account.amount;

        // let farm = Farm::Raydium { name: RAYSRM };
        let farm_key = {
            // let loader: AccountLoader<RaydiumVaultV1> = AccountLoader::try_from_unchecked(
            //     &ctx.accounts.base_program_id.key(),
            //     &vault,
            // )?;
            // {
            //     let vault = loader.load()?;
            //     vault.base.farm
            // }
            let mut vault_data = &**vault.try_borrow_data().unwrap();
            let vault_account = VaultBaseV1::deserialize(&mut vault_data).unwrap();
            vault_account.farm
        };

        // Deserialize gateway_state
        let gateway_state = get_gateway_state(&ctx.accounts.gateway_state_info);
        let current_index = gateway_state.current_index;
        let amount = gateway_state.payload_queue[current_index as usize] as u64;

        msg!("amount: {}", &amount.to_string());

        /*
            if this error is returned, it means the depositing_underlying_account
            has less tokens (X) then requested deposit amount (Y)
            Program log: RUNTIME ERROR: a(X) < b(Y)
            Program log: panicked at 'RUNTIME ERROR: a(0) < b(1)', programs/vaults/src/vault_instructions/deposit_tracking/acl_helpers.rs:198:9
        */
        let ix = new_issue_shares_ix(
            authority.key(),
            vault.key(),
            deposit_tracking_account.key(),
            deposit_tracking_pda.key(),
            vault_pda.key(),
            vault_underlying_account.key(),
            shares_mint.key(),
            receiving_shares_account.key(),
            depositing_underlying_account.key(),
            // farm,
            farm_key.into(),
            amount,
            ctx.accounts.base_program_id.key(),
        );
        anchor_lang::solana_program::program::invoke(
            &ix,
            &[
                authority.clone(),
                vault.clone(),
                deposit_tracking_account.clone(),
                deposit_tracking_pda.clone(),
                vault_pda.clone(),
                vault_underlying_account.to_account_info(),
                shares_mint.to_account_info(),
                receiving_shares_account.to_account_info(),
                depositing_underlying_account.to_account_info(),
            ],
        )?;

        share_token_account.reload()?;
        let share_token_amount_after = share_token_account.amount;
        let share_amount = share_token_amount_after - share_token_amount_before;

        // Return Result
        let result = DepositResultWrapper {
            share_amount,
        };
        let mut buffer: Vec<u8> = Vec::new();
        result.serialize(&mut buffer).unwrap();

        anchor_lang::solana_program::program::set_return_data(&buffer);

        Ok(())
    }

    /// withdraws `amount` of shares from the deposit tracking account into the `receiving_shares_account`.
    /// these withdrawn shares still accrue rewards, the rewards accrued are no longer tracked by the deposit
    /// tracking account
    pub fn withdraw<'a, 'b, 'c, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, Action<'info>>,
    ) -> Result<()> {
        // Deriving Keys
        let authority = ctx.remaining_accounts[0].clone();
        let vault = ctx.remaining_accounts[1].clone();
        let receiving_shares_account = ctx.remaining_accounts[12].clone();
        let lp_token_account_info = ctx.remaining_accounts[13].clone();
        let shares_mint = ctx.remaining_accounts[14].clone();
        let clock = ctx.remaining_accounts[15].clone();
        let deposit_tracking_account = ctx.remaining_accounts[20].clone();
        let deposit_tracking_pda = ctx.remaining_accounts[21].clone();
        let deposit_tracking_hold_account = ctx.remaining_accounts[22].clone();

        let mut lp_token_account = Account::<TokenAccount>::try_from(&lp_token_account_info)?;
        let lp_token_amount_before = lp_token_account.amount;

        // Deserialize gateway_state
        let gateway_state = get_gateway_state(&ctx.accounts.gateway_state_info);
        let current_index = gateway_state.current_index;
        let amount = gateway_state.payload_queue[current_index as usize] as u64;

        msg!("amount: {}", &amount.to_string());

        // let farm = Farm::Raydium { name: RAYSRM };
        let farm_key = {
            // let loader: AccountLoader<RaydiumVaultV1> = AccountLoader::try_from_unchecked(
            //     &ctx.accounts.base_program_id.key(),
            //     &vault,
            // )?;
            // {
            //     let vault = loader.load()?;
            //     vault.base.farm
            // }
            let mut vault_data = &**vault.try_borrow_data().unwrap();
            let vault_account = VaultBaseV1::deserialize(&mut vault_data).unwrap();
            vault_account.farm
        };

        let ix = new_withdraw_deposit_tracking_ix(
            authority.key(),
            deposit_tracking_account.key(),
            deposit_tracking_pda.key(),
            deposit_tracking_hold_account.key(),
            receiving_shares_account.key(),
            shares_mint.key(),
            vault.key(),
            farm_key.into(),
            // farm,
            amount,
            ctx.accounts.base_program_id.key(),
        );
        anchor_lang::solana_program::program::invoke(
             &ix,
             &[
                 authority.clone(),
                 clock.to_account_info(),
                 deposit_tracking_account.clone(),
                 deposit_tracking_pda.clone(),
                 deposit_tracking_hold_account.to_account_info(),
                 receiving_shares_account.to_account_info(),
                 shares_mint.to_account_info(),
                 vault.clone(),
             ],
        )?;

        let sighash_arr = sighash("global", "withdraw_raydium_vault");

        let withdraw_raydium_accounts = vec![
            AccountMeta::new(ctx.remaining_accounts[0].key(), true),
            AccountMeta::new(ctx.remaining_accounts[1].key(), false),
            AccountMeta::new(ctx.remaining_accounts[2].key(), false),
            AccountMeta::new(ctx.remaining_accounts[3].key(), false),
            AccountMeta::new(ctx.remaining_accounts[4].key(), false),
            AccountMeta::new(ctx.remaining_accounts[5].key(), false),
            AccountMeta::new(ctx.remaining_accounts[6].key(), false),
            AccountMeta::new(ctx.remaining_accounts[7].key(), false),
            AccountMeta::new(ctx.remaining_accounts[8].key(), false),
            AccountMeta::new(ctx.remaining_accounts[9].key(), false),
            AccountMeta::new(ctx.remaining_accounts[10].key(), false),
            AccountMeta::new(ctx.remaining_accounts[11].key(), false),
            AccountMeta::new(ctx.remaining_accounts[12].key(), false),
            AccountMeta::new(ctx.remaining_accounts[13].key(), false),
            AccountMeta::new(ctx.remaining_accounts[14].key(), false),
            AccountMeta::new_readonly(ctx.remaining_accounts[15].key(), false),
            AccountMeta::new_readonly(ctx.remaining_accounts[16].key(), false),
            AccountMeta::new_readonly(ctx.remaining_accounts[17].key(), false),
            AccountMeta::new(ctx.remaining_accounts[18].key(), false),
            AccountMeta::new(ctx.remaining_accounts[19].key(), false),
        ];

        let mut withdraw_raydium_data = vec![];
        withdraw_raydium_data.append(&mut sighash_arr.try_to_vec()?);
        withdraw_raydium_data.append(&mut amount.try_to_vec()?);

        let withdraw_raydium_ix = Instruction {
            program_id: ctx.accounts.base_program_id.key(),
            accounts: withdraw_raydium_accounts,
            data: withdraw_raydium_data,
        };

        anchor_lang::solana_program::program::invoke(
            &withdraw_raydium_ix,
            &ctx.remaining_accounts[0..20],
        )?;

        lp_token_account.reload()?;
        let lp_token_amount_after = lp_token_account.amount;
        let lp_amount = lp_token_amount_after - lp_token_amount_before;

        msg!("lp_amount: {}", lp_amount.to_string());

        // Return Result
        let result = WithdrawResultWrapper {
            lp_amount,
        };
        let mut buffer: Vec<u8> = Vec::new();
        result.serialize(&mut buffer).unwrap();

        anchor_lang::solana_program::program::set_return_data(&buffer);

        Ok(())
    }

    /// burns/redeems the `amount` of shares for their corresponding amount
    /// of underlying asset, using the mango standalone vault as the source of funds to withdraw from
    pub fn withdraw_multi_deposit_vault_through_mango(
        ctx: Context<WithdrawMangoMultiDepositOptimizerVault>,
        amount: u64,
    ) -> Result<()> {
        let standalone_vault_accounts = vec![
            AccountMeta::new_readonly(ctx.accounts.mango_group_account.key(), false),
            AccountMeta::new(ctx.accounts.withdraw_vault_mango_account.key(), false),
            AccountMeta::new_readonly(ctx.accounts.mango_cache.key(), false),
            AccountMeta::new_readonly(ctx.accounts.mango_root_bank.key(), false),
            AccountMeta::new(ctx.accounts.mango_node_bank.key(), false),
            AccountMeta::new(ctx.accounts.mango_token_account.key(), false),
            AccountMeta::new_readonly(ctx.accounts.mango_group_signer.key(), false),
            AccountMeta::new_readonly(ctx.accounts.system_program.key(), false),
        ];
        let ix = new_withdraw_multi_deposit_optimizer_vault_ix(
            ctx.accounts.common_data.authority.key(),
            ctx.accounts.common_data.multi_vault.key(),
            ctx.accounts.common_data.multi_vault_pda.key(),
            ctx.accounts.common_data.withdraw_vault.key(),
            ctx.accounts.common_data.withdraw_vault_pda.key(),
            ctx.accounts.common_data.platform_information.key(),
            ctx.accounts.common_data.platform_config_data.key(),
            ctx.accounts.common_data.lending_program.key(),
            ctx.accounts.common_data.multi_burning_shares_token_account.key(),
            ctx.accounts.common_data.withdraw_burning_shares_token_account.key(),
            ctx.accounts.common_data.receiving_underlying_token_account.key(),
            ctx.accounts.common_data.multi_underlying_withdraw_queue.key(),
            ctx.accounts.common_data.multi_shares_mint.key(),
            ctx.accounts.common_data.withdraw_shares_mint.key(),
            ctx.accounts.common_data.withdraw_vault_underlying_deposit_queue.key(),
            amount,
            standalone_vault_accounts.clone()
        );
        anchor_lang::solana_program::program::invoke(
            &ix,
            &[
                ctx.accounts.common_data.authority.clone(),
                ctx.accounts.common_data.multi_vault.clone(),
                ctx.accounts.common_data.multi_vault_pda.clone(),
                ctx.accounts.common_data.withdraw_vault.clone(),
                ctx.accounts.common_data.withdraw_vault_pda.clone(),
                ctx.accounts.common_data.platform_information.clone(),
                ctx.accounts.common_data.platform_config_data.clone(),
                ctx.accounts.common_data.lending_program.clone(),
                ctx.accounts.common_data.multi_burning_shares_token_account.to_account_info(),
                ctx.accounts.common_data.withdraw_burning_shares_token_account.to_account_info(),
                ctx.accounts.common_data.receiving_underlying_token_account.to_account_info(),
                ctx.accounts.common_data.multi_underlying_withdraw_queue.to_account_info(),
                ctx.accounts.common_data.multi_shares_mint.to_account_info(),
                ctx.accounts.common_data.withdraw_shares_mint.to_account_info(),
                ctx.accounts.common_data.withdraw_vault_underlying_deposit_queue.to_account_info(),
                ctx.accounts.mango_group_account.clone(),
                ctx.accounts.withdraw_vault_mango_account.clone(),
                ctx.accounts.mango_cache.clone(),
                ctx.accounts.mango_root_bank.clone(),
                ctx.accounts.mango_node_bank.clone(),
                ctx.accounts.mango_token_account.to_account_info(),
                ctx.accounts.mango_group_signer.clone(),
                ctx.accounts.system_program.to_account_info(),
                ctx.accounts.common_data.clock.to_account_info(),
            ],
        )?;
        Ok(())
    }
    /// burns/redeems the `amount` of shares for their corresponding amount
    /// of underlying asset, using the solend standalone vault as the source of funds to withdraw from
    pub fn withdraw_multi_deposit_vault_through_solend(
        ctx: Context<WithdrawSolendMultiDepositOptimizerVault>,
        amount: u64,
    ) -> Result<()> {
        let standalone_vault_accounts = vec![
            AccountMeta::new_readonly(ctx.accounts.reserve_account.key(), false),
            AccountMeta::new(ctx.accounts.reserve_liquidity_supply.key(), false),
            AccountMeta::new_readonly(ctx.accounts.reserve_collateral_mint.key(), false),
            AccountMeta::new_readonly(ctx.accounts.lending_market_account.key(), false),
            AccountMeta::new_readonly(ctx.accounts.derived_lending_market_authority.key(), false),
            AccountMeta::new_readonly(ctx.accounts.reserve_pyth_price_account.key(), false),
            AccountMeta::new_readonly(ctx.accounts.reserve_switchboard_price_account.key(), false),
        ];
        let ix = new_withdraw_multi_deposit_optimizer_vault_ix(
            ctx.accounts.common_data.authority.key(),
            ctx.accounts.common_data.multi_vault.key(),
            ctx.accounts.common_data.multi_vault_pda.key(),
            ctx.accounts.common_data.withdraw_vault.key(),
            ctx.accounts.common_data.withdraw_vault_pda.key(),
            ctx.accounts.common_data.platform_information.key(),
            ctx.accounts.common_data.platform_config_data.key(),
            ctx.accounts.common_data.lending_program.key(),
            ctx.accounts.common_data.multi_burning_shares_token_account.key(),
            ctx.accounts.common_data.withdraw_burning_shares_token_account.key(),
            ctx.accounts.common_data.receiving_underlying_token_account.key(),
            ctx.accounts.common_data.multi_underlying_withdraw_queue.key(),
            ctx.accounts.common_data.multi_shares_mint.key(),
            ctx.accounts.common_data.withdraw_shares_mint.key(),
            ctx.accounts.common_data.withdraw_vault_underlying_deposit_queue.key(),
            amount,
            standalone_vault_accounts.clone()
        );
        anchor_lang::solana_program::program::invoke(
            &ix,
            &[
                ctx.accounts.common_data.authority.clone(),
                ctx.accounts.common_data.multi_vault.clone(),
                ctx.accounts.common_data.multi_vault_pda.clone(),
                ctx.accounts.common_data.withdraw_vault.clone(),
                ctx.accounts.common_data.withdraw_vault_pda.clone(),
                ctx.accounts.common_data.platform_information.clone(),
                ctx.accounts.common_data.platform_config_data.clone(),
                ctx.accounts.common_data.lending_program.clone(),
                ctx.accounts.common_data.multi_burning_shares_token_account.to_account_info(),
                ctx.accounts.common_data.withdraw_burning_shares_token_account.to_account_info(),
                ctx.accounts.common_data.receiving_underlying_token_account.to_account_info(),
                ctx.accounts.common_data.multi_underlying_withdraw_queue.to_account_info(),
                ctx.accounts.common_data.multi_shares_mint.to_account_info(),
                ctx.accounts.common_data.withdraw_shares_mint.to_account_info(),
                ctx.accounts.common_data.withdraw_vault_underlying_deposit_queue.to_account_info(),
                ctx.accounts.reserve_account.clone(),
                ctx.accounts.reserve_liquidity_supply.to_account_info(),
                ctx.accounts.reserve_collateral_mint.to_account_info(),
                ctx.accounts.lending_market_account.clone(),
                ctx.accounts.derived_lending_market_authority.clone(),
                ctx.accounts.reserve_pyth_price_account.to_account_info(),
                ctx.accounts.reserve_switchboard_price_account.clone(),
                ctx.accounts.common_data.clock.to_account_info(),
            ],
        )?;
        Ok(())
    }
    /// burns/redeems the `amount` of shares for their corresponding amount
    /// of underlying asset, using the tulip standalone vault as the source of funds to withdraw from
    pub fn withdraw_multi_deposit_vault_through_tulip(
        ctx: Context<WithdrawTulipMultiDepositOptimizerVault>,
        amount: u64,
    ) -> Result<()> {
        let standalone_vault_accounts = vec![
            AccountMeta::new_readonly(ctx.accounts.reserve_account.key(), false),
            AccountMeta::new(ctx.accounts.reserve_liquidity_supply.key(), false),
            AccountMeta::new_readonly(ctx.accounts.reserve_collateral_mint.key(), false),
            AccountMeta::new_readonly(ctx.accounts.lending_market_account.key(), false),
            AccountMeta::new_readonly(ctx.accounts.derived_lending_market_authority.key(), false),
            AccountMeta::new_readonly(ctx.accounts.reserve_pyth_price_account.key(), false),
        ];
        let ix = new_withdraw_multi_deposit_optimizer_vault_ix(
            ctx.accounts.common_data.authority.key(),
            ctx.accounts.common_data.multi_vault.key(),
            ctx.accounts.common_data.multi_vault_pda.key(),
            ctx.accounts.common_data.withdraw_vault.key(),
            ctx.accounts.common_data.withdraw_vault_pda.key(),
            ctx.accounts.common_data.platform_information.key(),
            ctx.accounts.common_data.platform_config_data.key(),
            ctx.accounts.common_data.lending_program.key(),
            ctx.accounts.common_data.multi_burning_shares_token_account.key(),
            ctx.accounts.common_data.withdraw_burning_shares_token_account.key(),
            ctx.accounts.common_data.receiving_underlying_token_account.key(),
            ctx.accounts.common_data.multi_underlying_withdraw_queue.key(),
            ctx.accounts.common_data.multi_shares_mint.key(),
            ctx.accounts.common_data.withdraw_shares_mint.key(),
            ctx.accounts.common_data.withdraw_vault_underlying_deposit_queue.key(),
            amount,
            standalone_vault_accounts.clone()
        );
        anchor_lang::solana_program::program::invoke(
            &ix,
            &[
                ctx.accounts.common_data.authority.clone(),
                ctx.accounts.common_data.multi_vault.clone(),
                ctx.accounts.common_data.multi_vault_pda.clone(),
                ctx.accounts.common_data.withdraw_vault.clone(),
                ctx.accounts.common_data.withdraw_vault_pda.clone(),
                ctx.accounts.common_data.platform_information.clone(),
                ctx.accounts.common_data.platform_config_data.clone(),
                ctx.accounts.common_data.lending_program.clone(),
                ctx.accounts.common_data.multi_burning_shares_token_account.to_account_info(),
                ctx.accounts.common_data.withdraw_burning_shares_token_account.to_account_info(),
                ctx.accounts.common_data.receiving_underlying_token_account.to_account_info(),
                ctx.accounts.common_data.multi_underlying_withdraw_queue.to_account_info(),
                ctx.accounts.common_data.multi_shares_mint.to_account_info(),
                ctx.accounts.common_data.withdraw_shares_mint.to_account_info(),
                ctx.accounts.common_data.withdraw_vault_underlying_deposit_queue.to_account_info(),
                ctx.accounts.reserve_account.clone(),
                ctx.accounts.reserve_liquidity_supply.to_account_info(),
                ctx.accounts.reserve_collateral_mint.to_account_info(),
                ctx.accounts.lending_market_account.clone(),
                ctx.accounts.derived_lending_market_authority.clone(),
                ctx.accounts.reserve_pyth_price_account.to_account_info(),
                ctx.accounts.common_data.clock.to_account_info(),
            ],
        )?;
        Ok(())
    }

    pub fn supply<'a, 'b, 'c, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, Action<'info>>,
    ) -> Result<()> {
        let supply_ix: u8 = 4; // DepositReserveLiquidity

        // Deserialize gateway_state
        let gateway_state = get_gateway_state(&ctx.accounts.gateway_state_info);
        let current_index = gateway_state.current_index;

        // Get the data from payload queue
        let supply_amount = gateway_state.payload_queue[current_index as usize];

        msg!("supply_amount: {}", supply_amount.to_string());

        let add_supply_accounts = vec![
            AccountMeta::new(ctx.remaining_accounts[0].key(), false),
            AccountMeta::new(ctx.remaining_accounts[1].key(), false),
            AccountMeta::new(ctx.remaining_accounts[2].key(), false),
            AccountMeta::new(ctx.remaining_accounts[3].key(), false),
            AccountMeta::new(ctx.remaining_accounts[4].key(), false),
            AccountMeta::new_readonly(ctx.remaining_accounts[5].key(), false),
            AccountMeta::new_readonly(ctx.remaining_accounts[6].key(), false),
            AccountMeta::new_readonly(ctx.remaining_accounts[7].key(), true),
            AccountMeta::new_readonly(ctx.remaining_accounts[8].key(), false),
            AccountMeta::new_readonly(ctx.remaining_accounts[9].key(), false),
        ];

        let mut add_supply_data = vec![];
        add_supply_data.append(&mut supply_ix.try_to_vec()?);
        add_supply_data.append(&mut supply_amount.try_to_vec()?);

        let ix = Instruction {
            program_id: ctx.accounts.base_program_id.key(),
            accounts: add_supply_accounts, 
            data: add_supply_data
        };

        invoke(
            &ix, 
            ctx.remaining_accounts
        )?;

        Ok(())
    }

    pub fn unsupply<'a, 'b, 'c, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, Action<'info>>,
    ) -> Result<()> {
        let unsupply_ix: u8 = 5; // RedeemReserveCollateral

        // Deserialize gateway_state
        let gateway_state = get_gateway_state(&ctx.accounts.gateway_state_info);
        let current_index = gateway_state.current_index;

        // Get the data from payload queue
        let unsupply_amount = gateway_state.payload_queue[current_index as usize];

        msg!("unsupply_amount: {}", unsupply_amount.to_string());

        let remove_supply_accounts = vec![
            AccountMeta::new(ctx.remaining_accounts[0].key(), false),
            AccountMeta::new(ctx.remaining_accounts[1].key(), false),
            AccountMeta::new(ctx.remaining_accounts[2].key(), false),
            AccountMeta::new(ctx.remaining_accounts[3].key(), false),
            AccountMeta::new(ctx.remaining_accounts[4].key(), false),
            AccountMeta::new_readonly(ctx.remaining_accounts[5].key(), false),
            AccountMeta::new_readonly(ctx.remaining_accounts[6].key(), false),
            AccountMeta::new_readonly(ctx.remaining_accounts[7].key(), true),
            AccountMeta::new_readonly(ctx.remaining_accounts[8].key(), false),
            AccountMeta::new_readonly(ctx.remaining_accounts[9].key(), false),
        ];

        let mut remove_supply_data = vec![];
        remove_supply_data.append(&mut unsupply_ix.try_to_vec()?);
        remove_supply_data.append(&mut unsupply_amount.try_to_vec()?);

        let ix = Instruction {
            program_id: ctx.accounts.base_program_id.key(),
            accounts: remove_supply_accounts, 
            data: remove_supply_data
        };

        invoke(
            &ix, 
            ctx.remaining_accounts
        )?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct RegisterDepositTrackingAccount<'info> {
    /// CHECK: Safe
    #[account(mut, signer)]
    pub authority: AccountInfo<'info>,
    /// CHECK: Safe
    pub vault: AccountInfo<'info>,
    /// CHECK: Safe
    #[account(mut)]
    pub deposit_tracking_account: AccountInfo<'info>,
    /// CHECK: Safe
    #[account(mut)]
    pub deposit_tracking_queue_account: AccountInfo<'info>,
    /// CHECK: Safe
    #[account(mut)]
    pub deposit_tracking_hold_account: AccountInfo<'info>,
    #[account(mut)]
    pub shares_mint: Box<Account<'info, Mint>>,
    #[account(mut)]
    pub underlying_mint: Box<Account<'info, Mint>>,
    /// CHECK: Safe
    #[account(mut)]
    pub deposit_tracking_pda: AccountInfo<'info>,
    /// CHECK: Safe
    pub token_program: AccountInfo<'info>,
    /// CHECK: Safe
    pub associated_token_program: AccountInfo<'info>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
    /// CHECK: Safe
    pub vault_program: AccountInfo<'info>,
}


#[derive(Accounts)]
pub struct IssueShares<'info> {
    /// CHECK: Safe
    #[account(signer)]
    pub authority: AccountInfo<'info>,
    /// CHECK: Safe
    #[account(mut)]
    pub vault: AccountInfo<'info>,
    /// CHECK: Safe
    #[account(mut)]
    pub deposit_tracking_account: AccountInfo<'info>,
    /// CHECK: Safe
    #[account(mut)]
    pub deposit_tracking_pda: AccountInfo<'info>,
    /// CHECK: Safe
    pub vault_pda: AccountInfo<'info>,
    #[account(mut)]
    pub shares_mint: Box<Account<'info, Mint>>,
    #[account(mut)]
    /// the account which will receive the issued shares
    /// this is the deposit_tracking_hold_account
    pub receiving_shares_account: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    /// the account owned by the authority which contains the underlying tokens
    /// we want to deposit in exchange for the vault shares
    pub depositing_underlying_account: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    /// the underlying token account that is owned by the vault pda
    /// which holds the underlying tokens until they are swept into the farm.
    ///
    /// also known as the deposit queue account
    pub vault_underlying_account: Box<Account<'info, TokenAccount>>,
    pub system_program: Program<'info, System>,
    /// CHECK: Safe
    pub vault_program: AccountInfo<'info>,
    /// CHECK: Safe
    pub token_program: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct WithdrawDepositTrackingAccount<'info> {
    /// CHECK: Safe
    #[account(signer)]
    pub authority: AccountInfo<'info>,
    /// CHECK: Safe
    #[account(mut)]
    pub deposit_tracking_account: AccountInfo<'info>,
    /// CHECK: Safe
    #[account(mut)]
    pub deposit_tracking_pda: AccountInfo<'info>,
    /// CHECK: Safe
    #[account(mut)]
    pub deposit_tracking_hold_account: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    /// this is the token account owned by the authority that
    /// should receive the tokenized shares which are being removed
    /// from the deposit tracking account. do note that this means
    /// these shares are no longer being tracked by the deposit tracking
    /// account, and any newly accrued rewards tracked by the deposit tracking
    /// account will reflect the remaining balance that hasn't been withdrawn
    /// 
    /// **the shares that are being withdrawn still accrue rewards the same as shares that are held by the deposit tracking account**
    pub receiving_shares_account: Box<Account<'info, TokenAccount>>,
    /// CHECK: Safe
    pub shares_mint: AccountInfo<'info>,
    /// CHECK: Safe
    #[account(mut)]
    pub vault: AccountInfo<'info>,
    pub clock: Sysvar<'info, Clock>,
    /// CHECK: Safe
    pub vault_program: AccountInfo<'info>,
    /// CHECK: Safe
    pub token_program: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct WithdrawMultiDepositOptimizerVault<'info> {
    /// CHECK: Safe
    #[account(signer)]
    pub authority: AccountInfo<'info>,
    /// CHECK: Safe
    #[account(mut)]
    pub multi_vault: AccountInfo<'info>,
    /// CHECK: Safe
    pub multi_vault_pda: AccountInfo<'info>,
    /// CHECK: Safe
    #[account(mut)]
    pub withdraw_vault: AccountInfo<'info>,
    /// CHECK: Safe
    pub withdraw_vault_pda: AccountInfo<'info>,
    /// CHECK: Safe
    pub platform_information: AccountInfo<'info>,
    /// CHECK: Safe
    pub platform_config_data: AccountInfo<'info>,
    #[account(mut)]
    /// this is the token account owned by the authority for the multi vault
    /// shares mint, which are the tokens we are burning/redeeming in exchange
    /// for the underlying asset
    pub multi_burning_shares_token_account: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    /// this is the account owned by the multi vault pda that holds the tokenized
    /// shares issued by the withdraw vault. 
    pub withdraw_burning_shares_token_account: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    /// this is the account owned by the authority which will receive the underlying
    pub receiving_underlying_token_account: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    /// this is the underlying token account owned by the multi deposit vault
    /// which is used to temporarily store tokens during the token withdraw process
    pub multi_underlying_withdraw_queue: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    pub multi_shares_mint: Box<Account<'info, Mint>>,
    #[account(mut)]
    pub withdraw_shares_mint: Box<Account<'info, Mint>>,
    #[account(mut)]
    /// this is the underlying token account owned by the withdraw vault we are
    /// removing underlying assets from
    pub withdraw_vault_underlying_deposit_queue: Box<Account<'info, TokenAccount>>,
    pub clock: Sysvar<'info, Clock>,
    /// CHECK: Safe
    pub token_program: AccountInfo<'info>,
    /// CHECK: Safe
    pub lending_program: AccountInfo<'info>,
    /// CHECK: Safe
    pub vault_program: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct WithdrawMangoMultiDepositOptimizerVault<'info> {
    /// configuration data common to all multi deposit withdraw instructions
    /// regardless of the underlying vault htey are withdrawing from
    pub common_data: WithdrawMultiDepositOptimizerVault<'info>,
    /// CHECK: Safe
    pub mango_group_account: AccountInfo<'info>,
    /// CHECK: Safe
    #[account(mut)]
    pub withdraw_vault_mango_account: AccountInfo<'info>,
    /// CHECK: Safe
    pub mango_cache: AccountInfo<'info>,
    /// CHECK: Safe
    pub mango_root_bank: AccountInfo<'info>,
    /// CHECK: Safe
    #[account(mut)]
    pub mango_node_bank: AccountInfo<'info>,
    #[account(mut)]
    pub mango_token_account: Box<Account<'info, TokenAccount>>,
    /// CHECK: Safe
    pub mango_group_signer: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct WithdrawSolendMultiDepositOptimizerVault<'info> {
    /// configuration data common to all multi deposit withdraw instructions
    /// regardless of the underlying vault htey are withdrawing from
    pub common_data: WithdrawMultiDepositOptimizerVault<'info>,
    /// CHECK: Safe
    #[account(mut)]
    pub reserve_account: AccountInfo<'info>,
    #[account(mut)]
    pub reserve_liquidity_supply: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    pub reserve_collateral_mint: Box<Account<'info, Mint>>,
    /// CHECK: Safe
    pub lending_market_account: AccountInfo<'info>,
    /// CHECK: Safe
    pub derived_lending_market_authority: AccountInfo<'info>,
    /// CHECK: Safe
    pub reserve_pyth_price_account: AccountInfo<'info>,
    /// CHECK: Safe
    pub reserve_switchboard_price_account: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct WithdrawTulipMultiDepositOptimizerVault<'info> {
    /// configuration data common to all multi deposit withdraw instructions
    /// regardless of the underlying vault htey are withdrawing from
    pub common_data: WithdrawMultiDepositOptimizerVault<'info>,
    /// CHECK: Safe
    #[account(mut)]
    pub reserve_account: AccountInfo<'info>,
    #[account(mut)]
    pub reserve_liquidity_supply: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    pub reserve_collateral_mint: Box<Account<'info, Mint>>,
    /// CHECK: Safe
    pub lending_market_account: AccountInfo<'info>,
    /// CHECK: Safe
    pub derived_lending_market_authority: AccountInfo<'info>,
    /// CHECK: Safe
    pub reserve_pyth_price_account: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct WithdrawRaydium<'info> {
    /// CHECK: Safe
    pub vault_program: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct Action<'info> {
    pub gateway_authority: Signer<'info>,
    /// CHECK: Safe
    pub gateway_state_info: AccountInfo<'info>,
    /// CHECK: Safe
    pub base_program_id: AccountInfo<'info>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct GatewayStateWrapper {
    pub discriminator: u64,
    pub user_key: Pubkey,
    pub random_seed: u64,
    pub version: u8,
    pub current_index: u8, // Start from 0
    pub queue_size: u8,

    // Queues
    pub protocol_queue: [u8; 8],
    pub action_queue: [u8; 8],
    pub version_queue: [u8; 8],
    pub payload_queue: [u64; 8],

    // Extra metadata
    pub swap_min_out_amount: u64,
    pub pool_direction: u8,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct DepositResultWrapper {
    pub share_amount: u64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct WithdrawResultWrapper {
    pub lp_amount: u64,
}

fn get_gateway_state(gateway_state_info: &AccountInfo) -> GatewayStateWrapper {
    let mut gateway_state_data = &**gateway_state_info.try_borrow_data().unwrap();
    GatewayStateWrapper::deserialize(&mut gateway_state_data).unwrap()
}

pub fn sighash(namespace: &str, name: &str) -> [u8; 8] {
    let preimage = format!("{}:{}", namespace, name);
    let mut sighash = [0u8; 8];

    sighash.copy_from_slice(&hash(preimage.as_bytes()).to_bytes()[..8]);
    sighash
}