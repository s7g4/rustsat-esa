# Development Journal - RustSat ESA Project

## Week 1 (Started Nov 1, 2024)
**Goal**: Basic SpaceCAN frame implementation

**Monday**: 
- Set up project structure
- Started reading ESA-ECSS-E-ST-50-05C standard
- This document is DENSE. 200+ pages of space communication specs.

**Tuesday**: 
- Implemented basic frame structure
- Realized I forgot about priority fields (rookie mistake)
- Professor mentioned this is critical for space missions

**Wednesday**: 
- Added CRC validation attempt
- My CRC implementation is definitely wrong, frames are getting corrupted
- Need to research proper CRC16 algorithms

**Friday**: 
- Finally got CRC working after finding reference implementation
- Spent way too long debugging endianness issues
- Note to self: Always test on different architectures

**Weekend**: 
- Reading more LibreCube code to understand their approach
- Their SpaceCAN implementation is much more sophisticated than mine

## Week 2 (Started Nov 8, 2024)
**Goal**: Add network layer and error handling

**Monday**: 
- Started mesh networking implementation
- This is harder than I thought - routing in space is complex
- Satellites move fast, topology changes constantly

**Tuesday**: 
- Found bug in routing algorithm, was causing infinite loops
- Added debug prints everywhere to trace the issue
- Fixed by adding hop count limit

**Wednesday**: 
- Professor suggested adding Reed-Solomon error correction
- Space environment is much noisier than terrestrial networks
- Research: Reed-Solomon vs other FEC codes for space

**Thursday**: 
- Implemented basic Reed-Solomon encoding
- Performance is terrible, need to optimize
- Maybe I should use a library instead of rolling my own

**Friday**: 
- Switched to using `reed-solomon-erasure` crate
- Much better performance, but need to understand the parameters
- Different correction capabilities for different orbit altitudes
