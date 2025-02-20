# pbh-ctf

```

                    ██████  ██████  ██   ██      ██████ ████████ ███████ 
                    ██   ██ ██   ██ ██   ██     ██         ██    ██      
                    ██████  ██████  ███████     ██         ██    █████   
                    ██      ██   ██ ██   ██     ██         ██    ██      
                    ██      ██████  ██   ██      ██████    ██    ██      
    
                  
                                  __
                                 / \--..____
                                  \ \       \-----,,,..
                                   \ \       \         \--,,..
                                    \ \       \         \  ,'
                                     \ \       \         \ ``..
                                      \ \       \         \-''
                                       \ \       \__,,--'''
                                        \ \       \.
                                         \ \      ,/
                                          \ \__..-
                                           \ \
                                            \ \
                                             \ \   
                                              \ \
                                               \ \
                                                \ \
                                                 \ \
                                                  \ \
                                                   \ \
```


## Overview 
With the launch of Priority Blockspace for Humans (PBH) on World Chain Sepolia, a PBH CTF event will take place from `2025-02-28T05:00:00Z` to `2025-03-08T04:59:00Z` to discover edge cases and observe interesting/unexpected/outcomes.




## Getting a Testnet World ID

TODO: explain that pbh will be used 
TODO: explain how to get a testnet event id








## Warmup Game: PBH King of the Hill
- **Start Time:** `2025-02-28T05:00:00Z`
- **End Time:** `2025-03-02T04:59:00Z`


```solidity
contract PBHKotH {
    // --snip--
    /// @notice Function to attempt to capture the flag
    function ctf(address addr) public {
        require(block.timestamp < gameEnd, GameOver());

        // Ensure ctf hasnt been called yet this block
        require(block.timestamp > lastBlock, TooLate());
        lastBlock = uint128(block.timestamp);

        // Adjust the user's score
        uint256 score = leaderboard[addr];
        score += 1;
        leaderboard[addr] = score;

        if (score > highScore) {
            leader = addr;
            highScore = score;
        }
    }
}
```

## Break Things: 
- **Start Time:** `2025-02-28T05:00:00Z`
- **End Time:** `2025-03-08T04:59:00Z`



## PBH Testnet Configuration
- `pbhNonceLimit`: 100
    - TODO: explain this value
- `pbhGasLimit`: 10,500,000
    - TODO: explain this value
- `verifiedBlockspaceCapacity`: 70%
    - TODO: explain this value

## Important Links
- [World Chain Builder repo](https://github.com/worldcoin/world-chain/tree/main/world-chain-builder/crates/world)
- [PBH Specs](https://worldcoin.github.io/world-chain/)
- [Inclusion Proof RPC endpoint](TODO:)
- [World ID docs](https://docs.world.org/world-id/reference/contracts#usage)

## Testnet Contract Addresses
- [WorldID](https://worldscan.org/TODO:): `0xcoffee`
- [PBHEntryPoint](https://worldscan.org/TODO:): `0xcoffee`
- [PBHSignatureAggregator](https://worldscan.org/TODO:): `0xcoffee`
