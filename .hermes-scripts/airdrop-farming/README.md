# Airdrop Farming Scripts

Automated farming scripts for Berachain Artio testnet and Abstract testnet airdrops.

**⚠️ SECURITY WARNINGS - READ BEFORE USE ⚠️**

1. **NEVER** enter real seed phrases or mainnet private keys
2. These scripts create **NEW WALLETS ONLY** for testnet use
3. Never reuse testnet wallets on mainnet
4. Always verify contract addresses before transactions
5. These are educational/templates - actual contracts must be verified

---

## Directory Structure

```
/home/l-vs/Polygone/.hermes-scripts/airdrop-farming/
├── README.md                    # This file
├── berachain-farm.py           # Berachain Artio testnet farmer
├── abstract-farm.py            # Abstract testnet farmer
├── config.yaml                 # Configuration file (create from example)
└── requirements.txt             # Python dependencies
```

---

## Berachain Artio Testnet

### What is Berachain?
Berachain is a novel EVM-compatible L1 blockchain with a unique proof-of-stake and liquidity commit mechanism. The Artio testnet is their active testnet where you can test dApps and potentially qualify for an airdrop.

### Testnet Details
- **Network Name**: Berachain Artio
- **Chain ID**: 80085
- **RPC URL**: `https://artio-rpc.berachain.com`
- **Faucet**: `https://artio.faucet.berachain.com`

### Farming Activities
1. **Faucet claims** - Get test BERA tokens
2. **Bridge operations** - Bridge ETH ↔ WETH
3. **Swap operations** - Trade via DEX (Uniswap, Trident)
4. **Liquidity provision** - Add to LP pairs
5. **Governance participation** - Delegate votes

### Usage

```bash
# Install dependencies
pip install web3 eth-account mnemonic pyyaml requests

# Generate new wallet
python3 berachain-farm.py --new-wallet --save-wallet ./my-wallet.json

# Get test tokens
python3 berachain-farm.py --faucet

# Run full farming sequence
python3 berachain-farm.py --run-all --tx-count 10
```

### Cron Setup (Daily at 8 AM)
```bash
0 8 * * * /usr/bin/python3 /home/l-vs/Polygone/.hermes-scripts/airdrop-farming/berachain-farm.py --run-all >> /var/log/berachain-farm.log 2>&1
```

### Estimated Airdrop Value
- **Berachain** is considered HIGH priority airdrop
- Estimates: $500 - $5,000+ per wallet (highly speculative)
- Factors: Multi-wallet farming, diverse transactions, early participation

---

## Abstract Testnet

### What is Abstract?
Abstract is an Ethereum L2 focusing on consumer-friendly web3 apps. Their testnet may qualify participants for an airdrop.

### Testnet Details (Verify These!)
- **Network Name**: Abstract Testnet
- **Chain ID**: 11124 (verify!)
- **RPC URL**: `https://api.testnet.abs.xyz`
- **Faucet**: `https://faucet.testnet.abs.xyz`

### Farming Activities
1. **Faucet claims** - Get test ETH
2. **Bridge operations** - Bridge from L1
3. **Swap operations** - Trade on DEXes
4. **Liquidity provision** - LP rewards
5. **NFT minting** - Interact with NFT contracts

### Usage

```bash
# Install dependencies
pip install web3 eth-account mnemonic pyyaml requests

# Generate new wallet
python3 abstract-farm.py --new-wallet --save-wallet ./my-wallet.json

# Get test tokens
python3 abstract-farm.py --faucet

# Run full farming sequence
python3 abstract-farm.py --run-all --tx-count 10
```

### Cron Setup (Daily at 9 AM)
```bash
0 9 * * * /usr/bin/python3 /home/l-vs/Polygone/.hermes-scripts/airdrop-farming/abstract-farm.py --run-all >> /var/log/abstract-farm.log 2>&1
```

### Estimated Airdrop Value
- **Abstract** is a newer project, medium-high priority
- Estimates: $200 - $2,000+ per wallet (highly speculative)
- Factors: Early participation, diverse interactions

---

## Security Best Practices

### DO ✅
- Generate fresh wallets for each testnet
- Use strong passwords for keystore files
- Store seed phrases offline in secure location
- Verify all contract addresses on official docs
- Use separate wallets for testnet vs mainnet
- Check your transactions on block explorers

### DON'T ❌
- Enter real seed phrases in scripts
- Reuse testnet wallets on mainnet
- Share private keys or seed phrases
- Skip contract address verification
- Farm with more than you can afford to lose on testnet
- Ignore rate limits on public RPCs

### Wallet Security
```bash
# Store wallet securely
chmod 700 /path/to/wallets/
chmod 600 /path/to/wallet.json

# Use hardware wallet for mainnet operations
```

---

## Setup Instructions

### Prerequisites
```bash
# Python 3.8+
python3 --version

# pip
pip3 --version

# Optional: cast (for blockchain interactions)
curl -L https://foundry.paradigm.xyz | bash
foundryup
```

### Installation
```bash
# Create directory
mkdir -p /home/l-vs/Polygone/.hermes-scripts/airdrop-farming
cd /home/l-vs/Polygone/.hermes-scripts/airdrop-farming

# Clone or copy scripts
cp /path/to/scripts/* .

# Install Python dependencies
pip install web3 eth-account mnemonic pyyaml requests
```

### Configuration
Edit the configuration section at the top of each script:

```python
# Berachain
ARTIO_RPC_URLS = ["https://artio-rpc.berachain.com"]
WRAPPED_ETH = "0x..."  # Verify!
USDC_TOKEN = "0x..."   # Verify!
UNISWAP_ROUTER = "0x..."  # Verify!
```

---

## Troubleshooting

### "RPC endpoint failed"
- Try alternate RPC URLs
- Check if testnet is down
- Use rate limits appropriately

### "Insufficient funds"
- Request more from faucet
- Check if faucet is working
- Wait for block confirmations

### "Transaction reverted"
- Verify contract addresses
- Check token approvals
- Increase gas if needed

### "Connection timeout"
- Check internet connection
- Try different RPC
- Reduce transaction frequency

---

## Disclaimer

These scripts are for **EDUCATIONAL PURPOSES** only. They are templates that require:
1. Contract address verification
2. ABI/interface verification  
3. Testing on small amounts first

The estimated airdrop values are **SPECULATIVE** and based on:
- Similar project airdrops
- Current market conditions
- Project funding rounds
- Team reputation

**DO NOT** invest more than you can afford to lose. Testnet farming is free but your time has value.

---

## License

Educational use only. Use at your own risk.
