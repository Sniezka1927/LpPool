#![allow(non_snake_case)]
use std::time::SystemTime;
use rust_decimal::prelude::*;
use rust_decimal_macros::dec;

// Define the Token and StakedToken structures
#[derive(Debug, Copy, Clone)]
struct TokenAmount(Decimal);
#[derive(Debug, Copy, Clone)]
struct StakedTokenAmount(Decimal);
#[derive(Debug, Copy, Clone)]
struct LPTokenAmount(Decimal);
#[derive(Debug, Copy, Clone)]
struct Price(Decimal);
#[derive(Debug, Copy, Clone)]
struct Percentage(Decimal);
#[derive(Debug)]
enum RemovalResult {
    Success(LPTokenAmount, StakedTokenAmount),
    Error(String)
}
#[derive(Debug)]
enum SwapResult {
    Success(TokenAmount),
    Error(String)
}
#[derive(Debug)]
enum AddLiquidityResult {
    Success(LPTokenAmount),
    Error(String)
}

// Define the LpPool structure
#[derive(Debug)]
struct LpPool {
    price: Price,
    token_amount: TokenAmount,
    st_token_amount: StakedTokenAmount,
    lp_token_amount: LPTokenAmount,
    min_fee: Percentage,
    max_fee: Percentage,
}

impl From<TokenAmount> for Decimal {
    fn from(amount: TokenAmount) -> Self {
        amount.0
    }
}

impl From<StakedTokenAmount> for Decimal {
    fn from(amount: StakedTokenAmount) -> Self {
        amount.0
    }
}

// TODO

impl LpPool {
    fn init(price: Price, min_fee: Percentage, max_fee: Percentage, token_amount: TokenAmount) -> Self {
        LpPool {
            price,
            token_amount,
            st_token_amount: StakedTokenAmount(dec!(0)),
            lp_token_amount: LPTokenAmount(dec!(0)),
            min_fee,
            max_fee,
        }
    }
     
    fn add_liquidity(&mut self, tokens_to_add: TokenAmount) -> AddLiquidityResult {
        // ADD CHARGIN FEE WHEN TOKEN RESERVE IS BELOW LIQUIDITY POOL

        if self.is_positive(tokens_to_add.into()) {
        let tokens_to_add_fixed: Decimal = self.scale_up(tokens_to_add.into());
        let current_lp_tokens_fixed: Decimal = self.scale_up(self.lp_token_amount.0);
        let updated_lp_tokens_fixed: Decimal = tokens_to_add_fixed + current_lp_tokens_fixed;
        let new_liquidity_pool: Decimal = self.scale_down(updated_lp_tokens_fixed);

        let current_tokens_amount_fixed: Decimal = self.scale_up(self.token_amount.0);
        let updated_tokens_amount_fixed: Decimal = tokens_to_add_fixed + current_tokens_amount_fixed;
        let new_token_amount: Decimal = self.scale_down(updated_tokens_amount_fixed);
        
        self.lp_token_amount.0 = new_liquidity_pool;
        self.token_amount.0 = new_token_amount;

        return AddLiquidityResult::Success(LPTokenAmount(self.lp_token_amount.0))
        } else {
            return AddLiquidityResult::Error("You can only add positive value to the liquidity pool".to_string())
        }
    }

