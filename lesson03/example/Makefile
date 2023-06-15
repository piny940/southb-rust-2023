.PHONY: up-db
up-db:
	docker compose up -d db

.PHONY: stop-db
stop-db:
	docker compose stop db

.PHONY: down
down:
	docker compose down

.PHONY: psql
psql:
	docker compose exec db psql -U postgres

.PHONY: install-sqlx
install-sqlx:
	cargo install sqlx-cli --no-default-features --features rustls,postgres

.PHONY: migrate
migrate:
	sqlx migrate run