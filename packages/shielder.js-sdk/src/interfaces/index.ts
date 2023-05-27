export interface Deposit {
  token_id: number;
  token_amount: number;
  lead_idx?: number;
  trapdoor?: [number, number, number, number];
  nullifier?: [number, number, number, number];
  note?: [number, number, number, number];
  proof?: string
}
