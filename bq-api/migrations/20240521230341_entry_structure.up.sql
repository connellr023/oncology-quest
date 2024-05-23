CREATE TABLE IF NOT EXISTS supertasks(
    id SERIAL PRIMARY KEY,
    title TEXT NOT NULL,
    domain_id INT NOT NULL,
    FOREIGN KEY (domain_id) REFERENCES domains(id)
);

CREATE TABLE IF NOT EXISTS tasks(
    id SERIAL PRIMARY KEY,
    supertask_id INT,
    title TEXT NOT NULL,
    domain_id INT NOT NULL,
    FOREIGN KEY (domain_id) REFERENCES domains(id),
    FOREIGN KEY (supertask_id) REFERENCES supertasks(id)
);

CREATE TABLE IF NOT EXISTS subtasks(
    id SERIAL PRIMARY KEY,
    task_id INT,
    title TEXT NOT NULL,
    domain_id INT NOT NULL,
    FOREIGN KEY (domain_id) REFERENCES domains(id),
    FOREIGN KEY (task_id) REFERENCES tasks(id)
);