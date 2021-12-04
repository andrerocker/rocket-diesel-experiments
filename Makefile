database:
	docker run \
		--volume $(PWD)/dump:/temp \
		--env POSTGRES_PASSWORD=postgres \
		--publish 5432:5432 \
		postgres

db-shell:
	PGPASSWORD=postgres psql -h localhost -U postgres

load-samples:
	cat others/posts_samples.sql | PGPASSWORD=postgres psql -h localhost -U postgres

request:
	curl localhost:8000/v1/posts -vvv | jq

run:
	cargo run
