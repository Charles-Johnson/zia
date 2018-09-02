CREATE TABLE text (
	id integer PRIMARY KEY,
	result text NOT NULL
);
CREATE TABLE definitions (
	id integer PRIMARY KEY,
	applicant integer NOT NULL,
	argument integer NOT NULL
);
CREATE TABLE reductions (
	id integer PRIMARY KEY,
	normal_form integer NOT NULL
);
CREATE TABLE integers (
	id integer PRIMARY KEY,
	result integer NOT NULL
);
