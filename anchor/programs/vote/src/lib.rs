use anchor_lang::prelude::*;

pub mod constants;
pub mod errors;
pub mod instructions;
pub mod states;

#[allow(ambiguous_glob_reexports)]
pub use instructions::*;
pub use constants::ANCHOR_DISCRIMINATOR_SIZE;
pub use errors::ErrorCode::*;
pub use states::*;

declare_id!("4vinHdXKw3LhA9iZqWHL9SZ8kYHENh6ZhKDzKdpTPsDm");


#[program]
pub mod vote {

    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        instructions::initialize(ctx)
      }

    pub fn create_poll(
        ctx: Context<CreatePoll>,
        description: String,
        start: u64,
        end: u64,
    ) -> Result<()> {
        instructions::create_poll(ctx, description, start, end)
    }

    pub fn register_candidates(
        ctx: Context<RegisterCandidate>,
        poll_id: u64,
        name: String,
    ) -> Result<()> {
        instructions::register_candidates(ctx, poll_id, name)
    }

    pub fn voting(
        ctx: Context<VoteCandidate>,
        poll_id: u64,
        cid: u64,
    ) -> Result<()> {
        instructions::voting(ctx, poll_id, cid)
    }
}



