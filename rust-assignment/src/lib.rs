#[cfg(test)]
mod tests {
    use solana_sdk::signature::{Keypair, Signer};
    use solana_sdk::pubkey::Pubkey;
    
    #[test]
    fn keygen() {
        let kp = Keypair::new();
        println!("You've generated a new Solana wallet: {}\n", kp.pubkey());
        println!("To save your wallet, copy and paste the following into a JSON file:");
        println!("{:?}", kp.to_bytes());
    }

    #[test]
    fn airdrop() {}

    #[test]
    fn transfer_sold() {}
}
