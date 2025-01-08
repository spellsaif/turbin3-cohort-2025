import bs58 from "bs58";

const prompt = require("prompt-sync")();

//Taking private key as input
const privateKey: string = prompt("Enter the private key (should be bytes array): ");

//parse the private key to bytes
const privateKeyBytes = privateKey.split(",").map((byte) => parseInt(byte.trim(), 10));

//Converting to base58
const base58 = bs58.encode(Buffer.from(privateKeyBytes));

console.log("\n\n\nPrivate Key(Base58): ", base58);








