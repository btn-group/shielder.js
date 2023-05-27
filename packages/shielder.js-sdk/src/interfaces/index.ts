export interface Deposit {
  token_id: number;
  token_amount: number;
  lead_idx?: number;
  trapdoor?: [number, number, number, number];
  nullifier?: [number, number, number, number];
  note?: [number, number, number, number];
  proof?: string
}

export interface Withdraw {
  deposit: Deposit,
  withdraw_amount: number;
  fee: number;
  merkle_root: [number, number, number, number];
  merkle_path: any;
  recipient: any;
}