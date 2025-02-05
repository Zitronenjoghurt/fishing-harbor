.PHONY: up down dev-up dev-down

up:
	cd docker && docker compose up -d --build

down:
	cd docker && docker compose down

dev-up:
	docker compose -f docker/docker-compose.dev.yml up -d --build

dev-down:
	docker compose -f docker/docker-compose.dev.yml down