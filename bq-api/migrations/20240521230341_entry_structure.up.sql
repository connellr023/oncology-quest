CREATE TABLE supertasks(
    id SERIAL PRIMARY KEY,
    title TEXT NOT NULL,
    domain_id INT,
    FOREIGN KEY (domain_id) REFERENCES domains(id)
);

CREATE TABLE tasks(
    id SERIAL PRIMARY KEY,
    supertask_id INT,
    title TEXT NOT NULL,
    domain_id INT,
    FOREIGN KEY (domain_id) REFERENCES domains(id),
    FOREIGN KEY (supertask_id) REFERENCES supertasks(id)
);

CREATE TABLE subtasks(
    id SERIAL PRIMARY KEY,
    task_id INT,
    title TEXT NOT NULL,
    domain_id INT,
    FOREIGN KEY (domain_id) REFERENCES domains(id),
    FOREIGN KEY (task_id) REFERENCES tasks(id)
);