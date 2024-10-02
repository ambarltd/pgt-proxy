CREATE TABLE IF NOT EXISTS users (
  id SERIAL PRIMARY KEY,
  name VARCHAR(255) NOT NULL,
  email VARCHAR(255) NOT NULL,
  password VARCHAR(255) NOT NULL
);

-- Insert user only if email does not already exist
INSERT INTO users (name, email, password)
SELECT 'John Doe', 'john.doe@example.com', 'hashed_password'
    WHERE NOT EXISTS (
    SELECT 1 FROM users WHERE email = 'john.doe@example.com'
);