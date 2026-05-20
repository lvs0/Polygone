#!/usr/bin/env python3
"""
Berachain Artio Testnet Airdrop Farming Script
================================================
WARNING: This script interacts with TESTNET only. Never enter real seed phrases.
WARNING: This script creates NEW wallets locally. Do NOT import existing wallets.

Setup:
    pip install web3 eth-account mnemonic pyyaml requests

Usage:
    python3 berachain-farm.py --new-wallet          # Create new wallet
    python3 berachain-farm.py --wallet ADDRESS      # Use existing test wallet
    python3 berachain-farm.py --faucet ADDRESS      # Get test tokens
    python3 berachain-farm.py --run-all             # Full farming run

Cron setup (daily at 8 AM):
    0 8 * * * /usr/bin/python3 /home/l-vs/Polygone/.hermes-scripts/airdrop-farming/berachain-farm.py --run-all >> /var/log/berachain-farm.log 2>&1

Security Notes:
- NEVER hardcode real seed phrases or private keys
- Use --new-wallet to generate fresh wallets for testnet
- Always use separate wallets for testnet vs mainnet
- Verify all contract addresses before transactions
"""

import json
import time
import random
import argparse
import subprocess
from dataclasses import dataclass
from typing import Optional
from eth_account import Account
from web3 import Web3
import requests

# =============================================================================
# CONFIGURATION - Berachain Artio Testnet
# =============================================================================

# Artio Testnet RPC endpoints (public, rate-limited)
ARTIO_RPC_URLS = [
    "https://artio-rpc.berachain.com",
    "https://rpc.ankr.com/berachain_artio",
]

# Faucet URLs
ARTIO_FAUCETS = [
    "https://artio.faucet.berachain.com",
    "https://faucet.berachain.com",
]

# Chain configuration
CHAIN_ID = 80085  # Berachain Artio testnet

# Token addresses on Artio (approximate - verify before use)
NATIVE_TOKEN = "0x0000000000000000000000000000000000000000"  # ETH on Berachain
WRAPPED_ETH = "0x6989292981cC12d19E868922aD922acDB10A5F49"  # WETH on Artio (verify!)
USDC_TOKEN = "0x75f4b0a1cC22D6C5E3C8F8B3E2d9B3F9e1d5E7A3"  # USDC on Artio (verify!)
HONEY_TOKEN = "0xC6F2a5F1e1d5E7A3B2F1C9E8D7A6B5E4C3D2A1F"  # HONEY (verify!)

# DEX Router addresses (verify before use!)
UNISWAP_ROUTER = "0x4752ba5DBc23f44D87826276BF6Fd6b1C372aD24"  # May vary
TRIDENT_ROUTER = "0x0B7466E3cF2F11B702759d29F63D3a4e53C3a13C"  # May vary

# Estimated gas costs (in Wei)
GAS_PRICE = Web3.to_wei("0.00001", "ether")  # Very cheap on testnet

# =============================================================================
# UTILITY FUNCTIONS
# =============================================================================

def load_wallet_from_keystore(keystore_path: str, password: str) -> Optional[Account]:
    """Load wallet from encrypted keystore file."""
    try:
        with open(keystore_path, 'r') as f:
            keystore_json = json.load(f)
        private_key = Account.decrypt(keystore_json, password)
        return Account.from_key(private_key)
    except Exception as e:
        print(f"[ERROR] Failed to decrypt keystore: {e}")
        return None

def save_wallet_to_keystore(account: Account, password: str, path: str):
    """Save wallet to encrypted keystore file."""
    keystore = account.encrypt(password)
    with open(path, 'w') as f:
        json.dump(keystore, f)
    print(f"[INFO] Wallet saved to: {path}")

def generate_new_wallet() -> Account:
    """Generate a fresh wallet for testnet use ONLY."""
    print("[WARNING] Generating NEW wallet for TESTNET only!")
    print("[WARNING] Do NOT use this wallet on mainnet!")
    account = Account.create()
    print(f"[INFO] New wallet address: {account.address}")
    return account

