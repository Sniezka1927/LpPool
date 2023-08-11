# LP Pool Application

This Rust application simulates an LP (Liquidity Pool) system. It provides functionality for adding liquidity, swapping tokens, and removing liquidity from the pool. The application uses fixed-point decimal arithmetic for calculations and includes fee calculations based on a random XOR-Shift algorithm.

## Features

- Add liquidity to the pool
- Swap tokens within the pool
- Remove liquidity from the pool
- Fee calculations using a random XOR-Shift algorithm
- Fixed-point decimal arithmetic for accuracy

## Getting Started

### Prerequisites

- Rust: Ensure Rust is installed on your system. If not, you can download and install it from the [official Rust website](https://www.rust-lang.org/tools/install).

### Installation

1. Clone this repository:

   ```bash
   git clone https://github.com/your-username/lp-pool-app.git

   ```

2. Navigate to the project directoy:

   ```bash
   cd lp-pool-app
   ```

### Usage

1. Build the application:

   ```bash
   cargo build

   ```

2. Run the application:

   ```bash
   cargo run
   ```

### Application Overview

The LP Pool application simulates the behavior of a liquidity pool, where users can interact with the pool through various actions. Key components include:

- LpPool struct: Represents the liquidity pool with price, token reserves, staked tokens, and LP tokens.
- TokenAmount, StakedTokenAmount, LPTokenAmount, Price, and Percentage structs: Used to represent different amounts and values.
- RemovalResult, AddLiquidityResult, SwapResult enums: Represent outcomes of liquidity removal, adding and token swapping operations.
- Fixed-point decimal arithmetic ensures precision in calculations, while fee calculations are based on a random XOR-Shift algorithm.

### Examples

The main function in the application demonstrates interactions with the LP Pool:

1. Adding liquidity: Simulates adding tokens to the liquidity pool.
2. Swapping tokens: Simulates swapping staked tokens within the pool.
3. Adding more liquidity: Adds additional tokens to the liquidity pool.
4. Staking tokens: Simulates staking tokens in the pool.
5. Removing liquidity: Simulates removing tokens from the liquidity pool.
