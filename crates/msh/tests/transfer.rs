use blake3;
use msh::{Envelope, Blake3Hash};
use std::time::Instant;
use tokio::sync::mpsc;

#[tokio::test]
async fn test_announce_and_transfer_large_fake() {
    let (tx_announce, mut rx_announce) = mpsc::channel(10);
    let (tx_transfer, mut rx_transfer) = mpsc::channel(10);

    // Simuler un "modèle" de 1 Mo de données
    let fake_data = vec![13u8; 1_048_576];
    let fake_hash = Blake3Hash::from(
        blake3::hash(&fake_data).to_string()
    );

    // Annonce
    let announce = Envelope {
        t: "announce".to_string(),
        id: "test-large-1".to_string(),
        model_id: Some(fake_hash.clone()),
        chunk: None,
        payload: None,
        sig: Some("sig_fake".to_string()),
        nonce: "nonce1".to_string(),
        timestamp: 1234567890,
    };
    tx_announce.send(announce.clone()).await.unwrap();

    // Réception annonce
    let rec = rx_announce.recv().await.unwrap();
    assert_eq!(rec.model_id, Some(fake_hash.clone()));

    // Transfert
    let start = Instant::now();
    let transfer = Envelope {
        t: "transfer".to_string(),
        id: "xf-1".to_string(),
        model_id: Some(fake_hash.clone()),
        chunk: Some(msh::ChunkInfo {
            offset: 0,
            size: 1_048_576,
            total: 1_048_576,
            hash: blake3::hash(&fake_data).to_string(),
        }),
        payload: Some(fake_data.clone()),
        sig: Some("sig_xf".to_string()),
        nonce: "nonce3".to_string(),
        timestamp: 1234567910,
    };
    tx_transfer.send(transfer.clone()).await.unwrap();
    let got = rx_transfer.recv().await.unwrap();
    let elapsed = start.elapsed();

    assert_eq!(got.t, "transfer");
    assert_eq!(got.model_id, Some(fake_hash));
    let p = got.payload.unwrap();
    assert_eq!(p.len(), 1_048_576);
    assert_eq!(blake3::hash(&p).to_string(), blake3::hash(&fake_data).to_string());

    println!("[TEST] Transfert simulé 1 Mo - latence: {:?}", elapsed);
}
