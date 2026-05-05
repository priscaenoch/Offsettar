import { Keypair, Networks, TransactionBuilder, BASE_FEE } from "@stellar/stellar-sdk";

export class StellarHelper {
  static network = Networks.TESTNET;

  static generateKeypair() {
    return Keypair.random();
  }

  static isValidAddress(address: string): boolean {
    try {
      Keypair.fromPublicKey(address);
      return true;
    } catch {
      return false;
    }
  }
}
