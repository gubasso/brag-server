is_db_running:
	@. ./.env; \
	docker-compose -f Docker/compose.yaml ps -q db

migrate:
	sqlx migrate revert
	sqlx migrate run

spin_db:
	@. ./.env; \
	@echo "Starting postgresql the database..."; \
	docker compose -f Docker/compose.yaml up --wait

load_db:
	@# Check if the spin_db process is already running
	@if [ -z "$$(make -s is_db_running)" ]; then \
		make -s spin_db; \
	else \
		echo "Database already running."; \
	fi
	@echo "Running load_db: connect and load data to db"
	@. ./.env; \
	cargo run --bin load_db

connect_db:
	@. ./.env; \
	psql "$${DATABASE_URL}"

watch: load_db
	cargo watch -q -c -w src -x "run --bin brag-server"

db_clean:
	@. ./.env; \
	sqlx migrate revert; \
	sudo rm -rf "$$DATA_PATH" ../"$$DATA_PATH"; \
	rm -rf "$HOME"/.local/share/brag-server

dstop:
	@. ./.env; \
	docker compose -f Docker/compose.yaml down; \
	docker kill "$$(docker ps -q)"

dclean: db_clean dstop

dprune: db_clean dstop
	docker system prune -a -f && docker volume prune -f
	docker network prune -f

setup_dev: asdf_install_rust asdf_install_python
	asdf install
	asdf reshim
	pip install pre-commit
	pre-commit install
	pre-commit autoupdate
	pre-commit run --all-files

install_asdf:
	sudo apt-get install -y curl git
	git clone https://github.com/asdf-vm/asdf.git ~/.asdf --branch v0.13.1
	echo '. "$HOME/.asdf/asdf.sh"' >> ~/.bashrc
	echo '. "$HOME/.asdf/completions/asdf.bash"' >> ~/.bashrc
	. ~/.bashrc

asdf_install_rust: install_asdf
	asdf plugin add rust
	@. ./.env; \
	asdf install rust "$$RUST_VERSION"; \
	asdf global rust "$$RUST_VERSION"; \
	asdf local rust "$$RUST_VERSION"

asdf_install_python: install_asdf
	asdf plugin add python
	@. ./.env; \
	asdf install python "$$PYTHON_VERSION"; \
	asdf global python "$$PYTHON_VERSION"; \
	asdf local python "$$PYTHON_VERSION"

install: install_asdf asdf_install_rust
	asdf reshim
	make -s load_db
	make -s load_db
