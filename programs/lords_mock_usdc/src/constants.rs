use anchor_lang::prelude::*;

pub const DEVNET_ADMIN_PUBKEY : Pubkey = pubkey!("HpAYk14jYpomivS4F7oXySN81sdoPvTaHtFsPgiK2jzf");
pub const MAX_MOCK_USDC_PER_TX: u64 = 30_000 * 1_000_000; // 10,000 USDC (6 decimals)