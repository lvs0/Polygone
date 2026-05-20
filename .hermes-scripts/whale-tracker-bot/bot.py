#!/usr/bin/env python3
"""
Whale Tracker Bot - Blockchain Analytics Telegram Bot
Monetizable service for monitoring whale wallets and on-chain movements
"""

import os
import logging
import asyncio
from datetime import datetime
from decimal import Decimal
from typing import Optional

from telegram import Update, InlineKeyboardButton, InlineKeyboardMarkup
from telegram.ext import (
    Application,
    CommandHandler,
    CallbackQueryHandler,
    MessageHandler,
    filters,
    ContextTypes,
)
from telegram.constants import ParseMode

from web3 import Web3
from web3.eth import Eth
from web3.contract import Contract
from web3.exceptions import BlockNotFoundError

from sqlalchemy.orm import Session
from sqlalchemy import create_engine, Column, Integer, String, Float, Boolean, DateTime, Text
from sqlalchemy.ext.declarative import declarative_base
from sqlalchemy.orm import sessionmaker

# =============================================================================
# CONFIGURATION
# =============================================================================

TELEGRAM_BOT_TOKEN = os.getenv("TELEGRAM_BOT_TOKEN", "YOUR_BOT_TOKEN_HERE")
ETHEREUM_RPC_URL = os.getenv("ETHEREUM_RPC_URL", "https://eth.llamarpc.com")
ETHERSCAN_API_KEY = os.getenv("ETHERSCAN_API_KEY", "")
DATABASE_URL = os.getenv("DATABASE_URL", "sqlite:///whale_tracker.db")

# Subscription tiers (monthly pricing in USD)
SUBSCRIPTION_TIERS = {
    "free": {
        "name": "Free",
        "price": 0,
        "wallets": 3,
        "alerts_per_hour": 5,
        "chains": ["ethereum"],
    },
    "basic": {
        "name": "Basic",
        "price": 15,
        "wallets": 20,
        "alerts_per_hour": 50,
        "chains": ["ethereum", "polygon"],
    },
    "pro": {
        "name": "Pro",
        "price": 35,
        "wallets": 100,
        "alerts_per_hour": 500,
        "chains": ["ethereum", "polygon", "bsc", "arbitrum"],
    },
    "whale": {
        "name": "Whale",
        "price": 75,
        "wallets": float("inf"),
        "alerts_per_hour": float("inf"),
        "chains": ["ethereum", "polygon", "bsc", "arbitrum", "optimism"],
    },
}

# Known whale addresses (publicly known)
KNOWN_WHALES = {
    "0x28c6c06298d514db089934071355e5743bf21d60": {"name": "Binance Hot Wallet", "type": "exchange"},
    "0x21a31ee1afc51d94c2efccaa2092ad1028285540": {"name": "Binance cold wallet", "type": "exchange"},
    "0xdfd5293d8e347dfe59e90ffd55f1a5e8d2b3c5b5": {"name": "Binance Hot Wallet 2", "type": "exchange"},
    "0xa9d1e08c7793af67e9b59289e1d3a5f7b5c29f8c": {"name": "Alameda Research", "type": "institution"},
    "0xc099b6a1d8fa8d9bb4a40f8fe0ccfa2e0f80b2e9": {"name": "Wintermute", "type": "market_maker"},
    "0x5f3f73f9f86a48d6ca9c3f0e8d8f7b1a6f23c4e2": {"name": "Jump Trading", "type": "market_maker"},
    "0x47ac0fb4f2d84898e4d9e7b4dab3c55c5a8bd5c4": {"name": "Cumberland", "type": "institution"},
    "0x2faf487c0a567bc64381724067a044e53b54ad08": {"name": "Santiment Lab", "type": "analytics"},
}

# =============================================================================
# LOGGING
# =============================================================================

logging.basicConfig(
    format="%(asctime)s - %(name)s - %(levelname)s - %(message)s",
    level=logging.INFO,
)
logger = logging.getLogger(__name__)

# =============================================================================
# DATABASE MODELS
# =============================================================================

