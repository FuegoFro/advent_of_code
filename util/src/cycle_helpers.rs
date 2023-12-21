/// Taken from https://math.stackexchange.com/a/3864593/1264446
use std::error::Error;
use std::fmt::Debug;

use num_traits::{Num, Signed};

/// Extended Greatest Common Divisor Algorithm
///
/// Returns:
///     gcd: The greatest common divisor of a and b.
///     s, t: Coefficients such that s*a + t*b = gcd
///
/// Reference:
///     https://en.wikipedia.org/wiki/Extended_Euclidean_algorithm#Pseudocode
fn extended_gcd<N: Num + Copy>(a: N, b: N) -> (N, N, N) {
    let (mut old_r, mut r) = (a, b);
    let (mut old_s, mut s) = (N::one(), N::zero());
    let (mut old_t, mut t) = (N::zero(), N::one());

    while r != N::zero() {
        let quotient = old_r / r;
        let remainder = old_r % r;
        old_r = r;
        r = remainder;
        let new_s = old_s - quotient * s;
        let new_t = old_t - quotient * t;
        old_s = std::mem::replace(&mut s, new_s);
        old_t = std::mem::replace(&mut t, new_t);
    }

    (old_r, old_s, old_t)
}

/// Combine two phased rotations into a single phased rotation
///
/// Returns: combined_period, combined_phase
///
/// The combined rotation is at its reference point if and only if both a and b
/// are at their reference points.
fn combine_phased_rotations<N: Num + Copy>(
    a_period: N,
    a_phase: N,
    b_period: N,
    b_phase: N,
) -> Result<(N, N), Box<dyn Error>> {
    let (gcd, s, _t) = extended_gcd(a_period, b_period);
    let phase_difference = a_phase - b_phase;
    let (pd_mult, pd_remainder) = (phase_difference / gcd, phase_difference % gcd);

    if pd_remainder != N::zero() {
        return Err("Rotation reference points never synchronize.".into());
    }

    let combined_period = a_period / gcd * b_period;
    let combined_phase = (a_phase - s * pd_mult * a_period) % combined_period;
    Ok((combined_period, combined_phase))
}

pub trait FirstCommonCycle<N: Num + Copy + Signed> {
    /// Takes a series of (first_occurrence, second_occurrence) pairs and returns the first time all the cycles align
    fn find_first_common_cycle(self) -> N;
}

impl<N: Num + Copy + Signed + Debug + Ord + PartialOrd, I: Iterator<Item = (N, N)>>
    FirstCommonCycle<N> for I
{
    fn find_first_common_cycle(self) -> N {
        let (period, phase) = self.fold(
            (N::one(), N::zero()),
            |(existing_period, existing_phase), (new_offset, new_period)| {
                combine_phased_rotations(existing_period, existing_phase, new_period, -new_offset)
                    .unwrap()
            },
        );
        eprintln!("{:?}", (period, phase));
        if phase == N::zero() {
            period
        } else {
            let mut result = -phase;
            while result < N::zero() {
                result = result + period;
            }
            result
        }
    }
}

pub fn find_first_common_cycle<N: Num + Copy + Signed>(pairs: impl Iterator<Item = (N, N)>) -> N {
    let (period, phase) = pairs.fold(
        (N::one(), N::zero()),
        |(existing_period, existing_phase), (new_offset, new_period)| {
            combine_phased_rotations(existing_period, existing_phase, new_period, -new_offset)
                .unwrap()
        },
    );
    -phase % period
}
