"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
var web3_js_1 = require("@solana/web3.js");
var keypair = web3_js_1.Keypair.generate();
console.log("keypair is ", keypair.publicKey.toBase58());
console.log("".concat(keypair.secretKey));
