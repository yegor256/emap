// SPDX-FileCopyrightText: Copyright (c) 2023 Yegor Bugayenko
// SPDX-License-Identifier: MIT

#[inline]
#[must_use]
pub fn calc_capacity_ub(base: usize, reg_size: usize) -> usize {
    let cap = base.max(reg_size);
    cap + ((reg_size - (cap % reg_size)) % reg_size)
}

#[test]
fn test_calc_capacity_ub() {
    assert_eq!(calc_capacity_ub(4, 16), 16);
    assert_eq!(calc_capacity_ub(24, 16), 32);
    assert_eq!(calc_capacity_ub(100, 32), 128);
    assert_eq!(calc_capacity_ub(32, 32), 32);
}
