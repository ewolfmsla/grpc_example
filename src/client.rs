use payments::bitcoin_client::BitcoinClient;
use payments::BtcPaymentRequest;
use tonic::transport::Channel;

pub mod payments {
    tonic::include_proto!("payments");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = BitcoinClient::connect("http://[::1]:50051").await?;

    for bitcoins_to_send in 0..10 {
        make_payment(bitcoins_to_send, &mut client).await?;
    }

    match test_send().await? {
        Some(true) => println!("made it!"),
        _ => println!("failed")
    }

    Ok(())
}

async fn test_send() -> Result<Option<bool>, Box<dyn std::error::Error>> {
    let mut client = BitcoinClient::connect("http://[::1]:50051").await?;

    let request = tonic::Request::new(BtcPaymentRequest {
        from_addr: "123456".to_owned(),
        to_addr: "3232444".to_owned(),
        amount: 2,
    });

    let response = client.send_payment(request).await?;

    println!("TEST SEND RESPONSE: {:?}", response);

    Ok(Some(true))
}

async fn make_payment(
    bitcoins_to_send: u32,
    client: &mut BitcoinClient<Channel>,
) -> Result<(), Box<dyn std::error::Error>> {
    let request = tonic::Request::new(BtcPaymentRequest {
        from_addr: "123456".to_owned(),
        to_addr: "654321".to_owned(),
        amount: bitcoins_to_send + 1,
    });

    let response = client.send_payment(request).await?;

    println!("RESPONSE {}={:?}", bitcoins_to_send + 1, response);

    Ok(())
}
