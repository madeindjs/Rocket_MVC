CREATE TABLE `recipes` (
	`id`	INTEGER NOT NULL,
	`name`	TEXT NOT NULL,
	PRIMARY KEY(id)
);

INSERT INTO `recipes`(`name`) VALUES ('Hello'), ('world');
