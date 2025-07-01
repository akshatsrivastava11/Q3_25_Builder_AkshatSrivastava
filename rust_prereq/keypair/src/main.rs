use bs58;
use solana_client::rpc_client::RpcClient;
use solana_program::hash::hash;
use solana_program::{pubkey::Pubkey, system_instruction::transfer};
use solana_sdk::instruction::AccountMeta;
use solana_sdk::instruction::Instruction;
use solana_sdk::message::Message;
use solana_sdk::system_program;
use solana_sdk::sysvar::instructions::Instructions;
use solana_sdk::transaction::Transaction;
use solana_sdk::{
    native_token::LAMPORTS_PER_SOL,
    signature::{Keypair, Signer, read_keypair_file},
};
use std::io::{self, BufRead};

use std::str::FromStr;
fn generateKeypair() {
    let kp = Keypair::new();
    println!(
        "You've generated a new Solana wallet: {}",
        kp.pubkey().to_string()
    );
    println!("");
    println!("To save your wallet, copy and paste the following into a JSON file:");
    println!("{:?}", kp.to_bytes());
}

const RPC_URL: &str =
    "https://turbine-solanad-4cde.devnet.rpcpool.com/9a9da9cf-6db1-47dc-839a-55aca5
c9c80a";

fn claim_airdrop() {
    // Import our keypair
    let keypair = read_keypair_file("dev-wallet.json").expect("Couldn't find wallet file");

    let client = RpcClient::new(RPC_URL);

    match client.request_airdrop(&keypair.pubkey(), LAMPORTS_PER_SOL * 2) {
        Ok(sig) => {
            println!("Success! Check your TX here:");
            println!("https://explorer.solana.com/tx/{}?cluster=devnet", sig);
        }
        Err(err) => {
            println!("Airdrop failed: {}", err);
        }
    }
}
fn transfer_sol() {
    let keypair = read_keypair_file("dev-wallet.json").expect("Couldn't find wallet file");
    let pubkey = keypair.pubkey();
    let message_bytes = b"I verify my Solana Keypair!";
    let sig = keypair.sign_message(message_bytes);
    let sig_hashed = hash(sig.as_ref());
    match sig.verify(&pubkey.to_bytes(), message_bytes) {
        true => println!("Signature verified"),
        false => println!("Verification failed"),
    };
    let to_pubkey = Pubkey::from_str("DLkK7oCoDct33byQ3uiLfSVTZTM988EJDksjfte7UN6m").unwrap();
    let rpc_client = RpcClient::new(RPC_URL);
    let recent_blockhash = rpc_client
        .get_latest_blockhash()
        .expect("Failed to get recent blockhash");
    let balance = rpc_client
        .get_balance(&keypair.pubkey())
        .expect("Failed to get balance");
    let message = Message::new_with_blockhash(
        &[transfer(&keypair.pubkey(), &to_pubkey, balance)],
        Some(&keypair.pubkey()),
        &recent_blockhash,
    );
    let fee = rpc_client
        .get_fee_for_message(&message)
        .expect("Failed to get fee calculator");
    let transaction = Transaction::new_signed_with_payer(
        &[transfer(&keypair.pubkey(), &to_pubkey, balance - fee)],
        Some(&keypair.pubkey()),
        &vec![&keypair],
        recent_blockhash,
    );

    let signature = rpc_client
        .send_and_confirm_transaction(&transaction)
        .expect("Failed to send transaction");

    println!(
        "Success! Check out your TX here: https://explorer.solanause solana_sdk::system_program;
.com/tx/{}/?cluster=devnet",
        signature
    )
}

fn Turbin3main() {
    let rpc_client = RpcClient::new(RPC_URL);
    let signer = read_keypair_file("dev-wallet.json").expect("Couldn't find wallet file");
    let mint = Keypair::new();
    let turbin3_prereq_program =
        Pubkey::from_str("TRBZyQHB3m68FGeVsqTK39Wm4xejadjVhP5MAZaKWDM").unwrap();
    let collection = Pubkey::from_str("5ebsp5RChCGK7ssRZMVMufgVZhd2kFbNaotcZ5UvytN2").unwrap();
    let mpl_core_program =
        Pubkey::from_str("CoREENxT6tW1HoK8ypY1SxRMZTcVPm7R94rH4PZNhX7d").unwrap();
    let system_program = system_program::id();
    let signer_pubkey = signer.pubkey();
    let seeds = &[b"prereqs", signer_pubkey.as_ref()];

    let (prereq_pda, _bump) = Pubkey::find_program_address(seeds, &turbin3_prereq_program);

    let authoritySeeds = &[b"collection", collection.as_ref()];
    let data = vec![77, 124, 82, 163, 21, 133, 181, 206];
    let (authorityPda, bump) =
        Pubkey::find_program_address(authoritySeeds, &turbin3_prereq_program);
    let accounts = vec![
        AccountMeta::new(signer.pubkey(), true),
        AccountMeta::new(prereq_pda, false),
        AccountMeta::new(mint.pubkey(), true),
        AccountMeta::new(collection, false),
        AccountMeta::new_readonly(authorityPda, false),
        AccountMeta::new_readonly(mpl_core_program, false), // mpl core program
        AccountMeta::new_readonly(system_program, false),
    ];
    let block_hash = rpc_client
        .get_latest_blockhash()
        .expect("Failed to get blockhash");
    let instruction = Instruction {
        program_id: turbin3_prereq_program,
        accounts,
        data: data,
    };
    let transaction = Transaction::new_signed_with_payer(
        &[instruction],
        Some(&signer.pubkey()),
        &[&signer, &mint],
        block_hash,
    );
    println!("{}",prereq_pda);

    // let signature = rpc_client
    //     .send_and_confirm_transaction(&transaction)
    //     .expect("Failed to send transaction");
    // println!(
    //     "Success! Check out your TX here:\nhttps://explorer.solana.com/tx/{}/?cluster=devnet",
    //     signature
    // );
}

fn main() {
    // generateKeypair();
    // claim_airdrop();
    // transfer_sol();
    // base58_to_wallet();

    Turbin3main();
}
