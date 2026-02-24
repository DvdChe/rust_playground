use clap::Parser;
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
    let args = Args::parse();
    println!(
        "Starting scanning target {} ports from {} to {}",
        args.target, args.start_port, args.end_port
    );
    Ok(())
}
