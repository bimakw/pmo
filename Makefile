.PHONY: dev dev-db stop clean build run

# Development - database only
dev-db:
	docker compose -f docker-compose.dev.yml up -d

# Development - run backend locally
dev-backend:
	cd backend && cargo run

# Development - run frontend locally
dev-frontend:
	cd frontend && npm run dev

# Production - build all
build:
	docker compose build

# Production - run all services
run:
	docker compose up -d

# Stop all services
stop:
	docker compose down
	docker compose -f docker-compose.dev.yml down

# Clean up volumes
clean:
	docker compose down -v
	docker compose -f docker-compose.dev.yml down -v

# View logs
logs:
	docker compose logs -f

# Database shell
db-shell:
	docker exec -it percival-db psql -U postgres -d percival_db
