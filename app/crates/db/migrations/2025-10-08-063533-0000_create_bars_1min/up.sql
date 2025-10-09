-- Your SQL goes here
CREATE TABLE bars_1min (
   id SERIAL PRIMARY KEY,
   exchange VARCHAR NOT NULL,
   market VARCHAR NOT NULL,
   timestamp TIMESTAMP NOT NULL,
   open DECIMAL NOT NULL,
   close DECIMAL NOT NULL,
   min DECIMAL NOT NULL,
   max DECIMAL NOT NULL
)