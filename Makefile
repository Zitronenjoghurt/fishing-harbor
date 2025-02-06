.PHONY: up down dev-up dev-down test-up test-down test-backtrace test

up:
	cd docker && docker compose up -d --build

down:
	cd docker && docker compose down

dev-up:
	docker compose -f docker/docker-compose.dev.yml up -d --build

dev-down:
	docker compose -f docker/docker-compose.dev.yml down

test-up:
	docker compose -f docker/docker-compose.test.yml up -d --build

test:
	docker compose -f docker/docker-compose.test.yml exec -T app cargo test -- --nocapture --test-threads=1

test-backtrace:
	docker compose -f docker/docker-compose.test.yml exec -T -e RUST_BACKTRACE=1 app cargo test -- --nocapture --test-threads=1

test-down:
	docker compose -f docker/docker-compose.test.yml down

clean: down dev-down test-down
	docker image prune -f