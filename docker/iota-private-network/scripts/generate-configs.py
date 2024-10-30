import json
import yaml
from pathlib import Path

def load_json_data(file_path):
    with open(file_path, 'r') as f:
        return json.load(f)

def load_yaml_data(file_path):
    with open(file_path, 'r') as f:
        return yaml.safe_load(f)

def generate_fullnode_config(data, peer_list):
    template = """---
network-key-pair:
  value: {network_key}
db-path: /opt/iota/db
network-address: /ip4/0.0.0.0/tcp/8080/http
json-rpc-address: "0.0.0.0:9000"
metrics-address: "0.0.0.0:9184"
admin-interface-port: 1337
enable-event-processing: true
grpc-load-shed: ~
grpc-concurrency-limit: ~
jsonrpc-server-type: both
p2p-config:
  listen-address: "0.0.0.0:8084"
  external-address: {p2p_address}
  seed-peers:
{seed_peers}
genesis:
  genesis-file-location: /opt/iota/config/genesis.blob
migration-tx-data-path: /opt/iota/config/migration.blob
authority-store-pruning-config:
  num-latest-epoch-dbs-to-retain: 3
  epoch-db-pruning-period-secs: 3600
  num-epochs-to-retain: 18446744073709551615
  max-checkpoints-in-batch: 5
  max-transactions-in-batch: 1000
  use-range-deletion: true
  pruning-run-delay-seconds: 60
state-debug-dump-config:
  dump-file-directory: /opt/iota/state_debug_dump
enable-experimental-rest-api: true"""

    seed_peers_list = []
    for peer in peer_list:
        peer_entry = f"""    - address: {peer.get('address', '')}
      peer-id: {peer.get('peer-id', '')}"""
        seed_peers_list.append(peer_entry)
    
    seed_peers = '\n'.join(seed_peers_list)

    return template.format(
        network_key=data["network_keystore"]["privateBase64Key"],
        p2p_address=data["p2p_address"],
        seed_peers=seed_peers
    )


def generate_validator_config(data, peer_list):
    template = """---
protocol-key-pair:
  value: {protocol_key}
worker-key-pair:
  value: {worker_key}
account-key-pair:
  value: {account_key}
network-key-pair:
  value: {network_key}
db-path: /opt/iota/db/authorities_db
network-address: /ip4/0.0.0.0/tcp/8080/http
json-rpc-address: "0.0.0.0:9000"
enable-experimental-rest-api: true
metrics-address: "0.0.0.0:9184"
admin-interface-port: 1337
consensus-config:
  address: /dns/127.0.0.1/tcp/8083/http
  db-path: /opt/iota/db/consensus_db
  db-retention-epochs: ~
  db-pruner-period-secs: ~
  max-pending-transactions: ~
  max-submit-position: ~
  submit-delay-step-override-millis: ~
  parameters: ~
enable-event-processing: false
enable-index-processing: true
websocket-only: false
grpc-load-shed: ~
grpc-concurrency-limit: 20000000000
p2p-config:
  listen-address: "0.0.0.0:8084"
  external-address: {p2p_address}
  state-sync:
    checkpoint-content-timeout-ms: 10000
  seed-peers:
{seed_peers}
genesis:
  genesis-file-location: /opt/iota/config/genesis.blob
migration-tx-data-path: /opt/iota/config/migration.blob
authority-store-pruning-config:
  num-latest-epoch-dbs-to-retain: 3
  epoch-db-pruning-period-secs: 3600
  num-epochs-to-retain: 0
  max-checkpoints-in-batch: 10
  max-transactions-in-batch: 1000
end-of-epoch-broadcast-channel-capacity: 128
checkpoint-executor-config:
  checkpoint-execution-max-concurrency: 200
  local-execution-timeout-sec: 30
db-checkpoint-config:
  perform-db-checkpoints-at-epoch-end: false
indirect-objects-threshold: 18446744073709551615
expensive-safety-check-config:
  enable-epoch-iota-conservation-check: true
  enable-deep-per-tx-iota-conservation-check: false
  force-disable-epoch-iota-conservation-check: false
  enable-state-consistency-check: false
  force-disable-state-consistency-check: false
  enable-move-vm-paranoid-checks: false
transaction-deny-config:
  package-publish-disabled: false
  package-upgrade-disabled: false
  shared-object-disabled: false
  user-transaction-disabled: false
  receiving-objects-disabled: false
  zklogin-sig-disabled: false
  zklogin-disabled-providers: []
certificate-deny-config: {{}}
state-debug-dump-config: {{}}
state-archive-write-config:
  concurrency: 0
  use-for-pruning-watermark: false
state-archive-read-config: []
state-snapshot-write-config:
  concurrency: 0
indexer-max-subscriptions: ~
transaction-kv-store-read-config:
  base-url: ""
jwk-fetch-interval-seconds: 3600
zklogin-oauth-providers:
  Mainnet:
    - Apple
    - Facebook
    - Google
    - Twitch
  Testnet:
    - Apple
    - Facebook
    - Google
    - Twitch
  Unknown:
    - Apple
    - Facebook
    - Google
    - Kakao
    - Slack
    - Twitch
authority-overload-config:
  max-txn-age-in-queue:
    secs: 1
    nanos: 0
  overload-monitor-interval:
    secs: 10
    nanos: 0
  execution-queue-latency-soft-limit:
    secs: 1
    nanos: 0
  execution-queue-latency-hard-limit:
    secs: 10
    nanos: 0
  max-load-shedding-percentage: 95
  min-load-shedding-percentage-above-hard-limit: 50
  safe-transaction-ready-rate: 100
  check-system-overload-at-signing: true
execution-cache: passthrough-cache
enable-soft-bundle: true
enable-validator-tx-finalizer: true
"""
    seed_peers_list = []
    for peer in peer_list:
        peer_entry = f"""    - address: {peer.get('address', '')}
      peer-id: {peer.get('peer-id', '')}"""
        seed_peers_list.append(peer_entry)
    
    seed_peers = '\n'.join(seed_peers_list)

    # Add required fields from the JSON data
    return template.format(
        protocol_key=data.get('protocol_keystore', {}).get('privateBase64Key', ''),
        worker_key=data.get('worker_keystore', {}).get('privateBase64Key', ''),
        account_key=data.get('account_keystore', {}).get('privateBase64Key', ''),
        network_key=data.get('network_keystore', {}).get('privateBase64Key', ''),
        p2p_address=data.get('p2p_address', ''),
        seed_peers=seed_peers
    )

