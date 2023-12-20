// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Test, console2 as console} from "../../lib/forge-std/src/Test.sol";
import {Constants} from "./Constants.sol";

JuanContract constant juanVm = JuanContract(0x7109709ecfa91A80626Ff3989d68F67F5b1dD120);

interface JuanContract{
    function warp(uint256 timestamp) external;
}