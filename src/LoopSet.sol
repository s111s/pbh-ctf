pragma solidity 0.8.28;

contract LoopSet {

    uint256 public value = 0;
    uint256 public lastGasUsed = 0;
    function set(uint256 count) public {
        uint256 startGas = gasleft();
        uint i = 0;
        while (i < count) {
            value = i;
            i++;
        }
        lastGasUsed = startGas - gasleft();
    }
}