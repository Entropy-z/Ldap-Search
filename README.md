
# LDAP Search Tool
 is a powerful utility built in Rust, designed for efficient LDAP server searches. This tool empowers you to seamlessly connect to an LDAP server, perform authentication, and execute searches with personalized criteria.

## Features
**Easy Setup**: simplifies LDAP searches with a straightforward setup process.
**Flexible Criteria**: Customize your search using a variety of criteria, including LDAP filter and attributes to retrieve.
**Secure Authentication**: Safely authenticate using your username and password, ensuring a secure connection to the LDAP server.
**Domain Integration**: Optionally, you can specify a domain component to construct the Distinguished Name (DN) for more precise searches.
**Attribute Selection**: Choose specific attributes to retrieve in the search results, enhancing efficiency and relevance.

## Usage
Execute Ldap-Search by providing the required parameters in the command line:
```sh
cargo run -- -s <ldap_server:ldap_port> -u <username:password> [-d <domain_component>] [-f <filter>] [-a <attributes>]
```
- -s: Specifies the LDAP server address and port in the format <ldap_server:ldap_port>.
- -u: Provides the username and password in the format username:password.
- -d (optional): Allows you to include the domain component for constructing the DN.
- -f (optional): Enables you to set a custom LDAP filter for precise searches.
- -a (optional): Lets you list the attributes to be returned in the search results.

Example:
```sh
cargo run -s ldap.example.com:389 -u admin:password -d example -f "(objectclass=user)" -a cn,mail
```
## Installation

1 - Source installation:

Clone this repository: 
```sh
git clone https://github.com/Entropy-z/Ldap-Search
```
Navigate to the project directory: 
```sh
cd Ldap-Search
```
Build:
```sh
cargo build --release
```
2 - Binary Release.

