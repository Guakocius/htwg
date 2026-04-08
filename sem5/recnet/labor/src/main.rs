
use std::collections::HashMap;    

use surge_ping;
use gnuplot::{AxesCommon, Figure};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let payload = [0; 8];
    let count: u8 = 100;

    let servers = HashMap::from([
        ("Labor", "127.0.0.1"),
        ("htwg-konstanz.de", "141.37.20.31"),
        ("volteuropa.org", "104.26.13.146"),
        ("homeaffairs.gov.au", "2.21.239.17"),
        ("whitehouse.gov", "192.0.66.51"),
    ]);

    let mut fg = Figure::new();
    let axes = fg.axes2d();

    for (name, ip) in &servers {
        let mut x: Vec<f64> = Vec::new();
        let mut y: Vec<f64> = Vec::new();

        for i in 0..count {
            let (_packet, duration) = surge_ping::ping(ip.parse()?, &payload).await?;
            x.push((i + 1) as f64);
            y.push(duration.as_secs_f64() * 1000.0) // ms
        }
        axes.lines(&x, &y, &[gnuplot::Caption(name)]);
    }

    axes.set_title("Ping times", &[])
        .set_x_label("Ping #", &[])
        .set_y_label("Latency (ms)", &[]);

    fg.show()?;       
    Ok(())
}