Base = declarative_base()


class User(Base):
    __tablename__ = "users"

    id = Column(Integer, primary_key=True)
    telegram_id = Column(String(50), unique=True, nullable=False)
    username = Column(String(100))
    tier = Column(String(20), default="free")
    subscription_end = Column(DateTime, nullable=True)
    created_at = Column(DateTime, default=datetime.utcnow)
    is_active = Column(Boolean, default=True)


class Wallet(Base):
    __tablename__ = "wallets"

    id = Column(Integer, primary_key=True)
    user_id = Column(Integer, nullable=False)
    address = Column(String(100), nullable=False)
    label = Column(String(200), nullable=True)
    chain = Column(String(20), default="ethereum")
    is_whale = Column(Boolean, default=False)
    created_at = Column(DateTime, default=datetime.utcnow)


class Alert(Base):
    __tablename__ = "alerts"

    id = Column(Integer, primary_key=True)
    user_id = Column(Integer, nullable=False)
    wallet_address = Column(String(100), nullable=False)
    transaction_hash = Column(String(200), nullable=False)
    alert_type = Column(String(50), nullable=False)  # large_transfer, whale_movement, etc.
    amount_usd = Column(Float, nullable=True)
    token_symbol = Column(String(20), nullable=True)
    direction = Column(String(10), nullable=True)  # in, out
    message = Column(Text, nullable=True)
    created_at = Column(DateTime, default=datetime.utcnow)
    is_read = Column(Boolean, default=False)


class Subscription(Base):
    __tablename__ = "subscriptions"

    id = Column(Integer, primary_key=True)
    user_id = Column(Integer, nullable=False)
    tier = Column(String(20), nullable=False)
    start_date = Column(DateTime, default=datetime.utcnow)
    end_date = Column(DateTime, nullable=True)
    payment_id = Column(String(100), nullable=True)
    is_active = Column(Boolean, default=True)


# Database setup
engine = create_engine(DATABASE_URL)
Base.metadata.create_all(engine)
SessionLocal = sessionmaker(bind=engine)


def get_db():
    db = SessionLocal()
    try:
        return db
    except Exception:
        db.close()
        raise


# =============================================================================
# WEB3 SETUP
# =============================================================================

class BlockchainClient:
    """Web3 client for multi-chain blockchain data"""

    def __init__(self, rpc_url: str = ETHEREUM_RPC_URL):
        self.w3 = Web3(Web3.HTTPProvider(rpc_url))
        self.is_connected = self.w3.is_connected()

    def get_balance(self, address: str) -> Decimal:
        """Get ETH balance"""
        try:
            addr = self.w3.to_checksum_address(address)
            balance_wei = self.w3.eth.get_balance(addr)
            return Decimal(balance_wei) / Decimal(10**18)
        except Exception as e:
            logger.error(f"Error getting balance: {e}")
            return Decimal(0)

    def get_transaction(self, tx_hash: str) -> Optional[dict]:
        """Get transaction details"""
        try:
            tx = self.w3.eth.get_transaction(tx_hash)
            receipt = self.w3.eth.get_transaction_receipt(tx_hash)
            return {
                "hash": tx.hash.hex(),
                "from": tx["from"],
                "to": tx["to"],
                "value": float(tx["value"]) / 10**18,
                "gas_price": float(tx["gasPrice"]) / 10**9,
                "gas_used": receipt["gasUsed"],
                "status": receipt["status"],
                "block_number": tx["blockNumber"],
                "input": tx["input"][:10],  # first 4 bytes (function selector)
            }
        except Exception as e:
            logger.error(f"Error getting transaction: {e}")
            return None

    def get_wallet_transfers(self, address: str, last_block: int = 0, limit: int = 100) -> list:
        """Get wallet transfers since block"""
        try:
            addr = self.w3.to_checksum_address(address)
            logs = self.w3.eth.filter({
                "fromBlock": last_block,
                "address": addr,
            }).get_all_entries()
            return logs[-limit:] if len(logs) > limit else logs
        except Exception as e:
            logger.error(f"Error getting transfers: {e}")
            return []

    def estimate_usd_value(self, amount_wei: int, price_usd: float = 2000.0) -> float:
        """Estimate USD value of transaction"""
        return float(amount_wei) / 10**18 * price_usd

    def is_contract(self, address: str) -> bool:
        """Check if address is a contract"""
        try:
            addr = self.w3.to_checksum_address(address)
            code = self.w3.eth.get_code(addr)
            return len(code) > 2
        except Exception:
            return False


