CREATE TABLE order IF NOT EXIST (
    id INT AUTO_INCREMENT PRIMARY KEY,
    user_id INT NOT NULL,
    product_id INT NOT NULL,
    quantity INT NOT NULL,
    total_price DOUBLE NOT NULL,
    status VARCHAR(50) NOT NULL DEFAULT 'pending'
)