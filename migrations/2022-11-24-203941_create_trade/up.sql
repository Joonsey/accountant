CREATE TABLE trade (
	id SERIAL PRIMARY KEY,
	owner VARCHAR NOT NULL,
	itemString VARCHAR NOT NULL,
	itemName VARCHAR NOT NULL,
	stackSize INT NOT NULL,
	quantity INT NOT NULL,
	price INT NOT NULL,
	otherPlayer VARCHAR NOT NULL,
	player VARCHAR NOT NULL,
	time INT NOT NULL,
	source VARCHAR NOT NULL
)
-- Your SQL goes here
