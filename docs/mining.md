# Mining Guide

## What is Mining?

Mining is using your computer's CPU to help secure the JunoCash network. In return, you earn JUNO coins as rewards when you find blocks.

**Important:** Mining is competitive. Your chance of finding blocks depends on your CPU power vs the entire network.

## Starting to Mine

### Step 1: Make Sure Node is Synced

Before mining, your node must be fully synced with the blockchain.

Check **Dashboard** or **Mining** page:
- Look for block height matching the network
- Wait until "Syncing" status shows "Online"
- This can take hours on first run

### Step 2: Configure Mining

Go to **Mining** page:

**CPU Threads:**
- More threads = higher hashrate = more chance to find blocks
- But also = more CPU usage and heat
- **Recommended:** Use "Auto" (uses ~50% of your CPU cores)
- Or manually select: 2-4 threads for light mining, max threads for serious mining

**While Mining:**
- Computer will be slower for other tasks
- CPU will run hot (normal, but monitor temps)
- Electricity usage increases

### Step 3: Start Mining

Click the big power button in the center.

When active:
- Button glows purple
- Status shows "Online"
- Hashrate appears below (your mining speed)

**What the numbers mean:**
- **My Hashrate:** Your computer's mining speed (Sol/s or H/s)
- **Network Hashrate:** Everyone's combined speed
- **Difficulty:** How hard it is to find a block (higher = harder)
- **Block Reward:** JUNO you earn per block found

## Understanding Mining Rewards

When you find a block, you get the **Block Reward** (shown on Mining page).

**Important:** Newly mined coins are "immature" for 100 blocks (~2.5 hours). You can't spend them immediately.

### Shielding Mined Coins

Before you can spend mined coins, you must "shield" them (move to private balance).

**Why?** Mined coins are transparent by default. Shielding makes them private and spendable.

## Auto-Shielding

The wallet can automatically shield your mined coins for you.

### How to Enable:

1. On Mining page, find **Auto-Shielding** section
2. Toggle it **ON**
3. Select a **Destination Address** from dropdown
   - Pick an address starting with `j1...` (shielded address)
   - If you don't have one, go to **Receive** page and generate one

**What happens:**
- Every 5 minutes while mining, wallet checks for new mined coins
- Automatically shields them to your chosen address
- You'll see operation IDs in the console log

### Manual Shielding

If you prefer to shield manually:

1. Turn Auto-Shielding **OFF**
2. Click **"Trigger Manual Shield"** button when you want to shield
3. Select destination address first

## Thread Configuration Tips

**For Casual Mining:**
- Use 2-4 threads
- Low impact on daily computer use
- Lower electricity cost
- Still contribute to network

**For Serious Mining:**
- Use max threads (or close to it)
- Dedicate computer to mining
- Monitor CPU temperatures
- Consider cost of electricity vs rewards

**Auto Mode:**
- Safe default (~50% of cores)
- Good balance of mining vs usability
- Recommended for most users

## Monitoring Your Mining

**Hashrate:**
- Measures your mining speed
- Higher = better
- Varies by CPU model
- Example: 100 H/s to 10,000 H/s typical

**Network Stats:**
- Network Hashrate: Total mining power
- Your share = Your Hashrate ÷ Network Hashrate
- Example: 1,000 H/s with 100,000 H/s network = 1% chance per block

**Expected Rewards:**
- Depends on your hashrate vs network
- Block time: ~2.5 minutes average
- Calculate: (Your Hashrate ÷ Network Hashrate) × Blocks per day × Block Reward

## Is Mining Profitable?

Consider:
- Your electricity cost ($/kWh)
- Your CPU power consumption (watts)
- Network difficulty
- JUNO price

**Generally:**
- Home mining is usually not profitable vs electricity cost
- But you're supporting the network
- Good for learning and earning some coins
- Consider it a hobby unless you have free/cheap electricity

## Stopping Mining

Click the power button again to stop.

**Good practice:**
- Stop mining when doing CPU-intensive tasks
- Stop to let computer cool down
- Keep mining if you want to support network 24/7

## Troubleshooting

**Hashrate shows 0:**
- Node might not be synced yet
- Wait a few seconds after starting
- Check node is running

**Mining won't start:**
- Verify node is fully synced (check block height)
- Make sure you have space in data directory
- Check Settings are correct

**Computer too slow while mining:**
- Reduce thread count
- Use 1-2 threads
- Or stop mining during work

**Coins not appearing:**
- Check they're shielded (mined coins need shielding first)
- Wait for 100 confirmations (~2.5 hours)
- View balance in Dashboard after shielding

## Advanced: Command Line Mining

If you prefer terminal:

```bash
# Start mining with 4 threads
junocash-cli setgenerate true 4

# Stop mining
junocash-cli setgenerate false

# Check mining status
junocash-cli getmininginfo
```

But the wallet makes this easier with the UI.