    fn remove_liquidity(&mut self, tokens_to_remove: TokenAmount) -> RemovalResult {
        if self.is_positive(tokens_to_remove.into()) {
            let fee_charged: Decimal = self.calculate_fee(tokens_to_remove.into());
            // println!("Fee charged while removing liquidity: {}", fee_charged);
            let recieved_tokens_to_remove: Decimal = tokens_to_remove.into();
            let charged_tokens_to_remove: Decimal = recieved_tokens_to_remove - fee_charged;
            let mut tokens_to_remove_fixed: Decimal = self.scale_up(charged_tokens_to_remove);
            let current_staked_tokens_fixed: Decimal = self.scale_up(self.st_token_amount.0);
    
            if current_staked_tokens_fixed > tokens_to_remove_fixed {
                // println!("There are enough tokens in the stacked pool to remove");
                let updated_stacked_tokens_fixed: Decimal = current_staked_tokens_fixed - tokens_to_remove_fixed;
                let removed_tokens: Decimal = self.scale_down(tokens_to_remove_fixed);
                let updated_stacked_pool: Decimal = self.scale_down(updated_stacked_tokens_fixed);
                self.st_token_amount.0 = updated_stacked_pool;
                return RemovalResult::Success(LPTokenAmount(dec!(0)), StakedTokenAmount(removed_tokens))
            } else {
                let staked_tokens_removed: Decimal = self.scale_down(current_staked_tokens_fixed);
                tokens_to_remove_fixed -= current_staked_tokens_fixed;
                let current_tokens_amount_fixed: Decimal = self.scale_up(self.token_amount.0);
                if current_tokens_amount_fixed < tokens_to_remove_fixed {
                    return RemovalResult::Error("You can't extract more tokens than are in the tokens reserve!".to_string());    
                }
                let updated_tokens_amonunt_fixed: Decimal = current_tokens_amount_fixed - tokens_to_remove_fixed;
                let new_tokens_amount: Decimal = self.scale_down(updated_tokens_amonunt_fixed);
                let current_lp_tokens_fixed: Decimal = self.scale_up(self.lp_token_amount.0);
                if current_lp_tokens_fixed < tokens_to_remove_fixed {
                    return RemovalResult::Error("You can't extract more tokens than are in the liquidity pool!".to_string());
                }
                let updated_lp_tokens_fixed: Decimal = current_lp_tokens_fixed - tokens_to_remove_fixed;
                let removed_tokens: Decimal = self.scale_down(tokens_to_remove_fixed);
                let new_liquidity_pool: Decimal = self.scale_down(updated_lp_tokens_fixed);
                self.st_token_amount.0 = dec!(0);
                self.token_amount.0 = new_tokens_amount;
                self.lp_token_amount.0 = new_liquidity_pool;
                return RemovalResult::Success(LPTokenAmount(removed_tokens), StakedTokenAmount(staked_tokens_removed))
            }
        } else {
            return RemovalResult::Error("You can remove only positive amount of tokens from liquidity pool.".to_string());
        }
        
    }

    fn swap(&mut self, tokens_to_swap: StakedTokenAmount) -> SwapResult {

        // return fee
        if self.is_positive(tokens_to_swap.into()) {
        let tokens_to_swap: Decimal = tokens_to_swap.into();
        let tokens_to_stake_fixed: Decimal = self.scale_up(tokens_to_swap.into());
        let fee_charged: Decimal = self.calculate_fee(tokens_to_swap);

        println!("Fee charged: {:?}", fee_charged);
        let tokens_to_swap: Decimal = tokens_to_swap * self.price.0 - fee_charged;
        let tokens_to_swap_fixed: Decimal = self.scale_up(tokens_to_swap.into());

        self.calculate_fee(tokens_to_swap);
        let current_token_amount_fixed: Decimal = self.scale_up(self.token_amount.0);
        if tokens_to_swap_fixed > current_token_amount_fixed {
            return SwapResult::Error("There is no enough tokens in the reserve!".to_string());
        }
        let updated_token_amount_fixed: Decimal = current_token_amount_fixed - tokens_to_swap_fixed;
        let new_token_amount: Decimal = self.scale_down(updated_token_amount_fixed);

        let current_staked_tokens_fixed: Decimal = self.scale_up(self.st_token_amount.0);
        let updated_stacked_tokens_fixed: Decimal = tokens_to_stake_fixed + current_staked_tokens_fixed;
        let new_staked_pool: Decimal = self.scale_down(updated_stacked_tokens_fixed);
        
        self.st_token_amount.0 = new_staked_pool;
        self.token_amount.0 = new_token_amount;

        return SwapResult::Success(TokenAmount(tokens_to_swap))
    } else {
        return SwapResult::Error("You can swap only positive amount of tokens!".to_string());
    }
    }

    fn calculate_fee(&self, tokens_amount_to_charge_fee: Decimal) -> Decimal {
        let current_time: SystemTime = SystemTime::now();
        let since_epoch: std::time::Duration = current_time.duration_since(SystemTime::UNIX_EPOCH).unwrap();
        let mut seed: u128 = since_epoch.as_nanos();

        let min: Decimal = self.min_fee.0;
        let max: Decimal = self.max_fee.0;

        let fee_value: Decimal = self.xorshift(&mut seed) % (max - min + dec!(1)) + min;
        
        let fee_tokens_amount: Decimal = (tokens_amount_to_charge_fee * self.price.0) * fee_value / dec!(100);
        return fee_tokens_amount;
    }


    fn scale_up(&self, value: Decimal) -> Decimal {
        let multiplier: Decimal = dec!(1_000_000_000_000_000_000_000_000.0);
        let result: Decimal = (value) * multiplier;
        return result
    }

    fn scale_down(&self, value: Decimal) -> Decimal {
        let divisor: Decimal = dec!(1_000_000_000_000_000_000_000_000.0);
        let result: Decimal = value / divisor;
        return result
    }

    fn xorshift(&self, seed: &mut u128) -> Decimal {
        *seed ^= *seed << 13;
        *seed ^= *seed >> 7;
        *seed ^= *seed << 17;
        Decimal::from(*seed)
    }

