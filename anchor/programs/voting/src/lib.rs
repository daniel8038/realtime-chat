#![allow(clippy::result_large_err)]

use anchor_lang::prelude::*;

declare_id!("coUnmi3oBUtwtd9fjeAvSsJssXh5A5xyPbhpewyzRVF");

#[program]
pub mod voting {
    use super::*;
    pub fn initialize_poll(
        ctx: Context<InitializePoll>,
        poll_id: u64,
        poll_description: String,
        poll_start: u64,
        poll_end: u64,
    ) -> Result<()> {
        let poll = &mut ctx.accounts.poll;
        poll.poll_id = poll_id;
        poll.poll_description = poll_description;
        poll.poll_start = poll_start;
        poll.poll_end = poll_end;
        poll.poll_candidate_amount = 0;
        Ok(())
    }

    pub fn initialize_candidate(ctx: Context<InitializeCandidate>,poll_id: u64, candidate: String) -> Result<()> {
        Ok(())
    }
}
#[derive(Accounts)]
#[instruction(poll_id:u64)]
pub struct InitializePoll<'info> {
    // 创建新账户时(使用 init)必须包含 system_program
    // system_program 负责在 Solana 上创建新账户、分配空间和管理账户租金
    // 它就像是 Solana 的"账户管理员"
    pub system_program: Program<'info, System>,
    #[account(mut)]
    pub signer: Signer<'info>,
    // 8个字节是 Anchor 自动添加在账户数据开头的标识 账户区分标识符
    // Poll::INIT_SPACE 来自 #[derive(InitSpace)]，自动计算结构体需要的空间
    // 需要设置 space 因为 Solana 要预先知道账户大小以分配足够的空间
    // bump 是用来确保生成的 PDA 不在 ed25519 曲线上
    #[account(init,payer=signer,space = 8 + Poll::INIT_SPACE,seeds = [poll_id.to_le_bytes().as_ref()],bump)]
    pub poll: Account<'info, Poll>,
}
#[account]
#[derive(InitSpace)]
pub struct Poll {
    pub poll_id: u64,
    #[max_len(280)]
    pub poll_description: String,
    pub poll_start: u64,
    pub poll_end: u64,
    pub poll_candidate_amount: u64,
}
#[derive(Accounts)]
#[instruction(poll_id:u64,candidate:String)]
pub struct InitializeCandidate<'info> {
    pub system_program: Program<'info, System>,
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        init,
        payer = signer,
        space = 8 + CandidateAccount::INIT_SPACE, 
        seeds = [poll_id.to_le_bytes().as_ref(), candidate.as_ref()],
        bump)]
    pub candidate_account: Account<'info, CandidateAccount>,
}
#[account]
#[derive(InitSpace)]
pub struct CandidateAccount {
    #[max_len(32)]
    pub candidate_name: String,
    pub candidate_votes: u64,
}
