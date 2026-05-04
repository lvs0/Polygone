**Title:** Polygone — A private, post‑quantum P2P network you can run in 5 minutes (Live Demo inside)

**Body:**

Hey r/rust & r/privacy,

I’ve been working on **Polygone**, a post‑quantum privacy network built in Rust. The goal: make private, decentralized communication and storage as easy to run as a single binary.

**What it is now:**
- Post‑quantum crypto (ML‑KEM‑1024, Shamir 4‑of‑7, AES‑256‑GCM, BLAKE3)
- libp2p + Kademlia DHT for peer discovery
- Local Web UI dashboard (http://127.0.0.1:9050) showing node stats, peers, wallet
- A simple HTTP Gateway (HTTP → P2P) so you can expose content from the mesh to classic browsers
- One‑command install (`curl … | bash`) and one‑command start (`polygone start`) that opens the dashboard automatically

**Live Demo:**
We have a free tier node running on Render (sleeps after 15 min, but the keepalive script keeps it mostly awake):
👉 https://polygone-demo.onrender.com

**Why it might interest you:**
- The stack is small, auditable, and Rust‑first.
- We’re building a standard for AI‑weight exchange (“msh” protocol) so agents can share models over P2P instead of central servers.
- It’s MIT‑licensed and designed to be easy to fork and deploy on Fly.io, Railway, or your own VPS.

**What’s next:**
- Finish DHT‑based file storage (Drive) and encrypted messaging (Msg).
- Solidify the msh protocol (NEUROMESH → msh) for federated AI training.
- More redundancy/bridges so free tiers don’t sleep forever.

**Get involved:**
- Repo: https://github.com/lvs0/Polygone
- CONTRIBUTING.md has exact 5‑min deploy steps for Render/Fly/Railway.
- Join Discord: https://discord.gg/kSqe38NbJM

If you care about privacy, Rust, or decentralized AI, I’d love feedback, testers, and contributors. Let’s build a network where privacy is default, not optional.

Thanks for reading!

— Lévy (14), founder
