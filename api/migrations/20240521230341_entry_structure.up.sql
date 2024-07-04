CREATE TABLE IF NOT EXISTS supertasks(
    id SERIAL PRIMARY KEY,
    title TEXT NOT NULL,
    rotation_id INT NOT NULL,
    FOREIGN KEY (rotation_id) REFERENCES rotations(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS tasks(
    id SERIAL PRIMARY KEY,
    supertask_id INT NOT NULL,
    title TEXT NOT NULL,
    rotation_id INT NOT NULL,
    FOREIGN KEY (rotation_id) REFERENCES rotations(id) ON DELETE CASCADE,
    FOREIGN KEY (supertask_id) REFERENCES supertasks(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS subtasks(
    id SERIAL PRIMARY KEY,
    task_id INT NOT NULL,
    title TEXT NOT NULL,
    rotation_id INT NOT NULL,
    FOREIGN KEY (rotation_id) REFERENCES rotations(id) ON DELETE CASCADE,
    FOREIGN KEY (task_id) REFERENCES tasks(id) ON DELETE CASCADE
);