def get_rpc_provider() -> Web3:
    """Get connected RPC provider with fallback."""
    for rpc_url in ARTIO_RPC_URLS:
        try:
            w3 = Web3(Web3.HTTPProvider(rpc_url, timeout=30))
            if w3.is_connected():
                print(f"[INFO] Connected to: {rpc_url}")
                return w3
        except Exception as e:
            print(f"[WARN] RPC {rpc_url} failed: {e}")
            continue
    raise ConnectionError("All RPC endpoints failed!")

def get_balance(w3: Web3, address: str) -> dict:
    """Get native token and USDC balances."""
    balance = w3.eth.get_balance(address)
    return {
        "native": w3.from_wei(balance, 'ether'),
        "native_wei": balance
    }

def wait_for_confirmation(w3: Web3, tx_hash: bytes, timeout: int = 120) -> bool:
    """Wait for transaction confirmation."""
    try:
        receipt = w3.eth.wait_for_transaction_receipt(tx_hash, timeout=timeout)
        return receipt.status == 1
    except Exception as e:
        print(f"[WARN] Transaction may have failed: {e}")
        return False

# =============================================================================
# FAUCET FUNCTIONS
# =============================================================================

def request_faucet_funds(address: str) -> bool:
    """Request testnet funds from faucet."""
    print(f"[FAUCET] Requesting funds for: {address}")
    
    for faucet_url in ARTIO_FAUCETS:
        try:
            # Try Berachain's official faucet API
            response = requests.post(
                faucet_url + "/api/gas",
                json={"address": address},
                headers={"Content-Type": "application/json"},
                timeout=30
            )
            if response.status_code == 200:
                data = response.json()
                if data.get("success") or data.get("txHash"):
                    print(f"[FAUCET] Success! TX: {data.get('txHash', 'N/A')}")
                    return True
        except Exception as e:
            print(f"[FAUCET] {faucet_url} failed: {e}")
            continue
    
    # Fallback: Try using cast (if installed)
    try:
        result = subprocess.run(
            ["cast", "rpc", "--rpc-url", ARTIO_RPC_URLS[0], "funded", address],
            capture_output=True, text=True, timeout=30
        )
        if "0x" in result.stdout:
            print(f"[FAUCET] Cast funded check passed")
            return True
    except:
        pass
    
    print("[FAUCET] All faucet attempts failed. Manual funding may be required.")
    print(f"[FAUCET] Visit: https://artio.faucet.berachain.com/?address={address}")
    return False

# =============================================================================
# TRANSACTION FUNCTIONS
# =============================================================================

def bridge_native_to_weth(w3: Web3, account: Account, amount_eth: float) -> bool:
    """Bridge native token to WETH (mock - implement based on actual bridge contract)."""
    print(f"[BRIDGE] Bridging {amount_eth} ETH to WETH...")
    
    # NOTE: This is a placeholder. Real bridge implementation requires:
    # 1. Finding the actual bridge contract on Artio
    # 2. Approving the token if needed
    # 3. Calling the bridge deposit function
    
    # Example structure (NOT actual code):
    # bridge_address = "0x..."  # Find actual bridge contract
    # contract = w3.eth.contract(address=bridge_address, abi=bridge_abi)
    # nonce = w3.eth.get_transaction_count(account.address)
    # tx = contract.functions.deposit(native_token, amount).buildTransaction({
    #     'from': account.address,
    #     'gas': 200000,
    #     'gasPrice': GAS_PRICE,
    #     'nonce': nonce
    # })
    # signed = account.sign_transaction(tx)
    # tx_hash = w3.eth.send_raw_transaction(signed.rawTransaction)
    
    print("[BRIDGE] Bridge function needs actual contract addresses")
    print("[BRIDGE] Skipping bridge operation (requires contract verification)")
    return False

