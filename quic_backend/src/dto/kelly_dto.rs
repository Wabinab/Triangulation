// We only keep latest 100 transactions for kelly criterion. 
// Anything new will "shift" out position 0's and throw it away. 
use crate::*;

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct SubmitKelly {
  pub(crate) filename: String,
  pub(crate) stage_index: usize,
  pub(crate) kelly_index: Option<usize>,
  pub(crate) transactions: Option<Vec<Transaction>>
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Transaction {
  pub(crate) coin: String,
  pub(crate) buy: bool,  // true = buy, false = sell. 
  pub(crate) price: Decimal,
  pub(crate) amt: Decimal,
  pub(crate) price_1: Decimal,
  pub(crate) amt_1: Decimal,
  pub(crate) pred_prob: Decimal
}

