// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Test, console2 as console} from "../../lib/forge-std/src/Test.sol";

contract MultiVMBasicTest is Test {
    /// USDC TOKEN
    uint256 constant TOKEN_DECIMALS = 6;

    address constant ERA_TOKEN_ADDRESS = 0x3355df6D4c9C3035724Fd0e3914dE96A5a83aaf4;
    uint256 constant ERA_FORK_BLOCK = 19579636;

    address constant ETH_TOKEN_ADDRESS = 0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48;
    uint256 constant ETH_FORK_BLOCK = 19225195;

    uint256 forkEra;
    uint256 forkEth;

    // We need a way to persist EVM forks and state from `setUp()`, until then we call `_setUp()` manually
    function _setUp() public {
        forkEra = vm.createFork("mainnet", ERA_FORK_BLOCK);
        forkEth = vm.createFork("https://eth-mainnet.alchemyapi.io/v2/Lc7oIGYeL_QvInzI0Wiu_pOZZDEKBrdf", ETH_FORK_BLOCK);
    }

    function _verifyToken(address tokenAddress) public {
        (bool success, bytes memory data) = tokenAddress.call(abi.encodeWithSignature("decimals()"));
        require(success, "decimals() failed");
        uint256 decimals = uint256(bytes32(data));
        require(decimals == 6, "decimals() not 6");
    }

    function testSmoke() public {
        _setUp();
        console.log(block.number);

        // console.log("check era");
        vm.selectFork(forkEra);
        console.log(block.number, ERA_FORK_BLOCK);
        // _verifyToken(ERA_TOKEN_ADDRESS);

        // console.log("check evm");
        // forkEth = vm.createFork("https://eth-mainnet.alchemyapi.io/v2/Lc7oIGYeL_QvInzI0Wiu_pOZZDEKBrdf", ETH_FORK_BLOCK);
        vm.selectFork(forkEth);
        console.log(block.number, ETH_FORK_BLOCK);
        // _verifyToken(ETH_TOKEN_ADDRESS);
    }
}