/// 
/// Number of combinations: 133784560
///
/// Hashing:
/// Card 6 bits total:
///     4 bits for rank, 2 ^ 4 = 16, only 13 are used because we have 13 diff ranks
///     2 bits for suit, 2 ^ 2 = 4, one value for each suit
/// In fact 1 byte (8 bits) is used
///
/// Hand 7 * 8 = 56 bits
/// In fact 64 bits (8 bytes) is used (u64)
/// 
/// Table size = 8 * 133784560 = 1 070 276 480 bytes ~ 1 GB
///  
/// 2  - 0
/// 3  - 1
/// 4  - 2
/// 5  - 3
/// 6  - 4
/// 7  - 5
/// 8  - 6
/// 9  - 7
/// 10 - 8
/// J  - 9
/// Q  - 10
/// K  - 11
/// A  - 12
/// 
/// s - 0
/// d - 1
/// c - 2
/// h - 3 
/// 
/// 
/// 
///
pub mod lookup_table_generation;
 