# $\text{Oncology Quest}$
> An application to aid Medical Oncology Trainees.

![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)
![Actix](https://img.shields.io/badge/actix-%23ffffff.svg?style=for-the-badge&logo=actix&logoColor=black)
![Postgres](https://img.shields.io/badge/postgres-%23316192.svg?style=for-the-badge&logo=postgresql&logoColor=white)
![Vue.js](https://img.shields.io/badge/vue-%2335495e.svg?style=for-the-badge&logo=vuedotjs&logoColor=%234FC08D)
![TypeScript](https://img.shields.io/badge/typescript-%23007ACC.svg?style=for-the-badge&logo=typescript&logoColor=white)
![AWS](https://img.shields.io/badge/AWS-%23FF9900.svg?style=for-the-badge&logo=amazon&logoColor=white)
![Nginx](https://img.shields.io/badge/nginx-%23009639.svg?style=for-the-badge&logo=nginx&logoColor=white)
![Docker](https://img.shields.io/badge/docker-%230db7ed.svg?style=for-the-badge&logo=docker&logoColor=white)
![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/connellr023/oncology-quest/ci.yml?style=for-the-badge&logo=docker)

<br />

#### Features
 - Straightforward user registration and authentication
 - Management dashboard for administrative users
 - Responsive and dark themed front end user interface
 - Client side caching in browser session storage of frequently required data that is possibly large
    - Rotation entry data
    - User task data

<br />

#### Security Measures
 - Role based authentication for admins and regular users
 - Password hashing with **BCRYPT** combined with 64-bit integer nonce used as a **salt**
 - Simple rate limiter on sensitive **API** routes
 - **HTTPS** connection in production with appropriate **cookies** and **CORS** policies
 - **REGEX** patterns that filter every user input to prevent **XSS** attacks as well as **SQL** injection but the backend already uses prepared statements anyway

<br />

#### API Integration and Penetration Testing
 - There is a seperate Rust crate in this repository that handles integration testing for the **API** endpoint of this web app
 - The integration tests send automated requests to the **API** and assert the expected responses (as if to simulate a real user)
 - These tests are executed by orchestrating **3** containers via **Docker Compose**
    - Database container running a **PostgreSQL** instance
        - This serves as a disposable database for pure testing purposes
        - It is easily instantiated and destroyed since it is running in a container
    - Endpoint container running an instance of the backend server
    - Penetration/Integration container
        - This is the container that sends requests to the endpoint

<br />

#### Deployment Details
 - This web app is deployed for users on **AWS**
 - Chosen service is **ECS Fargate** for easy container orchestration
    - Currently **1** container running a **NGINX** server to handle frontend static file requests
    - Currently **1** container running backend **API** services to handle **CRUD** related requests
    - Currently **1** **PostgreSQL** database instance running on **RDS**
- Additionally, an **Application Load Balancer** as well as **Route 53 DNS** services are employed to route users between the **frontend** and **API** container under the same domain name

<br />

#### Building API for production
```bash
cargo build --release --features production
```
When building for production, it is essential to enable the *production* feature as that will enable the specialized **cookie** and **CORS** policies as well as the **rate limiter**.

<br />

#### Backend Development Environment Variables
`.env` file sample for backend
```ini
HOST_IP=127.0.0.1
HOST_PORT=8080
DATABASE_URL=postgres://admin:password@localhost:5432/bqdev
```

<br />
<br />

<div align="center">
    Developed and tested by <b>Connell Reffo</b> in 2024.
</div>
