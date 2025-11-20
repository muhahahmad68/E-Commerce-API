CREATE TABLE IF NOT EXISTS users (
    id INT UNSIGNED AUTO_INCREMENT PRIMARY KEY,
    email VARCHAR(100) NOT NULL,
    name VARCHAR(100) NOT NULL,
    user_type ENUM('admin', 'customer') NOT NULL,
    password VARCHAR(255) NOT NULL
);
