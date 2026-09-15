# RELEASED

## [0.1.4] - 2026-09-15

### Fixed

- Fixed `ArrayDeque::from_array` with const-generic and `AssocArraySize`-derived capacities.
  The method now constrains the input array directly with
  `[T; N]: AssocArraySize<Size = S>`, avoiding associated-type normalization
  failures in generic code.

### Changed

- Reworked `ArrayDeque::from_array` to move elements into pre-initialized deque
  storage instead of constructing the underlying `hybrid_array::Array`
  directly.

## [0.1.0] - 2026-09-15

### BREAKINGS

- Fork from generic-arraydeque and replace generic-array to hybrid-array
