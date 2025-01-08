import bs58 from "bs58";
const prompt = require("prompt-sync")();

// Take input as comma-separated bytes
const privateKey: string = prompt("Enter the private key (bytes array): ");
const privateKeyBytes = privateKey.split(",").map((byte) => parseInt(byte.trim(), 10));

// Convert byte array to Base58
const base58 = bs58.encode(Buffer.from(privateKeyBytes));
console.log("\n\n\nPrivate Key (Base58): ", base58);
