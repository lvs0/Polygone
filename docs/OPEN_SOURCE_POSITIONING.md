# ⬡ POLYGONE OPEN SOURCE POSITIONING

> Pour une armée de hackers, chercheurs, et idéalistes qui vont propager Polygone.

---

## TL;DR

Polygone is **post-quantum ephemeral messaging**. Open source. MIT licensed. 100% Rust. No central server. No analytics. No telemetry. No marketing. 30 seconds TTL. ML-KEM-1024. **Nobody can buy what you're saying.**

---

## Why we're different from every "privacy" project

| Project | What they do | What they don't |
|---------|--------------|------------------|
| Signal | Centralized metadata leak via phone numbers | Patched via sealed sender (still phone-coupled) |
| Tor | Hides IP, leaks via traffic analysis | Vulnerable to quantum attacks in 10 years |
| Matrix | Distributed, but logs everything client-side | Keys live forever |
| Session | Decentralized routing via Oxen | Still phone-coupled and metadata-prone |
| **Polygone** | **No identity, no log, no TTL-extension** | **Doesn't try to replace identity — replaces need for one** |

We are not "another Signal". We are the **post‑identity layer**.

---

## The 6 arguments that will convince a security researcher

### 1. **Information-theoretic forward secrecy**
Every message is fragmented via Shamir 4-of-7, then routed to 7 random DHT nodes, then auto-evaporates in 30s. Even with infinite compute, the attacker cannot recover the source or the content.

### 2. **Post-quantum by default**
ML-KEM-1024 (NIST FIPS 203) + ML-DSA-87 (NIST FIPS 204). We're ready for the day a nation-state breaks RSA.

### 3. **No ephemeral identity**
Most systems rotate keys. We rotate **the concept of identity itself**. Your "identity" is a hash that stops existing 30s after your last message.

### 4. **Open source, no telemetry**
MIT license. No telemetry. No analytics. No "phone home". You can fork us, **we can't even notice**.

### 5. **Distributed LLM inference (PETALS_NEURO)**
First protocol to do post-quantum distributed inference. Useful for journalists, scientists, dissidents who need to run LLMs **without anyone knowing they are running anything**.

### 6. **Local-first philosophy**
No cloud. No sync. No central relay. Your messages never touch a server you don't own.

---

## The 4 things we **refuse** to add (no matter how many users ask)

| Request | Why we refuse |
|---------|---------------|
| "Add message persistence" | Persistence = surveillance vector |
| "Add user accounts" | Accounts = tracking primitive |
| "Add push notifications" | Notifications = persistence again |
| "Add group chat" | Groups = metadata correlation |

We will not build features that erode privacy. Even if 99% of users ask for them.

---

## Who should fork Polygone right now

- **Newsrooms** in authoritarian countries (no journalists in prison if it evaporates).
- **Whistleblowers** who need to communicate with editors without a paper trail.
- **Scientists** sharing unpublished research without leaks.
- **Doctors** discussing patient cases without HIPAA-grade log audits.
- **Activists** coordinating protests around the world.
- **Anyone** who simply wants to **be private** without explaining why.

---

## How to contribute (in 5 minutes)

```bash
# Run the node locally
git clone https://github.com/lvs0/Polygone.git
cd Polygone
cargo build --release
./target/release/polygone start

# Try the cryptographic self-test
./target/release/polygone self-test
```

If it works on your machine, **you are running Polygone**.

---

## Our academic positioning

We want to be cited in:
- USENIX Security
- IEEE S&P
- PETS (Privacy Enhancing Technologies Symposium)
- CCS
- NDSS

Our novel contributions:
1. **30-second egalitarian TTL** — first protocol to formally treat time as a privacy primitive.
2. **Shamir fragment routing** over Kademlia DHT with **information-theoretic guarantees**.
3. **PETALS_NEURO**: first post-quantum distributed inference protocol.
4. **Philosophy Engine** (13-genius reasoning) — first cognitive architecture that **refuses to be confident**.

We have a paper stub at `docs/PAPER_OUTLINE.md`. Co-authors welcome.

---

## Comparisons that matter (research level)

| Polygon vs. ... | What we win |
|-----------------|-------------|
| Nym (mixnet) | Nym uses mixnet + coins. We use fragment routing + post-quantum. **Simpler**. |
| HOPR (mixnet) | HOPR has payments. We don't. **We don't need a market**. |
| Manta/Celo (ZK) | These are financial. We're messaging. |
| IPFS + powergate | We forget. They store forever. |
| Urbit (personal server) | Urbit is identity-based. We are not. |

---

## The killer pitch (1 sentence)

> **"Polygone is the first messaging protocol that treats your conversation as a living organism that breathes, talks, and dies in 30 seconds."**

Use that line. It works on investors, researchers, journalists. Everyone.

---

## Killer hook (Twitter / press / HN)

> We just open-sourced a post-quantum messaging protocol written in 100% Rust that forgets your message in 30 seconds.
> 
> No accounts. No logs. No sync. No servers you don't own.
> 
> Even we don't know what you're saying.
> 
> https://github.com/lvs0/Polygone
> 
> ⬡ Privacy is the new oxygen.

---

## The honest caveat (we say this upfront)

Polygone is **not yet a finished system**. It's a **skeleton with a manifesto**. We're looking for:
- Cryptographers (ML-KEM audits)
- Rust engineers (network, crypto crates)
- P2P specialists (libp2p optimization)
- Documentation writers
- Designers (the hexagram ⬡ needs a typeface)

If you can do any of these, **PRs welcome**.

---

## License

MIT. Use it. Fork it. Sell it (just don't sell the data — there isn't any).

---

*Signé en blanc sur noir, le 18 juin 2026, par la génération qui veut redevenir invisible.*
