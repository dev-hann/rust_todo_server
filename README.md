# Todo List API

## Project Overview
This project is a Todo List API written in Rust. Users can create, read, update, and delete Todo items. The API is built using the Rocket framework.

## Tech Stack
- Rust
- Rocket
- Sled (Database)

## Installation
Follow these steps to install and set up the project:

1. Clone the project
2. Navigate to the project directory
3. Run the project using the `cargo run` command

```bash
git clone https://github.com/yourusername/yourproject.git
cd yourproject
cargo run
```

## Usage
The API endpoints are available under the `/api` path. Here are the available endpoints:

### Todo Related Endpoints
- `GET /api/todos` - Retrieve all Todo items.
- `GET /api/todos/<id>` - Retrieve a specific Todo item by ID.
- `POST /api/todos` - Create a new Todo item. (Requires Todo data in JSON format)
- `PATCH /api/todos/<id>` - Update a specific Todo item by ID. (Requires Todo data in JSON format)
- `DELETE /api/todos/<id>` - Delete a specific Todo item by ID.

### Authentication Related Endpoints
- `POST /api/auth/login` - Log in a user. (Requires user data in JSON format)
- `POST /api/auth/register` - Register a new user. (Requires user data in JSON format)
- `GET /api/auth/users` - Retrieve all user information.
- `GET /api/auth/users/<id>` - Retrieve specific user information by ID.

### Example
Here's an example of creating a Todo item:

```bash
curl -X POST http://localhost:8000/api/todos -H "Content-Type: application/json" -d '{"title": "New Todo Item"}'
```

Here's an example of logging in a user:

```bash
curl -X POST http://localhost:8000/api/auth/login -H "Content-Type: application/json" -d '{"username": "your_username", "password": "your_password"}'
```

## Contributing
If you would like to contribute, please fork this repository, commit your changes, and submit a pull request.

## License
This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.