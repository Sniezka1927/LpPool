# Known issues

### Liquidity Pool

Liquidity Pool is out of target liquidity.

### Charging fee

The charging fee mechanism is not completly correct. It does not vary on available Tokens in the LpPool and target liquidity. The fee taken from each transaction is randomized.

### Adding liquidity

Adding liquidiy should charge the fee while the total reserve is over the target liquidity (?)

### The application structure

Application could be separed into modules.
