use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod robot_x {
    use super::*;

    pub fn create(ctx: Context<Create>) -> Result<()> {
        let action_state = &mut ctx.accounts.action_state;
        // * - means dereferencing
        action_state.user = *ctx.accounts.user.key;
        /*
        action_state.action.walk = false;
        action_state.action.run = false;
        action_state.action.jump = false;
        */
        action_state.action = 0;

        Ok(())
    }
    
    pub fn walk(ctx: Context<Walk>) -> Result<()> {
        let action_state = &mut ctx.accounts.action_state;
        // Lets change the robot action state to "walk"
        //action_state.action = Actions::Walk;
        //action_state.action.walk = true;
        action_state.action = 1;

        Ok(())
    }
    
    pub fn run(ctx: Context<Run>) -> Result<()> {
        let action_state = &mut ctx.accounts.action_state;
        // Lets change the robot action state to "run"
        //action_state.action = Actions::Run;
        //action_state.action.run = true;
        action_state.action = 2;

        Ok(())
    }
    
    pub fn jump(ctx: Context<Jump>) -> Result<()> {
        let action_state = &mut ctx.accounts.action_state;
        // Lets change the robot action state to "jump"
        //action_state.action = Actions::Jump;
        //action_state.action.jump = true;
        action_state.action = 3;
        
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Create<'info> {
    // init means to create action_state account
    // bump to use unique address for action_state account
    #[account(init, payer=user, space=ActionState::LEN, seeds=[b"action-state".as_ref(), user.key().as_ref()], bump)]
    pub action_state: Account<'info, ActionState>,
    // mut makes it changeble (mutable)
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Walk<'info> {
    // Only the user on account action_state, should be able to change state
    #[account(mut, has_one = user)]
    pub action_state: Account<'info, ActionState>,
    // mut makes it changeble (mutable)
    #[account(mut)]
    pub user: Signer<'info>,
    //pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Run<'info> {
    // Only the user on account action_state, should be able to change state
    #[account(mut, has_one = user)]
    pub action_state: Account<'info, ActionState>,
    // mut makes it changeble (mutable)
    #[account(mut)]
    pub user: Signer<'info>,
    //pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Jump<'info> {
    // Only the user on account action_state, should be able to change state
    #[account(mut, has_one = user)]
    pub action_state: Account<'info, ActionState>,
    // mut makes it changeble (mutable)
    #[account(mut)]
    pub user: Signer<'info>,
    //pub system_program: Program<'info, System>,
}

#[account]
pub struct ActionState {
    pub user: Pubkey,
    //pub action: Actions,
    // walk: 1,
	// run: 2,
	// jump: 3,
    pub action: u8,
}
/*
#[account]
pub struct Actions {
	walk: bool,
	run: bool,
	jump: bool,
}
*/
const DISCRIMINATOR_LENGTH: usize = 8;
const PUBLIC_KEY_LENGTH: usize = 32;
const U8_LENGTH: usize = 1;
//const ENUM_LENGTH: usize = 1; // 1 + Largest Variant Size

impl ActionState {
    const LEN: usize = DISCRIMINATOR_LENGTH +
                       PUBLIC_KEY_LENGTH +
                       U8_LENGTH;
}
/*
#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum Actions {
	Walk,
	Run,
	Jump,
}
*/