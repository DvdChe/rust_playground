use clap::Parser;
use std::sync::Arc;
use tokio::sync::Semaphore;
use tokio::{self};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]

struct Args {
    #[arg(short, long)]
    target: std::net::IpAddr,

    #[arg(short, long)]
    start_port: u16,

    #[arg(short, long)]
    end_port: u16,

    #[arg(short, long)]
    duration: u64,
}

async fn scan_port(addr: std::net::IpAddr, port: u16, timeout_ms: u64) -> bool {
    let socket_addr = std::net::SocketAddr::new(addr, port);
    match tokio::time::timeout(
        tokio::time::Duration::from_millis(timeout_ms),
        tokio::net::TcpStream::connect(&socket_addr),
    )
    .await
    {
        Ok(Ok(_)) => true,
        _ => false,
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let semaphore = Arc::new(Semaphore::new(100));
    let args = Args::parse();
    println!(
        "Starting scanning target {} ports from {} to {}",
        args.target, args.start_port, args.end_port
    );

    let mut tasks = Vec::new();

    for port in args.start_port..=args.end_port {
        let permit_clone = semaphore.clone();
        let task = tokio::spawn(async move {
            let _permit = permit_clone.acquire().await.unwrap();
            let is_open = scan_port(args.target, port, args.duration).await;
            if is_open {
                println!("Port {} is open", port);
            }
        });
        tasks.push(task);
    }

    for task in tasks {
        task.await.unwrap();
    }

    println!("Scanning finished");

    Ok(())
}
