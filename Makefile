database:
	docker run \
		--volume $(PWD)/dump:/temp \
		--env POSTGRES_PASSWORD=postgres \
		--publish 5432:5432 \
		postgres

db-shell:
	PGPASSWORD=postgres psql -h localhost -U postgres

run:
	cargo run
