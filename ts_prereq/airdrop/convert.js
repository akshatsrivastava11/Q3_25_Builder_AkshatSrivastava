"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
var bs58_1 = require("bs58");
var prompt = require('prompt-sync')();
// Convert Base58 private key to a Uint8Array (Solana Wallet format)
function base58ToWallet() {
    var input = prompt("Enter your Base58 private key:");
    if (!input) {
        console.error("No input provided.");
        return new Uint8Array();
    }
    var base58Key = input.trim();
    try {
        var decodedKey = bs58_1.default.decode(base58Key);
        console.log("Decoded Private Key (Uint8Array):", decodedKey);
        return decodedKey;
    }
    catch (error) {
        console.error("Error decoding Base58:", error);
        return new Uint8Array();
    }
}
// Convert Uint8Array (Solana Wallet format) to Base58 (Phantom format)
function walletToBase58() {
    var input = prompt("Enter your private key as a comma-separated byte array: ").trim();
    try {
        var byteArray = new Uint8Array(input.split(",").map(Number));
        var encodedKey = bs58_1.default.encode(byteArray);
        console.log("Base58 Encoded Private Key:", encodedKey);
        return encodedKey;
    }
    catch (error) {
        console.error("Error encoding Base58:", error);
        return "";
    }
}
// Main function
function main() {
    console.log("Choose an option:");
    console.log("1. Convert Base58 to Wallet Byte Array");
    console.log("2. Convert Wallet Byte Array to Base58");
    var choice = prompt("Enter choice (1 or 2): ").trim();
    if (choice === "1") {
        base58ToWallet();
    }
    else if (choice === "2") {
        walletToBase58();
    }
    else {
        console.log("Invalid choice. Exiting.");
    }
}
main();
