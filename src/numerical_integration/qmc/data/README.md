# Bundled lattice data

`kuo-33002.u64le` contains 9,125 little-endian unsigned 64-bit integers, with no
header. These are mathematical generating-vector data, not integration code.
The point generator, randomizations and statistical estimators are independent
Rust implementations in Numerica. No Python package is needed to use the data.

The original rule is Frances Kuo's
`lattice-33002-1024-1048576.9125`: an extensible base-two rank-one rule with
order-three weights (Gamma_1 = Gamma_2 = 1, Gamma_3 = 0.5), constructed for powers
of two from 1,024 through 1,048,576 points. The published range is enforced; it
must not be confused with the integer word size or extrapolated silently.

- Author's publication and description: <https://web.maths.unsw.edu.au/~fkuo/lattice/>
- Related paper: R. Cools, F. Y. Kuo and D. Nuyens, *Constructing embedded lattice
  rules for multivariate integration*, SIAM J. Sci. Comput. 28(6), 2162–2188 (2006).
- Distribution used here: QMCPy, commit
  `0f6d3c28f5fdd7effb1c883bc5d0351d50987427`, file
  `qmcpy/discrete_distribution/lattice/generating_vectors/kuo.lattice-33002-1024-1048576.9125.npy`.
- Source URL: <https://github.com/QMCSoftware/qmcpy/blob/0f6d3c28f5fdd7effb1c883bc5d0351d50987427/qmcpy/discrete_distribution/lattice/generating_vectors/kuo.lattice-33002-1024-1048576.9125.npy>

The third-party data are distributed under QMCPy's Apache License 2.0, included
as `LICENSE-APACHE-2.0`. Copyright 2021 Illinois Institute of Technology.
The surrounding Numerica implementation remains MIT-licensed. This file records
the format change: the original NumPy 1.0 header (128 bytes, dtype `<u8`, shape
`(9125,)`, C order) was removed; every payload byte is unchanged.

Source SHA-256:
`dbb76538e5dc9249f1a8de2b73df6f3d8c4ffec86f07b32df5e481441e712a3b`

Payload SHA-256:
`f1ea2884947828c718ddff18540d55ef5415cfb9aec55c1b88b8e6f50c707fbb`

Reproduce the conversion with `dd if=SOURCE.npy of=kuo-33002.u64le bs=1 skip=128`.
This is a one-time data import; neither builds nor execution download data.