# =============================================================================
# TELEGRAM BOT HANDLERS
# =============================================================================

# Global blockchain client
eth_client = BlockchainClient()


def get_user_tier(telegram_id: str) -> str:
    """Get user's subscription tier"""
    db = get_db()
    try:
        user = db.query(User).filter(User.telegram_id == str(telegram_id)).first()
        if user and user.subscription_end and user.subscription_end > datetime.utcnow():
            return user.tier
        return "free"
    finally:
        db.close()


def get_user_wallet_count(telegram_id: str) -> int:
    """Get number of wallets user is monitoring"""
    db = get_db()
    try:
        user = db.query(User).filter(User.telegram_id == str(telegram_id)).first()
        if user:
            return db.query(Wallet).filter(Wallet.user_id == user.id).count()
        return 0
    finally:
        db.close()


async def start_command(update: Update, context: ContextTypes.DEFAULT_TYPE):
    """Handle /start command"""
    keyboard = [
        [InlineKeyboardButton("➕ Add Wallet", callback_data="add_wallet")],
        [InlineKeyboardButton("📊 My Wallets", callback_data="list_wallets")],
        [InlineKeyboardButton("💰 Check Balance", callback_data="check_balance")],
        [InlineKeyboardButton("📈 Upgrade Plan", callback_data="upgrade")],
        [InlineKeyboardButton("🐋 Whale Alerts", callback_data="whale_alerts")],
    ]
    reply_markup = InlineKeyboardMarkup(keyboard)

    welcome_text = (
        "🐋 *Whale Tracker Bot*\n\n"
        "Monitor whale wallets, track on-chain movements, and get real-time alerts.\n\n"
        "*Your current plan:* Free\n"
        "*Wallets monitored:* 3/3\n\n"
        "Select an option below:"
    )

    await update.message.reply_text(
        welcome_text, parse_mode=ParseMode.MARKDOWN, reply_markup=reply_markup
    )


async def help_command(update: Update, context: ContextTypes.DEFAULT_TYPE):
    """Handle /help command"""
    help_text = (
        "🐋 *Whale Tracker Bot - Help*\n\n"
        "*Commands:*\n"
        "/start - Start the bot\n"
        "/add <address> - Add wallet to monitor\n"
        "/remove <address> - Remove wallet\n"
        "/wallets - List your wallets\n"
        "/balance <address> - Check wallet balance\n"
        "/tier - View subscription plans\n"
        "/upgrade - Upgrade your plan\n"
        "/help - Show this help\n\n"
        "*Premium Features:*\n"
        "• Multi-chain monitoring (ETH, BSC, Polygon)\n"
        "• Real-time whale alerts\n"
        "• Token analytics\n"
        "• Custom alert thresholds\n"
    )
    await update.message.reply_text(help_text, parse_mode=ParseMode.MARKDOWN)


