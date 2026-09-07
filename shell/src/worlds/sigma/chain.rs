//! The real money: this body's balance ON THE CHAIN, and the flow that
//! moves it. Everything here talks to the first configured network
//! (pussy) over its native endpoints: `GET /balance/<neuron>` for the
//! state, `POST /v1/pay` for transfers. One signal, one block — the pay
//! response carries the height and root it finalized in, and that pair
//! is shown as the receipt.
//!
//! The subsidy side of the loop (proofs -> testpussy) lives on the node
//! per tru/specs/rewards.md §8; here is where the earned balance becomes
//! visible and spendable.

use std::sync::{Arc, Mutex};

use bevy::prelude::*;

/// What the chain last told us. Threads write, the page reads.
#[derive(Clone, Debug, Default)]
pub struct ChainMoneyState {
    pub balance: u64,
    pub supply: u64,
    pub height: u64,
    /// The last receipt: "paid N to X - block h=.. root .."
    pub receipt: String,
    pub error: String,
    pub busy: bool,
    /// Bumped on every update so the page knows to repaint.
    pub version: u64,
}

#[derive(Resource, Clone, Default)]
pub struct ChainMoney(pub Arc<Mutex<ChainMoneyState>>);

impl ChainMoney {
    pub fn snapshot(&self) -> ChainMoneyState {
        self.0.lock().map(|s| s.clone()).unwrap_or_default()
    }

    /// Ask the chain for this neuron's balance, off-thread.
    pub fn refresh(&self, url: String, neuron_hex: String) {
        let slot = self.0.clone();
        if let Ok(mut s) = slot.lock() {
            s.busy = true;
            s.version += 1;
        }
        std::thread::Builder::new()
            .name("sigma-balance".into())
            .spawn(move || {
                let agent = crate::worlds::body::networks::agent();
                let out = agent
                    .get(&format!("{url}/balance/{neuron_hex}"))
                    .call()
                    .ok()
                    .and_then(|mut r| r.body_mut().read_to_string().ok());
                let mut s = slot.lock().expect("chain money");
                s.busy = false;
                s.version += 1;
                match out {
                    Some(body) => {
                        let field = |key: &str| -> u64 {
                            body.lines()
                                .find(|l| l.trim_start().starts_with(key))
                                .and_then(|l| l.split_once(':'))
                                .and_then(|(_, v)| v.trim().parse().ok())
                                .unwrap_or(0)
                        };
                        s.balance = field("balance");
                        s.supply = field("supply");
                        s.height = field("height");
                        s.error.clear();
                    }
                    None => s.error = "chain unreachable".into(),
                }
            })
            .expect("spawn sigma-balance");
    }

    /// Send `amount` to `to` (label or hex), then re-read the balance.
    /// The receipt shows the block the pay finalized in — that IS the
    /// finality: one signal, one block, no waiting period.
    pub fn pay(&self, url: String, neuron_hex: String, to: String, amount: u64) {
        let slot = self.0.clone();
        let me = self.clone();
        if let Ok(mut s) = slot.lock() {
            s.busy = true;
            s.version += 1;
        }
        std::thread::Builder::new()
            .name("sigma-pay".into())
            .spawn(move || {
                let agent = crate::worlds::body::networks::agent();
                let body = serde_json::json!({
                    "neuron": neuron_hex,
                    "to": to,
                    "amount": amount,
                });
                let resp = agent
                    .post(&format!("{url}/v1/pay"))
                    .send_json(&body)
                    .ok()
                    .and_then(|mut r| r.body_mut().read_json::<serde_json::Value>().ok());
                {
                    let mut s = slot.lock().expect("chain money");
                    s.busy = false;
                    s.version += 1;
                    match resp {
                        Some(v) if v.get("ok").and_then(|x| x.as_bool()) == Some(true) => {
                            let h = v.get("height").and_then(|x| x.as_u64()).unwrap_or(0);
                            let root = v
                                .get("root")
                                .and_then(|x| x.as_str())
                                .unwrap_or("")
                                .chars()
                                .take(8)
                                .collect::<String>();
                            s.receipt =
                                format!("paid {amount} to {to} - final in block h={h} {root}..");
                            s.error.clear();
                        }
                        Some(v) => {
                            s.error = v
                                .get("error")
                                .and_then(|x| x.as_str())
                                .unwrap_or("pay refused")
                                .to_string();
                        }
                        None => s.error = "chain unreachable".into(),
                    }
                }
                me.refresh(url, neuron_hex);
            })
            .expect("spawn sigma-pay");
    }
}

/// The identity neuron as the chain spells it.
pub fn neuron_hex(who: &crate::worlds::identity::Identity) -> String {
    who.neuron.iter().map(|b| format!("{b:02x}")).collect()
}

/// First network's URL, if any is configured.
pub fn chain_url(hub: &crate::worlds::body::networks::NetHub) -> Option<String> {
    hub.states
        .lock()
        .ok()
        .and_then(|v| v.first().map(|n| n.url.clone()))
}
