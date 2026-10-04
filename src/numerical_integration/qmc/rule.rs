use crate::domains::integer::gcd_unsigned;

use super::QmcError;

/// Published construction range of the bundled extensible Kuo rule.
pub const KUO_MIN_POINTS: u64 = 1 << 10;
pub const KUO_MAX_POINTS: u64 = 1 << 20;
pub const KUO_MAX_DIMENSION: usize = 9125;
const KUO_DATA: &[u8] = include_bytes!("data/kuo-33002.u64le");

/// Provenance of a generating vector. All point generation is implemented in Rust.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RuleSource {
    Supplied,
    /// Kuo's extensible order-three-weight rule, published for 2^10..=2^20 points.
    Kuo33002,
}

/// A rank-one lattice `{i z / n}` for `i = 0..n`.
///
/// Moduli are limited to 2^53 so lattice indices remain exactly representable
/// in `f64`. Integer products are reduced before conversion to floating point.
/// Each generator component must be coprime to the modulus. A caller-supplied
/// vector is accepted without making a claim about its integration quality.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(try_from = "RuleConfig", into = "RuleConfig"))]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Rank1Rule {
    modulus: u64,
    generator: Vec<u64>,
    source: RuleSource,
}

impl Rank1Rule {
    pub fn new(modulus: u64, generator: Vec<u64>) -> Result<Self, QmcError> {
        if !(2..=1 << 53).contains(&modulus) {
            return Err(QmcError::InvalidRule("modulus must be in 2..=2^53".into()));
        }
        if generator.is_empty() {
            return Err(QmcError::InvalidRule("dimension must be positive".into()));
        }
        let generator: Vec<_> = generator.into_iter().map(|z| z % modulus).collect();
        if generator.iter().any(|&z| gcd_unsigned(z, modulus) != 1) {
            return Err(QmcError::InvalidRule(
                "every generator component must be coprime to the modulus".into(),
            ));
        }
        Ok(Self {
            modulus,
            generator,
            source: RuleSource::Supplied,
        })
    }

    /// Load the published Kuo vector for an exact power-of-two point count.
    ///
    /// Counts outside the published range are rejected rather than silently
    /// rounded or extrapolated. Supply another vector with [`Self::new`] for
    /// other counts. Provenance and the data license are in `qmc/data/README.md`.
    pub fn kuo(points: u64, dimension: usize) -> Result<Self, QmcError> {
        if !points.is_power_of_two() || !(KUO_MIN_POINTS..=KUO_MAX_POINTS).contains(&points) {
            return Err(QmcError::InvalidRule(
                "Kuo rule requires a power of two in 1024..=1048576".into(),
            ));
        }
        if !(1..=KUO_MAX_DIMENSION).contains(&dimension) {
            return Err(QmcError::InvalidRule(
                "Kuo dimension must be in 1..=9125".into(),
            ));
        }
        let generator = KUO_DATA
            .chunks_exact(8)
            .take(dimension)
            .map(|bytes| u64::from_le_bytes(bytes.try_into().unwrap()) % points)
            .collect();
        Ok(Self {
            modulus: points,
            generator,
            source: RuleSource::Kuo33002,
        })
    }

    pub fn points(&self) -> u64 {
        self.modulus
    }
    pub fn dimension(&self) -> usize {
        self.generator.len()
    }
    pub fn generator(&self) -> &[u64] {
        &self.generator
    }
    pub fn source(&self) -> RuleSource {
        self.source
    }

    pub(super) fn numerator(&self, index: u64, axis: usize) -> u64 {
        if self.modulus.is_power_of_two() {
            index.wrapping_mul(self.generator[axis]) & (self.modulus - 1)
        } else {
            ((index as u128 * self.generator[axis] as u128) % self.modulus as u128) as u64
        }
    }
}

#[cfg(feature = "serde")]
#[derive(serde::Serialize, serde::Deserialize)]
struct RuleConfig {
    modulus: u64,
    generator: Vec<u64>,
    source: RuleSource,
}

#[cfg(feature = "serde")]
impl From<Rank1Rule> for RuleConfig {
    fn from(rule: Rank1Rule) -> Self {
        Self {
            modulus: rule.modulus,
            generator: rule.generator,
            source: rule.source,
        }
    }
}

#[cfg(feature = "serde")]
impl TryFrom<RuleConfig> for Rank1Rule {
    type Error = QmcError;
    fn try_from(config: RuleConfig) -> Result<Self, Self::Error> {
        let rule = Self::new(config.modulus, config.generator)?;
        if config.source == RuleSource::Kuo33002 {
            let known = Self::kuo(rule.points(), rule.dimension())?;
            if known.generator != rule.generator {
                return Err(QmcError::InvalidRule(
                    "Kuo provenance does not match vector".into(),
                ));
            }
            Ok(known)
        } else {
            Ok(rule)
        }
    }
}
