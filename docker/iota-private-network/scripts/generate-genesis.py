import docker
import json
import os
from pathlib import Path

iota_tools_image = "iota-tools"

def load_json_data(file_path):
    with open(file_path, 'r') as f:
        return json.load(f)

def run_iotatool_docker(command):
    client = docker.from_env()

    container_output = client.containers.run(
        iota_tools_image,
        command=command,
        remove=True,
        detach=False,
        volumes={
            os.path.abspath('./configs/temp-genesis'): {
                'bind': '/iota/genesis',
                'mode': 'rw'
            }
        },
        working_dir='/iota/genesis'
    )
    client.close()
    return container_output.decode('utf-8').strip()

def genesis_ceremony_init():    
    command = "/bin/sh -c '/usr/local/bin/iota genesis-ceremony init'"
    run_iotatool_docker(command)

def genesis_ceremony_add_validators(validators_data):
    os.makedirs("./configs/temp-genesis/keys", exist_ok=True)
    for validator_name, validator_data in validators_data:
            validator_keys_dir = Path(f'./configs/temp-genesis/keys/{validator_name}')
            validator_keys_dir.mkdir(exist_ok=True)

            with open(validator_keys_dir / 'protocol.key', 'w') as f:
                f.write(validator_data['protocol_keystore']['privateBase64Key'])
                

            with open(validator_keys_dir / 'worker.key', 'w') as f:
                f.write(validator_data['worker_keystore']['privateBech32Key'])
                

            with open(validator_keys_dir / 'account.key', 'w') as f:
                f.write(validator_data['account_keystore']['privateBech32Key'])
                

            with open(validator_keys_dir / 'network.key', 'w') as f:
                f.write(validator_data['network_keystore']['privateBech32Key'])
    
    for validator_name, validator_data in validators_data:
        command = f"/bin/sh -c '/usr/local/bin/iota genesis-ceremony add-validator \
            --name \"{validator_name}\" \
            --validator-key-file /iota/genesis/keys/{validator_name}/protocol.key \
            --worker-key-file /iota/genesis/keys/{validator_name}/worker.key \
            --account-key-file /iota/genesis/keys/{validator_name}/account.key \
            --network-key-file /iota/genesis/keys/{validator_name}/network.key \
            --network-address \"{validator_data['network_address']}\" \
            --p2p-address \"{validator_data['p2p_address']}\" \
            --narwhal-primary-address \"{validator_data['narwhal_primary_address']}\" \
            --narwhal-worker-address \"{validator_data['narwhal_worker_address']}\" \
            --description \"{validator_name}\" \
            --image-url \"\" \
            --project-url \"\"'"
        run_iotatool_docker(command)

def genesis_ceremony_add_token_allocation(faucet_address):
    command = f"/bin/sh -c '/usr/local/bin/iota genesis-ceremony add-token-allocation \
        --recipient-address {faucet_address} \
        --amount-nanos 750000000000000000'"
    run_iotatool_docker(command)

def genesis_ceremony_build_unsigned_checkpoint():
    command = f"/bin/sh -c '/usr/local/bin/iota genesis-ceremony build-unsigned-checkpoint \
        --remote-migration-snapshots https://stardust-objects.s3.eu-central-1.amazonaws.com/iota/alphanet/test/stardust_object_snapshot.bin.gz'"
    run_iotatool_docker(command)

def genesis_ceremony_verify_and_sign(validators_data):
    for validator_name, _ in validators_data:
        command = f"/bin/sh -c '/usr/local/bin/iota genesis-ceremony verify-and-sign \
            --key-file /iota/genesis/keys/{validator_name}/protocol.key'"
        run_iotatool_docker(command)

def genesis_ceremony_finalize():
    command = "/bin/sh -c '/usr/local/bin/iota genesis-ceremony finalize'"
    run_iotatool_docker(command)
    
def main():
    genesis_folder = Path('./configs/temp-genesis')
    genesis_folder.mkdir(exist_ok=True)
    
    validators_data = []
    faucet_address = ''
    keystores_folder = Path('./configs/keystores')
    for json_file in keystores_folder.glob('*.json'):
        if json_file.stem.startswith('validator'):
            data = load_json_data(json_file)
            validators_data.append((json_file.stem, data))
        if json_file.stem == 'fullnode-0':
            data = load_json_data(json_file)
            faucet_address = data['faucet_keystore']['iotaAddress']
            
    genesis_ceremony_init()
    genesis_ceremony_add_validators(validators_data)
    genesis_ceremony_add_token_allocation(faucet_address)
    genesis_ceremony_build_unsigned_checkpoint()
    genesis_ceremony_verify_and_sign(validators_data)
    genesis_ceremony_finalize()

if __name__ == "__main__":
    main()