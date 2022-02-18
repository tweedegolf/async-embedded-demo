#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use firmware::config;

use cortex_m_rt::entry;
use defmt::{info, unwrap};
use defmt_rtt as _; // global logger
use embassy::executor::{Executor, Spawner};
use embassy::io::AsyncWriteExt;
use embassy::time::{Duration, Timer};
use embassy::util::Forever;
use embassy_net::{Ipv4Address, StackResources, TcpSocket};

#[cfg(feature = "dhcp")]
use embassy_net::DhcpConfigurator as Configurator;
#[cfg(not(feature = "dhcp"))]
use embassy_net::{Config as NetConfig, Ipv4Cidr, StaticConfigurator as Configurator};

use embassy_stm32::eth::lan8742a::LAN8742A;
use embassy_stm32::eth::{Ethernet, State};
use embassy_stm32::rng::Rng;
use embassy_stm32::{interrupt, peripherals};
use panic_probe as _;

use peripherals::RNG;

#[embassy::task]
async fn main_task(
    device: &'static mut Ethernet<'static, LAN8742A, 4, 4>,
    config: &'static mut Configurator,
    spawner: Spawner,
) {
    let net_resources = NET_RESOURCES.put(StackResources::new());

    // Init network stack
    embassy_net::init(device, config, net_resources);

    // Launch network task
    unwrap!(spawner.spawn(net_task()));

    info!("Network task initialized");

    // Then we can use it!
    let mut rx_buffer = [0; 1024];
    let mut tx_buffer = [0; 1024];
    let mut socket = TcpSocket::new(&mut rx_buffer, &mut tx_buffer);

    socket.set_timeout(Some(embassy_net::SmolDuration::from_secs(10)));

    let ip_addr: Ipv4Address = unwrap!(include_str!("../../target/HOST_IP_ADDR").parse());
    let port: u16 = include_str!("../../target/HOST_PORT").parse().unwrap();

    info!("Connecting to {}:{}...", &ip_addr, &port);
    let remote_endpoint = (ip_addr, port);
    let r = socket.connect(remote_endpoint).await;
    if let Err(e) = r {
        info!("connect error: {:?}", e);
        return;
    }
    info!("Connected!");
    loop {
        info!("Sending message");
        let r = socket.write_all(b"Hello\n").await;
        if let Err(e) = r {
            info!("write error: {:?}", e);
            return;
        }
        Timer::after(Duration::from_secs(1)).await;
    }
}

#[embassy::task]
async fn net_task() {
    embassy_net::run().await
}

#[no_mangle]
fn _embassy_rand(buf: &mut [u8]) {
    use rand_core::RngCore;

    critical_section::with(|_| unsafe {
        unwrap!(RNG_INST.as_mut()).fill_bytes(buf);
    });
}

static mut RNG_INST: Option<Rng<RNG>> = None;

static EXECUTOR: Forever<Executor> = Forever::new();
static STATE: Forever<State<'static, 4, 4>> = Forever::new();
static ETH: Forever<Ethernet<'static, LAN8742A, 4, 4>> = Forever::new();
static CONFIG: Forever<Configurator> = Forever::new();
static NET_RESOURCES: Forever<StackResources<1, 2, 8>> = Forever::new();

#[entry]
fn main() -> ! {
    info!("Hello World!");

    info!("Setup RCC...");

    let p = embassy_stm32::init(config());

    let rng = Rng::new(p.RNG);
    unsafe {
        RNG_INST.replace(rng);
    }

    let eth_int = interrupt::take!(ETH);
    let mac_addr = [0x00, 0x00, 0xDE, 0xAD, 0xBE, 0xEF];
    let state = STATE.put(State::new());

    let eth = unsafe {
        ETH.put(Ethernet::new(
            state, p.ETH, eth_int, p.PA1, p.PA2, p.PC1, p.PA7, p.PC4, p.PC5, p.PG13, p.PB13,
            p.PG11, LAN8742A, mac_addr, 0,
        ))
    };

    #[cfg(not(feature = "dhcp"))]
    let config = Configurator::new(NetConfig {
        // You may want to configure another IP address if this one is already taken
        address: Ipv4Cidr::new(Ipv4Address::new(192, 168, 0, 2), 24),
        dns_servers: heapless::Vec::new(),
        gateway: None,
    });

    #[cfg(feature = "dhcp")]
    let config = Configurator::new();

    let config = CONFIG.put(config);

    let executor = EXECUTOR.put(Executor::new());

    executor.run(move |spawner| {
        unwrap!(spawner.spawn(main_task(eth, config, spawner)));
    })
}
