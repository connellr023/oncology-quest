CREATE TABLE IF NOT EXISTS user_tasks(
    id SERIAL PRIMARY KEY,
    user_id INT NOT NULL,
    subtask_id INT NOT NULL,
    rotation_id INT NOT NULL,
    is_completed BOOLEAN NOT NULL,
    comment TEXT NOT NULL DEFAULT '',
    FOREIGN KEY (user_id) REFERENCES users(id),
    FOREIGN KEY (subtask_id) REFERENCES subtasks(id),
    FOREIGN KEY (rotation_id) REFERENCES rotations(id)
);