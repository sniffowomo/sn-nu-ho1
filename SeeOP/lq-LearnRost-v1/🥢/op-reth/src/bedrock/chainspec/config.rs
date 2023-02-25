/// Optimism configuration
pub struct OptimismConfig {
    /// Configured elasticity multiplier as defined in [EIP-1559](https://eips.ethereum.org/EIPS/eip-1559).
    pub eip1559_elasticity: u64,
    /// Configured base fee max change denominator as defined in [EIP-1559](https://eips.ethereum.org/EIPS/eip-1559).
    pub eip1559_denominator: u64,
}
