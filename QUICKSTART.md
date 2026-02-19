# Space Trader - Quick Start Guide

## 🚀 Get Playing in 2 Minutes

### Step 1: Install Rust
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Step 2: Generate Assets
```bash
cd tools
python3 generate_placeholder_assets.py
cd ..
```

### Step 3: Run
```bash
cargo run
```

That's it! 🎉

## 🎮 First 5 Minutes of Gameplay

### 1. Start the Game
- Press `N` for New Game
- You start at **Acamar** with a **Flea** ship
- You have **1000 credits** and **20 cargo bays**

### 2. Your First Trade
1. Press `T` to open the trading screen
2. Look for **cheap goods** (prices under 10 credits)
3. Press `↑` or `↓` to select a good
4. Press `5` to buy 5 units
5. Fill your cargo hold (buy more)
6. Press `ESC` to return to main screen

### 3. Travel to Another System
1. Press `W` to open the warp screen
2. Look for systems **within fuel range** (your Flea has 20 fuel)
3. Press `↑` or `↓` to select a destination
4. Press `W` or `ENTER` to warp
5. **Each parsec costs 1 fuel!**

### 4. Make Your Profit
1. At the new system, press `T` for trading
2. Press `↑` or `↓` to find the goods you bought
3. Check if the **price is higher** than what you paid
4. Press `A` to sell all of that good
5. **Profit!** 💰

### 5. Keep Trading
- Look for **high-tech systems** - they produce advanced goods cheaper
- **Low-tech systems** pay more for advanced goods
- Watch your **fuel** - you need it to travel!
- Track your **credits** - you need them to buy goods and fuel

## 📊 Understanding the Screens

### Main Screen
```
System: Acamar (TechLevel: 5)
Credits: 1000
Fuel: 20/20
Days: 0
```
- **System**: Your current location
- **Credits**: Your money
- **Fuel**: Current/Max fuel
- **Days**: Game time elapsed

### Trading Screen
```
Good          Price   Cargo   Available
Water         3 cr    0       52
Furs          15 cr   0       12
Food          7 cr    0       38
Ore           10 cr   0       25
```
- **Price**: Cost per unit at this system
- **Cargo**: How many you're carrying
- **Available**: How many the system has to sell

### Warp Screen
```
System        Distance   Fuel Cost
Adahn         8.5        9
Aldebaran     12.3       13
Altair        15.7       16
```
- **Distance**: Parsecs away
- **Fuel Cost**: Fuel needed to reach it

## 💡 Pro Tips

### Trading
- **Buy low, sell high** - Classic trading!
- **Tech levels matter** - High-tech = cheap tech goods
- **Fill your hold** - Don't waste cargo space
- **Track prices** - Remember what you paid

### Travel
- **Plan ahead** - Don't get stranded without fuel!
- **Short hops** - Save fuel by traveling to nearby systems
- **Visit all systems** - Each has different prices
- **Watch range** - Can't warp beyond your fuel capacity

### Economics
1. **Water** - Always cheap, sell anywhere
2. **Narcotics** - High profit but illegal
3. **Robots** - Need high tech to buy, sell to low tech
4. **Machinery** - Mid-level goods, steady profit

### Gameplay Loop
```
Trade (buy goods) → Warp (new system) → Trade (sell goods) → Profit → Repeat
```

## ⌨️ All Controls

### Main Screen
| Key | Action |
|-----|--------|
| `T` | Open trading screen |
| `W` | Open warp screen |
| `S` | Save game |
| `Q` | Quit to main menu |
| `ESC` | Quit to main menu |

### Trading Screen
| Key | Action |
|-----|--------|
| `↑` | Select previous good |
| `↓` | Select next good |
| `B` | Buy 1 unit of selected good |
| `5` | Buy 5 units |
| `S` | Sell 1 unit |
| `A` | Sell ALL of selected good |
| `ESC` | Return to main screen |
| `Q` | Return to main screen |

### Warp Screen
| Key | Action |
|-----|--------|
| `↑` | Select previous system |
| `↓` | Select next system |
| `W` | Warp to selected system |
| `ENTER` | Warp to selected system |
| `ESC` | Return to main screen |
| `Q` | Return to main screen |

## 🎯 Your First Goal

**Make 10,000 credits!**

Here's a simple strategy:
1. **Find cheap goods** (Water, Food, Furs)
2. **Buy as much as you can** (fill cargo hold)
3. **Travel to nearby system** (save fuel)
4. **Sell for profit** (look for higher prices)
5. **Repeat** until you have 10,000 credits

## 🚀 What's Next?

Once you're comfortable with trading:
- **Explore the galaxy** - All 120 systems
- **Visit different tech levels** - See price differences
- **Maximize profits** - Find the best trade routes
- **Track your progress** - Watch your days counter

### Available Now
- ✅ **Fuel purchase** - Buy fuel at systems (`F`)
- ✅ **Ship repairs** - Fix ship hull (`R`)
- ✅ **Ship upgrades/shop** - Buy better ships (`H` / `U`)
- ✅ **Random encounters** - Traders, police, pirates while traveling

### Planned Next
- ⏳ **Combat depth** - Expand encounter outcomes and battle flow
- ⏳ **Special events/quests** - Story-driven missions
- ⏳ **Additional polish** - UI cleanup and module extraction

## ❓ Troubleshooting

### Game won't start
```bash
# Make sure you have Rust installed
rustc --version

# Make sure assets are generated
ls assets/ships/

# Try a clean build
cargo clean && cargo run
```

### Can't buy goods
- Check your **credits** (shown on main screen)
- Check **cargo space** (you might be full)
- Check **available quantity** (system might be out of stock)

### Can't warp
- Check your **fuel** (shown on main screen)
- Check **distance** (system might be too far)
- Make sure you have **enough fuel** for the trip

### Ran out of fuel
Use `F` on the main game screen to purchase fuel when you have enough credits.

## 📚 More Information

- **Full documentation**: [README.md](README.md)
- **Roadmap**: [docs/ROADMAP.md](docs/ROADMAP.md)
- **Current status**: [docs/STATUS.md](docs/STATUS.md)
- **Contributing**: [CONTRIBUTING.md](CONTRIBUTING.md)
- **Original game**: https://github.com/videogamepreservation/spacetrader

---

**Happy Trading! 🚀💰**