async def add_wallet_command(update: Update, context: ContextTypes.DEFAULT_TYPE):
    """Handle /add <address> command"""
    if not context.args:
        await update.message.reply_text(
            "📝 *Add Wallet*\n\n"
            "Usage: /add <wallet_address>\n"
            "Example: /add 0x1234...5678",
            parse_mode=ParseMode.MARKDOWN,
        )
        return

    address = context.args[0].strip()
    tier = get_user_tier(update.effective_user.id)
    tier_config = SUBSCRIPTION_TIERS.get(tier, SUBSCRIPTION_TIERS["free"])

    # Validate address
    if not address.startswith("0x") or len(address) != 42:
        await update.message.reply_text("❌ Invalid Ethereum address format.")
        return

    db = get_db()
    try:
        user = db.query(User).filter(User.telegram_id == str(update.effective_user.id)).first()
        if not user:
            user = User(telegram_id=str(update.effective_user.id), username=update.effective_user.username)
            db.add(user)
            db.commit()
            user = db.query(User).filter(User.telegram_id == str(update.effective_user.id)).first()

        current_count = db.query(Wallet).filter(Wallet.user_id == user.id).count()

        if current_count >= tier_config["wallets"]:
            await update.message.reply_text(
                f"⚠️ *Limit Reached*\n\n"
                f"You've reached your wallet limit ({tier_config['wallets']}). "
                f"Upgrade your plan to monitor more wallets.",
                parse_mode=ParseMode.MARKDOWN,
            )
            return

        # Check if already exists
        existing = db.query(Wallet).filter(
            Wallet.user_id == user.id, Wallet.address == address
        ).first()

        if existing:
            await update.message.reply_text("⚠️ Wallet already being monitored.")
            return

        # Add wallet
        is_whale = address.lower() in [w.lower() for w in KNOWN_WHALES.keys()]
        wallet = Wallet(
            user_id=user.id,
            address=address,
            label=KNOWN_WHALES.get(address, {}).get("name", "Custom Wallet"),
            is_whale=is_whale,
        )
        db.add(wallet)
        db.commit()

        await update.message.reply_text(
            f"✅ *Wallet Added*\n\n"
            f"Address: `{address}`\n"
            f"Label: {wallet.label}\n"
            f"Whale: {'🐋 Yes' if is_whale else '❌ No'}",
            parse_mode=ParseMode.MARKDOWN,
        )

    except Exception as e:
        logger.error(f"Error adding wallet: {e}")
        await update.message.reply_text("❌ Error adding wallet. Please try again.")
    finally:
        db.close()


async def list_wallets_command(update: Update, context: ContextTypes.DEFAULT_TYPE):
    """Handle /wallets command"""
    db = get_db()
    try:
        user = db.query(User).filter(User.telegram_id == str(update.effective_user.id)).first()
        if not user:
            await update.message.reply_text("You have no wallets. Use /add to add one.")
            return

        wallets = db.query(Wallet).filter(Wallet.user_id == user.id).all()

        if not wallets:
            await update.message.reply_text("📭 No wallets being monitored. Use /add to add one.")
            return

        text = "📊 *Your Monitored Wallets*\n\n"
        for w in wallets:
            whale_emoji = "🐋" if w.is_whale else "📝"
            text += f"{whale_emoji} `{w.address[:10]}...{w.address[-6:]}`\n"
            text += f"   Label: {w.label}\n"
            text += f"   Chain: {w.chain}\n\n"

        await update.message.reply_text(text, parse_mode=ParseMode.MARKDOWN)

    finally:
        db.close()


async def check_balance_command(update: Update, context: ContextTypes.DEFAULT_TYPE):
    """Handle /balance command"""
    if not context.args:
        await update.message.reply_text(
            "💰 *Check Balance*\n\n"
            "Usage: /balance <wallet_address>\n"
            "Example: /balance 0x1234...5678",
            parse_mode=ParseMode.MARKDOWN,
        )
        return

    address = context.args[0].strip()

    if not address.startswith("0x") or len(address) != 42:
        await update.message.reply_text("❌ Invalid Ethereum address format.")
        return

    balance = eth_client.get_balance(address)

    text = (
        "💰 *Wallet Balance*\n\n"
        f"Address: `{address}`\n"
        f"ETH Balance: *{balance:.6f} ETH*\n"
        f"Est. USD: *${balance * 2000:.2f}*"
    )

    await update.message.reply_text(text, parse_mode=ParseMode.MARKDOWN)


