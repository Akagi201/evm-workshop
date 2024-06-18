// SPDX-License-Identifier: Apache-2.0
pragma solidity 0.8.26;

contract Add {
  uint8 v1 = 128;
  function add(uint8 n) external view returns (uint8) {
    require(n < 128, "n is too large");
    return v1 + n;
  }
}
