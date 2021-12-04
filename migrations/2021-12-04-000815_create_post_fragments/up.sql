CREATE TABLE fragments (
       id SERIAL PRIMARY KEY,
       image_url TEXT NOT NULL,
       comment TEXT NOT NULL,
       post_id INT NOT NULL,
       CONSTRAINT fk_customer
                  FOREIGN KEY(post_id) 
	          REFERENCES posts(id)
);