async def tier_command(update: Update, context: ContextTypes.DEFAULT_TYPE):
    """Handle /tier command - show subscription plans"""
    keyboard = [
        [InlineKeyboardButton("Basic - $15/mo", callback_data="sub_basic")],
        [InlineKeyboardButton("Pro - $35/mo", callback_data="sub_pro")],
        [InlineKeyboardButton("Whale - $75/mo", callback_data="sub_whale")],
    ]
    reply_markup = InlineKeyboardMarkup(keyboard)

    text = (
        "💎 *Subscription Plans*\n\n"
        "*Free Tier*\n"
        "• 3 wallets\n"
        "• 5 alerts/hour\n"
        "• Ethereum only\n\n"
        "*Basic - $15/mo*\n"
        "• 20 wallets\n"
        "• 50 alerts/hour\n"
        "• Ethereum + Polygon\n\n"
        "*Pro - $35/mo*\n"
        "• 100 wallets\n"
        "• 500 alerts/hour\n"
        "• ETH, Polygon, BSC, Arbitrum\n\n"
        "*Whale - $75/mo*\n"
        "• Unlimited wallets\n"
        "• Unlimited alerts\n"
        "• All chains + priority support"
    )

    await update.message.reply_text(text, parse_mode=ParseMode.MARKDOWN, reply_markup=reply_markup)


async def button_callback(update: Update, context: ContextTypes.DEFAULT_TYPE):
    """Handle inline button callbacks"""
    query = update.callback_query
    await query.answer()

    if query.data == "add_wallet":
        await query.edit_message_text(
            "📝 *Add Wallet*\n\n"
            "Send me a wallet address to monitor:\n"
            "Example: 0x1234567890abcdef1234567890abcdef12345678\n\n"
            "Tip: You can also use /add <address>",
            parse_mode=ParseMode.MARKDOWN,
        )
    elif query.data == "list_wallets":
        await list_wallets_command(update, context)
    elif query.data == "check_balance":
        await query.edit_message_text(
            "💰 *Check Balance*\n\n"
            "Send me a wallet address to check:\n"
            "Example: 0x1234567890abcdef1234567890abcdef12345678\n\n"
            "Tip: You can also use /balance <address>",
            parse_mode=ParseMode.MARKDOWN,
        )
    elif query.data == "upgrade":
        await tier_command(update, context)
    elif query.data == "whale_alerts":
        await query.edit_message_text(
            "🐋 *Whale Alerts*\n\n"
            "Enable real-time alerts for known whale wallets.\n\n"
            "*Monitored Whales:*\n"
            "• Binance Hot/Cold Wallets\n"
            "• Alameda Research\n"
            "• Wintermute\n"
            "• Jump Trading\n"
            "• Cumberland\n\n"
            "Use /add <whale_address> to start monitoring.",
            parse_mode=ParseMode.MARKDOWN,
        )
    elif query.data.startswith("sub_"):
        tier_name = query.data.replace("sub_", "")
        if tier_name in SUBSCRIPTION_TIERS:
            tier_config = SUBSCRIPTION_TIERS[tier_name]
            await query.edit_message_text(
                f"💳 *{tier_config['name']} Plan - ${tier_config['price']}/month*\n\n"
                f"Features:\n"
                f"• {tier_config['wallets']} wallets\n"
                f"• {tier_config['alerts_per_hour']} alerts/hour\n"
                f"• Chains: {', '.join(tier_config['chains'])}\n\n"
                f"Contact @support to subscribe.",
                parse_mode=ParseMode.MARKDOWN,
            )


async def handle_wallet_address(update: Update, context: ContextTypes.DEFAULT_TYPE):
    """Handle wallet address messages"""
    address = update.message.text.strip()

    if not address.startswith("0x") or len(address) != 42:
        return

    # Treat as add wallet command
    context.args = [address]
    await add_wallet_command(update, context)


async def error_handler(update: Update, context: ContextTypes.DEFAULT_TYPE):
    """Handle errors"""
    logger.error(f"Update {update} caused error {context.error}")


# =============================================================================
# WHALE MONITORING ENGINE
# =============================================================================