    fn is_positive(&self, value: Decimal) -> bool {
        value > dec!(0)
    }
    
}

fn main() {
    let price: Price = Price(dec!(1.5)); // 1.0 in fixed-point
    let min_fee: Percentage = Percentage(dec!(0.001)); // 0.1% in fixed-point
    let max_fee: Percentage = Percentage(dec!(1.35)); // 9.0% in fixed-point
    let token_amount: TokenAmount = TokenAmount(dec!(90)); // 90.0 in fixed-point

    let mut lp_pool: LpPool = LpPool::init(price, min_fee, max_fee, token_amount);

    println!("Initialized!");
    println!("{:?}", lp_pool);

    println!("");
    println!("Starting Story Brief!");
    println!("");

    let minted_tokens: AddLiquidityResult =    lp_pool.add_liquidity(TokenAmount(dec!(100)));
    println!("Added 100 tokens to liquidity");
    println!("Minted data: {:?}", minted_tokens);
    println!("{:?}", lp_pool);
    println!("");

    let swapped_tokens: SwapResult = lp_pool.swap(StakedTokenAmount(dec!(6)));
    println!("Staked 6 tokens");   
    println!("Swapped data: {:?}", swapped_tokens);
    println!("{:?}", lp_pool);
    println!("");

    let minted_tokens_again: AddLiquidityResult = lp_pool.add_liquidity(TokenAmount(dec!(10)));
    println!("Added 10 tokens to liquidity");   
    println!("Minted data: {:?}", minted_tokens_again);
    println!("{:?}", lp_pool);
    println!("");

    let swapped_tokens_again: SwapResult = lp_pool.swap(StakedTokenAmount(dec!(30)));
    println!("Staked 30 tokens");   
    println!("Swappped data: {:?}", swapped_tokens_again);
    println!("{:?}", lp_pool);
    println!("");

    let removed_liquidity: RemovalResult = lp_pool.remove_liquidity(TokenAmount(dec!(100)));
    println!("Removed 100 tokens from liquidity");   
    println!("Removed data: {:?}", removed_liquidity);
    println!("{:?}", lp_pool);
    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_liquidity() {
        let price: Price = Price(dec!(1.5));
        let min_fee: Percentage = Percentage(dec!(0.001));
        let max_fee: Percentage = Percentage(dec!(1.35));
        let mut lp_pool: LpPool = LpPool::init(price, min_fee, max_fee, TokenAmount(dec!(90)));

        let added_liquidity: AddLiquidityResult = lp_pool.add_liquidity(TokenAmount(dec!(100)));
        assert!(matches!(added_liquidity, AddLiquidityResult::Success(_)));
    }

    #[test]
    fn test_swap() {
        let price: Price = Price(dec!(1.5));
        let min_fee: Percentage = Percentage(dec!(0.001));
        let max_fee: Percentage = Percentage(dec!(1.35));
        let mut lp_pool: LpPool = LpPool::init(price, min_fee, max_fee, TokenAmount(dec!(90)));

        let swapped_tokens: SwapResult = lp_pool.swap(StakedTokenAmount(dec!(6)));
        assert!(matches!(swapped_tokens, SwapResult::Success(_)));
    }
    #[test]
    fn test_remove_liquidity() {
        let price: Price = Price(dec!(1.5));
        let min_fee: Percentage = Percentage(dec!(0.001));
        let max_fee: Percentage = Percentage(dec!(1.35));
        let mut lp_pool: LpPool = LpPool::init(price, min_fee, max_fee, TokenAmount(dec!(90)));
    
        let added_liquidity: AddLiquidityResult = lp_pool.add_liquidity(TokenAmount(dec!(100)));
        assert!(matches!(added_liquidity, AddLiquidityResult::Success(_)));
    
        let tokens_to_remove: TokenAmount = TokenAmount(dec!(50));
        let removed_liquidity: RemovalResult = lp_pool.remove_liquidity(tokens_to_remove);
    
        assert!(matches!(removed_liquidity, RemovalResult::Success(_, _)));
    }
    
    #[test]
    fn test_remove_liquidity_errors() {
        let price: Price = Price(dec!(1.5));
        let min_fee: Percentage = Percentage(dec!(0.001));
        let max_fee: Percentage = Percentage(dec!(1.35));
        let mut lp_pool: LpPool = LpPool::init(price, min_fee, max_fee, TokenAmount(dec!(90)));
    
        let tokens_to_remove: TokenAmount = TokenAmount(dec!(1000));
        let removal_result: RemovalResult = lp_pool.remove_liquidity(tokens_to_remove);
        assert!(matches!(removal_result, RemovalResult::Error(_)));    
    }


}