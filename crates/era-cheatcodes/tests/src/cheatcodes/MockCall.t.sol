// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Test, console2 as console} from "../../lib/forge-std/src/Test.sol";
import {Constants} from "./Constants.sol";

contract Mock {
    // function numberA() public pure returns (uint256) {
    //     uint256 amount = 10 - 8;
    //     console.log("foo");
    //     return amount;
    // }

    function numberA() public pure returns (uint256) {
        return 8;
    }
    
    // function numberA() public pure {
    // }
    // function numberA() public pure {
    //     uint256 amount = 10 - 8;
    //     console.log(amount);
    // }
}

contract MockCallTest is Test {
    function testMockGetters() public {
        Mock target = new Mock();

        // pre-mock
        // assertEq(target.numberA(), 1);
        // assertEq(target.numberB(), 2);

        console.log(115792089237316195423570985008687907853269984665640564039457584007913129639935);
        vm.mockCall(
            address(target),
            abi.encodeWithSelector(target.numberA.selector),
            abi.encode(16)
        );

        // post-mock
        uint256 number = target.numberA();
        console.log(number);
        require(number == 16, "numberB failed mock");
    }
}
