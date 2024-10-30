if ! python3 -m venv venv; then
    echo "Failed to create virtual environment"
    exit 1
fi

if ! source venv/bin/activate; then
    echo "Failed to activate virtual environment" 
    exit 1
fi

if ! pip install -r scripts/requirements.txt; then
    echo "Failed to install requirements"
    exit 1
fi

generate_keystores() {
  echo "Generating keystores for nodes..."
  rm -rf ./config/keystores
  python scripts/generate-keystores.py
}

generate_configs() {
  echo "Generating config files for nodes..."
  rm -rf ./configs/fullnodes
  rm -rf ./configs/validators
  rm -rf ./configs/faucet
  python scripts/generate-configs.py
}

generate_genesis() {
  echo "Generating genesis files for nodes..."
  rm -rf ./configs/genesis
  python scripts/generate-genesis.py
}

cleanup_genesis_folder() {
  mkdir -p ./configs/genesis
  mv ./configs/temp-genesis/genesis.blob ./configs/genesis/genesis.blob
  mv ./configs/temp-genesis/migration.blob ./configs/genesis/migration.blob
  rm -rf ./configs/temp-genesis
}

create_folder_for_postgres() {
  echo "Creating Postgres folder ..."
  mkdir -p ./data/postgres
  chown -R 999:999 ./data/postgres
  chmod 0755 ./data/postgres
}

if [ ! -d "./configs" ]; then
    echo "Configs folder not found. Generating all required files..."
    generate_keystores
    generate_configs 
    generate_genesis
    cleanup_genesis_folder
    create_folder_for_postgres
else
    read -p "Do you want to fully reset? (y/n) " reset_answer
    if [ "$reset_answer" = "y" ]; then
        echo "Fully resetting..."
        ./cleanup.sh
        generate_keystores
        generate_configs
        generate_genesis
        cleanup_genesis_folder
        create_folder_for_postgres
    else
        read -p "Do you want to regenerate the genesis files? (y/n) " genesis_answer
        if [ "$genesis_answer" = "y" ]; then
            echo "Regenerating genesis files..."
            generate_genesis
            cleanup_genesis_folder
            create_folder_for_postgres
        else
            echo "Only creating postgres folder..."
            create_folder_for_postgres
        fi
    fi
fi