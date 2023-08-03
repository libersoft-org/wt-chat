# Requirements

1. Get a Linux server with public IP address
2. Buy a domain name and create an A record, for example: **chat.domain.tld** directing your Linux server IP address to this domain name

# Installation

1. Install Linux dependencies:

```sh
apt update
apt -y upgrade
apt -y install build-essential cmake libclang-dev curl git
```

2. Install Rust

```sh
curl https://sh.rustup.rs -sSf | sh -s -- --no-prompt
source "$HOME/.cargo/env"
```

3. Download the latest version of WebTransport chat

```sh
git clone https://github.com/libersoft-org/wt-chat.git
cd wt-chat
```

4. Create a certificate:

```sh
certbot certonly --standalone --register-unsafely-without-email --agree-tos -d chat.domain.tld
```

(replace "**chat.domain.tld**" with your actual domain name)

Alternatively you can create your self-signed certificate:

```sh
openssl req -newkey rsa:2048 -new -nodes -x509 -days $(expr '(' $(date -d 2999/01/01 +%s) - $(date +%s) + 86399 ')' / 86400) -subj "/" -keyout key.pem -out cert.pem
```

... but the browser will not accept it by default. You can run the browser with parameter that will ignore certificate errors, but first create your fingerprint:

```sh
openssl x509 -pubkey -noout -in cert.pem | openssl pkey -pubin -outform der | openssl dgst -sha256 -binary | base64
```

... and then run your browser:

```sh
chrome --ignore-certificate-errors-spki-list=FINGERPRINT --ignore-certificate-errors --v=2 --enable-logging=stderr --origin-to-force-quic-on=SERVER:PORT
```

(replace **FINGERPRINT**, **SERVER** and **PORT** with your actual values)

5. Create a config file in **src/settings.json**

```json
{
 "http_port": 80,
 "https_port": 443,
 "https_cert_path": "/etc/letsencrypt/live/chat.domain.tld/",
 "web_root": "src/web",
 "log_to_file": true,
 "log_file": "chat.log",
}
```

(replace "**chat.domain.tld**" with your actual domain name)

6. Install dependencies and run the WebTransport Chat Server:

```sh
cargo run
```
