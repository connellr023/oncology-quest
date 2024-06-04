CREATE TABLE IF NOT EXISTS supertasks(
    id SERIAL PRIMARY KEY,
    title TEXT NOT NULL,
    rotation_id INT NOT NULL,
    FOREIGN KEY (rotation_id) REFERENCES rotations(id)
);

CREATE TABLE IF NOT EXISTS tasks(
    id SERIAL PRIMARY KEY,
    supertask_id INT NOT NULL,
    title TEXT NOT NULL,
    rotation_id INT NOT NULL,
    FOREIGN KEY (rotation_id) REFERENCES rotations(id),
    FOREIGN KEY (supertask_id) REFERENCES supertasks(id)
);

CREATE TABLE IF NOT EXISTS subtasks(
    id SERIAL PRIMARY KEY,
    task_id INT NOT NULL,
    title TEXT NOT NULL,
    rotation_id INT NOT NULL,
    FOREIGN KEY (rotation_id) REFERENCES rotations(id),
    FOREIGN KEY (task_id) REFERENCES tasks(id)
);