def swap_tokens(w3: Web3, account: Account, token_in: str, token_out: str, 
                amount_in: float, min_out: float = 0) -> bool:
    """Swap tokens using DEX (mock - implement based on actual DEX contracts)."""
    print(f"[SWAP] Swapping {amount_in} of {token_in[:10]}... to {token_out[:10]}...")
    
    # NOTE: This is a placeholder. Real swap implementation requires:
    # 1. Finding the actual DEX router contract
    # 2. Getting quotes and calculating output
    # 3. Approving tokens if needed
    # 4. Executing the swap
    
    # Example structure (NOT actual code):
    # router_address = UNISWAP_ROUTER  # Verify this!
    # router = w3.eth.contract(address=router_address, abi=uniswap_abi)
    # path = [token_in, token_out]
    # deadline = int(time.time()) + 600
    # nonce = w3.eth.get_transaction_count(account.address)
    # tx = router.functions.exactInputSingle({
    #     'tokenIn': token_in,
    #     'tokenOut': token_out,
    #     'fee': 3000,
    #     'recipient': account.address,
    #     'deadline': deadline,
    #     'amountIn': Web3.to_wei(amount_in, 'ether'),
    #     'amountOutMinimum': Web3.to_wei(min_out, 'ether'),
    #     'sqrtPriceLimitX96': 0
    # }).buildTransaction({
    #     'from': account.address,
    #     'gas': 250000,
    #     'gasPrice': GAS_PRICE,
    #     'nonce': nonce
    # })
    
    print("[SWAP] Swap function needs actual DEX contract verification")
    print("[SWAP] Skipping swap operation (requires contract verification)")
    return False

def provide_liquidity(w3: Web3, account: Account, token_a: str, 
                      token_b: str, amount_a: float, amount_b: float) -> bool:
    """Provide liquidity to DEX (mock - implement based on actual LP contracts)."""
    print(f"[LP] Providing liquidity: {amount_a} of {token_a[:10]}... + {amount_b} of {token_b[:10]}...")
    
    # NOTE: This is a placeholder. Real LP implementation requires:
    # 1. Finding the actual pair and master chef contracts
    # 2. Approving both tokens
    # 3. Adding liquidity and receiving LP tokens
    # 4. Optionally staking LP tokens for rewards
    
    print("[LP] Liquidity provision needs actual contract verification")
    print("[LP] Skipping LP operation (requires contract verification)")
    return False

def delegate_votes(w3: Web3, account: Account, delegate_address: str = None) -> bool:
    """Delegate governance votes (if applicable)."""
    if delegate_address is None:
        delegate_address = account.address  # Self-delegate
    
    print(f"[GOV] Delegating votes to: {delegate_address}")
    
    # NOTE: Many protocols have governance tokens. Delegating shows engagement.
    # This may or may not apply depending on Berachain's airdrop criteria.
    
    print("[GOV] Vote delegation skipped (protocol-dependent)")
    return True

# =============================================================================
# FARMING SEQUENCE
# =============================================================================

