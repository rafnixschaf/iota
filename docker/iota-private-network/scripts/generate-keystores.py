import os
import json
import docker
import shutil

iota_tools_image = "iota-tools"

def run_keytool_docker(command):
    client = docker.from_env()

    container_output = client.containers.run(
        iota_tools_image,
        command=command,
        remove=True,
        detach=False
    )
    client.close()
    return container_output.decode('utf-8').strip()

def generate_ed25519_keystore():
    command = "/bin/sh -c '/usr/local/bin/iota keytool generate ed25519 --json && cat *.key'"
    output = run_keytool_docker(command)
    if output:
        results = output.split('\n')
        json_data = json.loads('\n'.join(results[:-1]))
        privateBech32Key = results[-1].strip()
        privateBase64Key = convert_base32_to_base64(results[-1].strip())
        return json_data["peerId"], {
            "iotaAddress": json_data["iotaAddress"],
            "publicBase64Key": json_data["publicBase64Key"],
            "mnemonic": json_data["mnemonic"],
            "privateBech32Key": privateBech32Key,
            "privateBase64Key": privateBase64Key
        }
    return None, None

def convert_base32_to_base64(privateKey):
    command = f"/bin/sh -c '/usr/local/bin/iota keytool convert {privateKey} --json'"
    output = run_keytool_docker(command)
    if output:
        json_data = json.loads(output)
        return json_data.get("base64WithFlag")
    return None

def generate_bls12381_keystore():
    command = "/bin/sh -c '/usr/local/bin/iota keytool generate bls12381 --json && cat *.key'"
    output = run_keytool_docker(command)
    if output:
        results = output.split('\n')
        json_data = json.loads('\n'.join(results[:-1]))
        private_key = results[-1].strip()
        return {
            "iotaAddress": json_data["iotaAddress"],
            "publicBase64Key": json_data["publicBase64Key"],
            "privateBase64Key": private_key
        }
    return None

def generate_validator_data(validator_name):
    protocol_keystore = generate_bls12381_keystore()
    _, worker_keystore = generate_ed25519_keystore()
    _, account_keystore = generate_ed25519_keystore()
    peerId, network_keystore = generate_ed25519_keystore()

    network_address = "/dns/{}/tcp/8080/http".format(validator_name)
    p2p_address= "/dns/{}/udp/8084".format(validator_name)
    narwhal_primary_address= "/dns/{}/udp/8081".format(validator_name)
    narwhal_worker_address= "/dns/{}/udp/8082".format(validator_name)

    validator_data = {
        "peerId": peerId,
        "protocol_keystore": protocol_keystore,
        "worker_keystore": worker_keystore,
        "account_keystore": account_keystore,
        "network_keystore": network_keystore,
        "network_address": network_address,
        "p2p_address":p2p_address,
        "narwhal_primary_address":narwhal_primary_address,
        "narwhal_worker_address":narwhal_worker_address
    }
    
    return validator_data

def generate_fullnode_data(fullnode_name):
    peerId, network_keystore = generate_ed25519_keystore()
    p2p_address = "/dns/{}/udp/8084".format(fullnode_name)
    
    fullnode_data = {
        "peerId": peerId,
        "network_keystore": network_keystore,
        "p2p_address": p2p_address
    }
    
    if fullnode_name == "fullnode-0":
        _, faucet_keystore = generate_ed25519_keystore()
        fullnode_data["faucet_keystore"] = faucet_keystore
        
    return fullnode_data
        
def generate_validators_data():
    validators_data = {}
    for i in range(4):
        validator_name = f"validator-{i}"
        validators_data[validator_name] = generate_validator_data(validator_name)
    return validators_data

def generate_fullnodes_data():
    validators_data = {}
    for i in range(2):
        fullnode_name = f"fullnode-{i}"
        validators_data[fullnode_name] = generate_fullnode_data(fullnode_name)
    return validators_data
    
def generate_peer_list(validators, fullnodes):
    peer_list = []

    for _, validator_data in validators.items():
        peer_list.append(f"- address: {validator_data['p2p_address']}\n  peer-id: {validator_data['peerId']}")
    for _, fullnode_data in fullnodes.items():
        peer_list.append(f"- address: {fullnode_data['p2p_address']}\n  peer-id: {fullnode_data['peerId']}")
            
    return "\n".join(peer_list)

def store_keystores(validators, fullnodes, peer_list):
    os.makedirs("./configs/keystores", exist_ok=True)
    for validator_name, validator_data in validators.items():
        with open(f"./configs/keystores/{validator_name}.json", "w") as f:
            json.dump(validator_data, f, indent=2)

    for fullnode_name, fullnode_data in fullnodes.items():
        with open(f"./configs/keystores/{fullnode_name}.json", "w") as f:
            json.dump(fullnode_data, f, indent=2)
    
    with open("./configs/keystores/peer_list.yaml", "w") as f:
        f.write(peer_list)
    

validators = generate_validators_data()
fullnodes = generate_fullnodes_data()
peer_list = generate_peer_list(validators, fullnodes)
store_keystores(validators, fullnodes, peer_list)