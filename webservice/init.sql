CREATE TABLE IF NOT EXISTS users (
	userid uuid PRIMARY KEY,
	username text NOT NULL,
	password text NOT NULL
);

CREATE TABLE IF NOT EXISTS queue (
    title text NOT NULL,
	queueid uuid NOT NULL,
	owner uuid REFERENCES users ON DELETE CASCADE,
	PRIMARY KEY (queueid)
);

CREATE TABLE IF NOT EXISTS aliases (
	name text PRIMARY KEY,
	userid uuid REFERENCES users ON DELETE CASCADE NOT NULL
);

CREATE TABLE IF NOT EXISTS tasks (
	id uuid PRIMARY KEY,
    creator uuid REFERENCES users(userid) ON DELETE SET NULL,
    queue uuid REFERENCES queue(queueid) ON DELETE CASCADE NOT NULL,
    title text NOT NULL,
    content text,
    priority integer,
    dueDate timestamp,
    creationTime timestamp NOT NULL DEFAULT 'now'
);