def generate_faucet_client(data):
    template = """---
keystore:
  File: /root/.iota/iota_config/iota.keystore
envs:
  - alias: iota-private-network
    rpc: "http://iota:9000"
    ws: ~
    basic_auth: ~
active_env: iota-private-network
active_address: "{iotaAddress}"
"""
    return template.format(
        iotaAddress=data['faucet_keystore']['iotaAddress'],
    )
    
def generate_faucet_keystore(data):
    template = """
[
    "{privateKey}"
]
"""
    return template.format(
        privateKey=data['faucet_keystore']['privateBech32Key'],
    )

def main():
    validators_config_dir = Path('./configs/validators')
    validators_config_dir.mkdir(exist_ok=True)
    
    fullnodes_config_dir = Path('./configs/fullnodes')
    fullnodes_config_dir.mkdir(exist_ok=True)
    
    faucet_dir = Path('./configs/faucet')
    faucet_dir.mkdir(exist_ok=True)
    
    peer_list_path = Path('./configs/keystores/peer_list.yaml')
    peer_list = load_yaml_data(peer_list_path)
    
    keystores_dir = Path('./configs/keystores')
    for json_file in keystores_dir.glob('*.json'):
        data = load_json_data(json_file)
        
        if json_file.stem.startswith('validator'):
            config_content = generate_validator_config(data, peer_list)
            output_path = validators_config_dir / f"{json_file.stem}.yaml"
            with open(output_path, 'w') as f:
                f.write(config_content)
        
        if json_file.stem.startswith('fullnode'):
            config_content = generate_fullnode_config(data, peer_list)
            output_path = fullnodes_config_dir / f"{json_file.stem}.yaml"
            with open(output_path, 'w') as f:
                f.write(config_content)
                
        if json_file.stem == 'fullnode-0':
            data = load_json_data(json_file)
            config_content = generate_faucet_client(data)
            output_path = faucet_dir / "client.yaml"
            with open(output_path, 'w') as f:
                f.write(config_content)
            config_content = generate_faucet_keystore(data)
            output_path = faucet_dir / "iota.keystore"
            with open(output_path, 'w') as f:
                f.write(config_content)
            
if __name__ == "__main__":
    main()