class WhaleMonitor:
    """Background monitoring engine for whale wallets"""

    def __init__(self, poll_interval: int = 30):
        self.poll_interval = poll_interval
        self.last_block = 0
        self.running = False

    async def start(self, app: Application):
        """Start whale monitoring loop"""
        self.running = True
        logger.info("Starting whale monitor...")

        while self.running:
            try:
                await self.check_for_whale_activity(app)
            except Exception as e:
                logger.error(f"Error in whale monitor: {e}")

            await asyncio.sleep(self.poll_interval)

    def stop(self):
        """Stop whale monitoring"""
        self.running = False
        logger.info("Stopping whale monitor...")

    async def check_for_whale_activity(self, app: Application):
        """Check for new whale activity"""
        db = get_db()
        try:
            # Get all whale wallets
            whale_wallets = db.query(Wallet).filter(Wallet.is_whale == True).all()

            current_block = eth_client.w3.eth.block_number

            for wallet in whale_wallets:
                try:
                    # Get recent logs for this wallet
                    logs = eth_client.get_wallet_transfers(
                        wallet.address,
                        last_block=max(self.last_block, current_block - 1000),
                        limit=10,
                    )

                    for log in logs:
                        # Create alert for significant transfer
                        amount_usd = eth_client.estimate_usd_value(
                            int(log.get("data", "0x0"), 16) if log.get("data") else 0
                        )

                        if amount_usd > 10000:  # Only alert for >$10k transfers
                            alert = Alert(
                                user_id=wallet.user_id,
                                wallet_address=wallet.address,
                                transaction_hash=log.transactionHash.hex() if log.transactionHash else "",
                                alert_type="whale_movement",
                                amount_usd=amount_usd,
                                direction="unknown",
                                message=f"🐋 Large transfer: ${amount_usd:.2f} from {wallet.label}",
                            )
                            db.add(alert)

                            # Send Telegram notification
                            try:
                                user = db.query(User).filter(User.id == wallet.user_id).first()
                                if user and user.is_active:
                                    await app.bot.send_message(
                                        chat_id=user.telegram_id,
                                        text=f"🐋 *Whale Alert*\n\n"
                                             f"Wallet: {wallet.label}\n"
                                             f"Amount: ${amount_usd:.2f}\n"
                                             f"Tx: `{log.transactionHash.hex() if log.transactionHash else 'N/A'}`",
                                        parse_mode=ParseMode.MARKDOWN,
                                    )
                            except Exception as e:
                                logger.error(f"Error sending alert: {e}")

                except Exception as e:
                    logger.error(f"Error checking wallet {wallet.address}: {e}")

            self.last_block = current_block
            db.commit()

        except Exception as e:
            logger.error(f"Error in whale activity check: {e}")
        finally:
            db.close()


# Global monitor instance
whale_monitor = WhaleMonitor(poll_interval=30)


# =============================================================================
# MAIN APPLICATION
# =============================================================================

def main():
    """Run the bot"""
    if TELEGRAM_BOT_TOKEN == "YOUR_BOT_TOKEN_HERE":
        logger.error("Please set TELEGRAM_BOT_TOKEN environment variable")
        return

    # Create application
    app = Application.builder().token(TELEGRAM_BOT_TOKEN).build()

    # Add handlers
    app.add_handler(CommandHandler("start", start_command))
    app.add_handler(CommandHandler("help", help_command))
    app.add_handler(CommandHandler("add", add_wallet_command))
    app.add_handler(CommandHandler("remove", add_wallet_command))
    app.add_handler(CommandHandler("wallets", list_wallets_command))
    app.add_handler(CommandHandler("balance", check_balance_command))
    app.add_handler(CommandHandler("tier", tier_command))
    app.add_handler(CommandHandler("upgrade", tier_command))
    app.add_handler(CallbackQueryHandler(button_callback))
    app.add_handler(MessageHandler(filters.TEXT & ~filters.COMMAND, handle_wallet_address))

    # Error handler
    app.add_error_handler(error_handler)

    # Start polling
    logger.info("Starting Whale Tracker Bot...")
    app.run_polling(allowed_updates=Update.ALL_TYPES)


if __name__ == "__main__":
    main()