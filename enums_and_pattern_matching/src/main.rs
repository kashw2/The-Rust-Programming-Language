#[derive(Debug)]
enum IpAddressKind {
    IpV4(String),
    IpV6(String),
}

fn main() {
    let ipv4: IpAddressKind = IpAddressKind::IpV4(String::from("127.0.0.1"));
    let ipv6: IpAddressKind = IpAddressKind::IpV6(String::from("::1"));

    println!("{:?}", ipv4);

    dbg!(&split_ip(&ipv4));
    dbg!(&split_ip(&ipv6));
}

fn split_ip(address: &IpAddressKind) -> Vec<&str> {
    match address {
        IpAddressKind::IpV4(address) => address.split(".").collect::<Vec<_>>(),
        IpAddressKind::IpV6(address) => address.split("::").collect::<Vec<_>>()
    }
}
