The __Publisher__ application reads the messages CSV file, serializes messages to binary format and saves them in the memory mapped file one by one at 1-second intervals.

# Config
n/a

# Build 
`cargo build -p publisher`

# Test
n/a

# Run
`research/bin/publisher.sh <MESSAGES FILE> <MMAP FILE>`. 
Example: 
```
~/work/src/research/bin/publisher.sh ~/work/src/research/tools/publisher/data/arb.csv /dev/shm/arb_msg.mmap
```