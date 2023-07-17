run:
	docker run -p 8000:8000 neu-backend
	
build:
	docker build --tag neu-backend --file Dockerfile .
sqlx_prepare:
	cargo sqlx prepare -- --lib