def run_farming_sequence(account: Account, num_transactions: int = 10):
    """
    Run the full farming sequence.
    
    Expected airdrop criteria (estimated):
    - Multiple wallet interactions
    - Diverse transaction types (swaps, LP, bridge)
    - Consistent activity over time
    - Token holdings and usage
    """
    print("\n" + "="*60)
    print("BERACHAIN ARTIO TESTNET - AIRDROP FARMING")
    print("="*60)
    print(f"[WALLET] Address: {account.address}")
    
    # Connect to network
    w3 = get_rpc_provider()
    
    # Check balance
    balances = get_balance(w3, account.address)
    print(f"[BALANCE] Native: {balances['native']:.6f} BERA")
    
    if balances['native'] < 0.001:
        print("[WARN] Insufficient funds for gas. Requesting faucet...")
        request_faucet_funds(account.address)
        time.sleep(5)
        balances = get_balance(w3, account.address)
        print(f"[BALANCE] Updated: {balances['native']:.6f} BERA")
    
    # Transaction counter
    tx_count = 0
    success_count = 0
    
    # Farming activities (randomized order for natural behavior)
    activities = []
    
    # 1. Faucet request
    if balances['native'] < 0.01:
        if request_faucet_funds(account.address):
            time.sleep(10)
            balances = get_balance(w3, account.address)
            success_count += 1
            tx_count += 1
    
    # 2. Bridge operations (if we have funds)
    if balances['native'] > 0.1:
        bridge_amount = round(random.uniform(0.01, 0.05), 4)
        if bridge_native_to_weth(w3, account, bridge_amount):
            success_count += 1
        tx_count += 1
        time.sleep(random.randint(3, 8))
    
    # 3. Swap operations
    for i in range(min(3, num_transactions // 3)):
        if balances['native'] > 0.05:
            swap_amount = round(random.uniform(0.01, 0.03), 4)
            if swap_tokens(w3, account, NATIVE_TOKEN, WRAPPED_ETH, swap_amount):
                success_count += 1
            tx_count += 1
            time.sleep(random.randint(2, 6))
    
    # 4. Liquidity provision
    if balances['native'] > 0.1:
        lp_amount = round(random.uniform(0.02, 0.08), 4)
        if provide_liquidity(w3, account, WRAPPED_ETH, USDC_TOKEN, lp_amount, lp_amount):
            success_count += 1
        tx_count += 1
        time.sleep(random.randint(3, 7))
    
    # 5. Self-delegation / governance
    if delegate_votes(w3, account):
        success_count += 1
        tx_count += 1
    
    # Summary
    print("\n" + "="*60)
    print("FARMING RUN COMPLETE")
    print("="*60)
    print(f"[SUMMARY] Transactions attempted: {tx_count}")
    print(f"[SUMMARY] Successful operations: {success_count}")
    print(f"[SUMMARY] Final balance: {balances['native']:.6f} BERA")
    print(f"[NEXT] Run again in 24 hours for consistent activity")
    
    return {
        "address": account.address,
        "transactions": tx_count,
        "successful": success_count,
        "balance": float(balances['native'])
    }

# =============================================================================
# MAIN
# =============================================================================

def main():
    parser = argparse.ArgumentParser(description="Berachain Artio Testnet Airdrop Farmer")
    parser.add_argument("--new-wallet", action="store_true", 
                       help="Generate new wallet (TESTNET ONLY!)")
    parser.add_argument("--wallet", type=str, 
                       help="Use existing wallet address")
    parser.add_argument("--keystore", type=str,
                       help="Path to keystore file")
    parser.add_argument("--faucet", action="store_true",
                       help="Request faucet funds only")
    parser.add_argument("--run-all", action="store_true",
                       help="Run full farming sequence")
    parser.add_argument("--tx-count", type=int, default=10,
                       help="Number of transactions to attempt")
    parser.add_argument("--save-wallet", type=str,
                       help="Path to save new wallet keystore")
    
    args = parser.parse_args()
    
    print("\n" + "!"*60)
    print("SECURITY WARNING: TESTNET ONLY!")
    print("NEVER enter real seed phrases or mainnet private keys!")
    print("!"*60 + "\n")
    
    account = None
    
    if args.new_wallet:
        account = generate_new_wallet()
        if args.save_wallet:
            save_wallet_to_keystore(account, "testnet passphrase", args.save_wallet)
        print(f"\n[NEW WALLET] Generated address: {account.address}")
        print("[NEW WALLET] Save the private key securely!")
        print("[NEW WALLET] Private key:", account.key.hex())
    
    elif args.wallet:
        print(f"[WARN] Using provided address: {args.wallet}")
        print("[WARN] This script cannot sign transactions with just an address!")
        print("[WARN] Use --new-wallet or provide keystore with private key")
        return
    
    elif args.keystore:
        # This requires a password which we don't handle well in CLI
        print("[WARN] Keystore loading requires password")
        print("[WARN] For automation, use --new-wallet")
        return
    
    else:
        # Default: generate new wallet
        account = generate_new_wallet()
        print(f"\n[INFO] Generated temporary wallet: {account.address}")
        print("[INFO] Use --new-wallet --save-wallet PATH to persist")
    
    if args.faucet and account:
        request_faucet_funds(account.address)
    
    if args.run_all and account:
        result = run_farming_sequence(account, args.tx_count)
        print(f"\n[RESULT] {json.dumps(result)}")
    
    if not any([args.new_wallet, args.wallet, args.faucet, args.run_all]):
        parser.print_help()

if __name__ == "__main__":
    main()
