use ldap3::*;
use std::{env, process::exit};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 9 {
        eprintln!("Usage: {} -s <ldap_server:ldap_port> -u <username:password> [-d <domain_component>] [-f <filter>] [-a <attributes>]", args[0]);
        exit(1);
    }

    let mut ldap_server = "";
    let mut ldap_port = "";
    let mut username = "";
    let mut password = "";
    let mut domain_component = "";
    let mut filter = format!("(&(objectclass=user)(samaccountname={}*)", username);
    let mut attributes: Vec<&str> = vec!["dn"];

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "-s" => {
                let server_info: Vec<&str> = args[i + 1].split(':').collect();
                if server_info.len() != 2 {
                    eprintln!("Invalid server information: {}", args[i + 1]);
                    exit(1);
                }
                ldap_server = server_info[0];
                ldap_port = server_info[1];
            }
            "-u" => {
                let user_info: Vec<&str> = args[i + 1].split(':').collect();
                if user_info.len() != 2 {
                    eprintln!("Invalid username and password: {}", args[i + 1]);
                    exit(1);
                }
                username = user_info[0];
                password = user_info[1];
            }
            "-d" => domain_component = &args[i + 1],
            "-f" => filter = args[i + 1].clone(),
            "-a" => attributes = args[i + 1].split(',').collect(),
            _ => {
                eprintln!("Invalid option: {}", args[i]);
                exit(1);
            }
        }
        i += 2;
    }

    let ldap_url = format!("ldap://{}:{}", ldap_server, ldap_port);
    let ldap = LdapConn::new(&ldap_url);

    let mut ldapcon = match ldap {
        Ok(l) => l,
        Err(r) => panic!("{}", r),
    };

    let bind_result = ldapcon.simple_bind(username, password);
    if let Err(err) = bind_result {
        eprintln!("Bind failed: {:?}", err);
        exit(1);
    }

    let full_base_dn = if domain_component.is_empty() {
        "".to_string()
    } else {
        format!("DC={},", domain_component)
    };

    println!("Full Base DN: {}", full_base_dn);
    println!("Filter: {}", filter);
    println!("Attributes: {:?}", attributes);

    let res = ldapcon.search(&full_base_dn, Scope::Subtree, &filter, attributes).unwrap();

    let (re, _) = res.success().unwrap();

    for i in re {
        println!("{:#?}", SearchEntry::construct(i).dn);
    }
}
