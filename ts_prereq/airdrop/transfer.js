"use strict";
var __awaiter = (this && this.__awaiter) || function (thisArg, _arguments, P, generator) {
    function adopt(value) { return value instanceof P ? value : new P(function (resolve) { resolve(value); }); }
    return new (P || (P = Promise))(function (resolve, reject) {
        function fulfilled(value) { try { step(generator.next(value)); } catch (e) { reject(e); } }
        function rejected(value) { try { step(generator["throw"](value)); } catch (e) { reject(e); } }
        function step(result) { result.done ? resolve(result.value) : adopt(result.value).then(fulfilled, rejected); }
        step((generator = generator.apply(thisArg, _arguments || [])).next());
    });
};
var __generator = (this && this.__generator) || function (thisArg, body) {
    var _ = { label: 0, sent: function() { if (t[0] & 1) throw t[1]; return t[1]; }, trys: [], ops: [] }, f, y, t, g = Object.create((typeof Iterator === "function" ? Iterator : Object).prototype);
    return g.next = verb(0), g["throw"] = verb(1), g["return"] = verb(2), typeof Symbol === "function" && (g[Symbol.iterator] = function() { return this; }), g;
    function verb(n) { return function (v) { return step([n, v]); }; }
    function step(op) {
        if (f) throw new TypeError("Generator is already executing.");
        while (g && (g = 0, op[0] && (_ = 0)), _) try {
            if (f = 1, y && (t = op[0] & 2 ? y["return"] : op[0] ? y["throw"] || ((t = y["return"]) && t.call(y), 0) : y.next) && !(t = t.call(y, op[1])).done) return t;
            if (y = 0, t) op = [op[0] & 2, t.value];
            switch (op[0]) {
                case 0: case 1: t = op; break;
                case 4: _.label++; return { value: op[1], done: false };
                case 5: _.label++; y = op[1]; op = [0]; continue;
                case 7: op = _.ops.pop(); _.trys.pop(); continue;
                default:
                    if (!(t = _.trys, t = t.length > 0 && t[t.length - 1]) && (op[0] === 6 || op[0] === 2)) { _ = 0; continue; }
                    if (op[0] === 3 && (!t || (op[1] > t[0] && op[1] < t[3]))) { _.label = op[1]; break; }
                    if (op[0] === 6 && _.label < t[1]) { _.label = t[1]; t = op; break; }
                    if (t && _.label < t[2]) { _.label = t[2]; _.ops.push(op); break; }
                    if (t[2]) _.ops.pop();
                    _.trys.pop(); continue;
            }
            op = body.call(thisArg, _);
        } catch (e) { op = [6, e]; y = 0; } finally { f = t = 0; }
        if (op[0] & 5) throw op[1]; return { value: op[0] ? op[1] : void 0, done: true };
    }
};
Object.defineProperty(exports, "__esModule", { value: true });
var web3_js_1 = require("@solana/web3.js");
var fs = require("fs");
var wallet = JSON.parse(fs.readFileSync("./dev-wallet.json", { encoding: 'utf-8' }));
var to = new web3_js_1.PublicKey("DLkK7oCoDct33byQ3uiLfSVTZTM988EJDksjfte7UN6m");
var keypair = web3_js_1.Keypair.fromSecretKey(new Uint8Array(wallet));
var connection = new web3_js_1.Connection("https://api.devnet.solana.com ", "confirmed");
(function () { return __awaiter(void 0, void 0, void 0, function () {
    var balance, txs, _a, fee, signature, error_1;
    return __generator(this, function (_b) {
        switch (_b.label) {
            case 0:
                _b.trys.push([0, 5, , 6]);
                return [4 /*yield*/, connection.getBalance(keypair.publicKey)];
            case 1:
                balance = _b.sent();
                txs = new web3_js_1.Transaction().add(web3_js_1.SystemProgram.transfer({
                    fromPubkey: keypair.publicKey,
                    toPubkey: to,
                    lamports: balance
                }));
                _a = txs;
                return [4 /*yield*/, connection.getLatestBlockhash()];
            case 2:
                _a.recentBlockhash = (_b.sent()).blockhash;
                txs.feePayer = keypair.publicKey;
                return [4 /*yield*/, connection.getFeeForMessage(txs.compileMessage(), 'confirmed')];
            case 3:
                fee = (_b.sent()).value;
                if (!fee)
                    return [2 /*return*/];
                txs.instructions.pop();
                txs.add(web3_js_1.SystemProgram.transfer({
                    fromPubkey: keypair.publicKey,
                    toPubkey: to,
                    lamports: balance - fee
                }));
                return [4 /*yield*/, (0, web3_js_1.sendAndConfirmTransaction)(connection, txs, [keypair])];
            case 4:
                signature = _b.sent();
                console.log("Success! Check out your TX here:\nhttps://explorer.solana.com/tx/".concat(signature, "?cluster=devnet"));
                return [3 /*break*/, 6];
            case 5:
                error_1 = _b.sent();
                console.log("Oops Something went wrong ".concat(error_1), error_1);
                return [3 /*break*/, 6];
            case 6: return [2 /*return*/];
        }
    });
}); })();
