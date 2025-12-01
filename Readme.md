Rust E-Commerce API

A backend API built with Rust, Axum, MySQL, and JWT Authentication.
Supports user registration, login, role-based authorization, products, and orders.

📌 Features
✅ Authentication & Authorization

JWT-based authentication

Role-based access:

Public routes (no token required)

Customer routes (token + role = customer)

Admin routes (token + role = admin)

🧑‍💻 Users

User registration

Login with hashed passwords (bcrypt)

Role support: customer, admin

📦 Products

Admin can create categories & items

Customers can view items

🛍️ Orders

Customers can create orders

Customers can view only their own orders

🗄 Database

MySQL database using sqlx

Safe query binding

Automatic struct mapping

Project Structure<br>
src/<br>
│<br>
├── main.rs<br>
│<br>
├── db.rs<br>
│            <br>
│<br>
├── middleware/<br>
│   ├── auth.rs<br>      
│   └── mod.rs<br>
│ <br>
│<br>
├── models/<br>
│   ├── auth.rs<br>
│   ├── mod.rs<br>
│   ├── order.rs<br>
│   ├── product.rs<br>
│   └── user.rs<br>
│<br>
├── services/<br>
│   ├── mod.rs<br>          
│   ├── order.rs<br>
│   ├── product.rs<br>
│   └── user.rs<br>
│   <br>
│<br>
├── error.rs<br>
│<br>
├── state.rs<br>
│ <br>
└── config.rs<br>

🔧 Configurations
Environment Variables

Create a .env file:

DATABASE_URL=mysql://root:admin@localhost:3306/db_rust
JWT_SECRET=your-secret-key

🚀 Running the Application
Install dependencies
cargo build

Run migrations

If using sqlx-cli:

sqlx migrate run

Start the server
cargo run


Server runs by default on:

http://localhost:3000

🌐 API Endpoints
🔓 Public Routes
Method	Endpoint	Description
POST	/api/register	Register user
POST	/api/login	Login user
GET	/	Server status
👤 Customer Routes (Require JWT)
Method	Endpoint	Description
GET	/api/items	View all items
GET	/api/items/{id}	View single item
POST	/api/orders	Create order
GET	/api/orders	Get the user's orders
🛠 Admin Routes (Require Admin Role)
Method	Endpoint	Description
POST	/api/admin/items	Create product item
POST	/api/admin/categories	Create category
GET	/api/admin/users	Get all users
GET	/api/admin/users/{id}	Get single user
DELETE	/api/admin/users/{id}	Delete user
🔐 Authentication Workflow

User registers → password hashed using bcrypt

User logs in → receives a JWT

Each protected route:

Reads token from Authorization: Bearer <token>

Verifies with middleware

Loads the current user into Extension<User>

Additional middleware checks roles:

require_customer

require_admin

🛠 Technologies Used

Rust

Axum – web framework

SQLx – async database ORM

MySQL

JWT – authentication

bcrypt – password hashing

Tower – middleware layer

🧪 Testing

Using any API client (e.g., Postman):

Register new user

Login and obtain JWT token

Add header to protected routes:

Authorization: Bearer <your_jwt_token>

