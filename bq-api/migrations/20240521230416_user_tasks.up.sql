CREATE TABLE user_tasks(
    id SERIAL PRIMARY KEY,
    user_id INT,
    subtask_id INT,
    completed BOOLEAN NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users(id),
    FOREIGN KEY (subtask_id) REFERENCES subtasks(id)
);