# LUMINAR
**Linux Micro-Inventory & Node Audit Runtime**

A lightweight, stateless Rust binary that you can pipe to any remote node to instantly audit its security posture and GPU health, returning results in high-speed JSON.

### Quick Start
```bash
cargo build --release
./target/release/luminar
```

## Usage Examples

### 1. UDP Streaming to a Central Logger
Stream node health to a centralized collector.

**Terminal 1 (The Listener):**
```bash
# Listening for the audit payload on port 9000
nc -u -l 9000
```

**Terminal 2 (The Node):**
```bash
# Run the audit and pipe the minified JSON to the listener
luminar | nc -u 127.0.0.1 9000
```

**Resulting Output at Listener:**
```json
{"gpu_telemetry":{"attached_devices":1,"utilization_rates":[42],"temperatures":[68]},"rogue_processes":[],"critical_files_secure":true,"load_average":[1.45,1.10]}
```

### 2. Massive Parallel Cluster Audit

Use LUMINAR to audit 100+ nodes simultaneously using `pdsh` (Parallel Distributed Shell) or a simple `ssh` loop. You execute LUMINAR across a cluster without pre-installing the binary.

**The "Stateless Push" Command:**
```bash
# Push the binary to a remote node's RAM, execute, and return results locally
cat ./target/release/luminar | ssh node-01 "cat > /tmp/l && chmod +x /tmp/l && /tmp/l && rm /tmp/l"
```

**SSH loop (Looping through a node list):**
```bash
for node in $(cat gpu_nodes.txt); do
    echo -n "$node: "
    cat ./target/release/luminar | ssh $node "cat > /tmp/l && chmod +x /tmp/l && /tmp/l && rm /tmp/l"
done
```
