DROP TABLE text;
DROP TABLE definitions;
DROP TABLE reductions;
DROP TABLE integers;

CREATE TABLE text (
	id integer PRIMARY KEY NOT NULL,
	result text NOT NULL
);
CREATE TABLE definitions (
	id integer PRIMARY KEY NOT NULL,
	applicant integer NOT NULL,
	argument integer NOT NULL
);
CREATE TABLE reductions (
	id integer PRIMARY KEY NOT NULL,
	normal_form integer NOT NULL
);
CREATE TABLE integers (
	id integer PRIMARY KEY NOT NULL,
	result integer NOT NULL
);
