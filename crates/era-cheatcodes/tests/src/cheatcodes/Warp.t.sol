// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Test, console2 as console} from "../../lib/forge-std/src/Test.sol";
import {Constants} from "./Constants.sol";
import {juanVm} from "./Juan.sol";

contract CheatcodeWarpTest is Test {
    uint256 constant NEW_BLOCK_TIMESTAMP = uint256(10000);

    function testWarp() public {
        uint256 initialTimestamp = block.timestamp;
        console.log("timestamp before:", initialTimestamp);
        
        //this does not work
        juanVm.warp(NEW_BLOCK_TIMESTAMP);

        //this works
        (bool success, ) = Constants.CHEATCODE_ADDRESS.call(
            abi.encodeWithSignature("warp(uint256)", NEW_BLOCK_TIMESTAMP)
        );
    }
}
