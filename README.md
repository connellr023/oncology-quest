# $\text{Oncology Quest}$
> A progressive web app architected to aid Medical Oncology trainees and rotation directors. 

> [!IMPORTANT]
> This app is only meant for trainees. If that is not you, then *please* do **not** try using this service as it will place unnecassary load on the webserver and illegitimate accounts will be removed.

> [!NOTE]
> A flutter client is in development to facilitate a mobile app and *possibly* replace the web client in the future so that the client code lives on a single code base.

![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)
![Actix](https://img.shields.io/badge/actix-%23ffffff.svg?style=for-the-badge&logo=actix&logoColor=black)
![JWT](https://img.shields.io/badge/JWT-red?style=for-the-badge&logo=JSON%20web%20tokens)
![Postgres](https://img.shields.io/badge/postgres-%23316192.svg?style=for-the-badge&logo=postgresql&logoColor=white)
![Vue.js](https://img.shields.io/badge/vue-%2335495e.svg?style=for-the-badge&logo=vuedotjs&logoColor=%234FC08D)
![TypeScript](https://img.shields.io/badge/typescript-%23007ACC.svg?style=for-the-badge&logo=typescript&logoColor=white)
![Flutter](https://img.shields.io/badge/Flutter-%2302569B.svg?style=for-the-badge&logo=Flutter&logoColor=white)
![Dart](https://img.shields.io/badge/dart-%230175C2.svg?style=for-the-badge&logo=dart&logoColor=white)
![AWS](https://img.shields.io/badge/AWS-%23FF9900.svg?style=for-the-badge&logo=amazon&logoColor=white)
![Docker](https://img.shields.io/badge/docker-%230db7ed.svg?style=for-the-badge&logo=docker&logoColor=white)
![API CI Status](https://img.shields.io/github/actions/workflow/status/connellr023/oncology-quest/ci.yml?style=for-the-badge&logo=docker)
![Flutter Client CI Status](https://img.shields.io/github/actions/workflow/status/connellr023/oncology-quest/ci_flutter.yml?style=for-the-badge&logo=flutter)

<br />

#### Table of Contents
 - [Features](#features)
 - [Security Measures](#security-measures)
 - [API Testing](#api-integration-and-penetration-testing)
 - [Deployment Details](#deployment-details)
 - [Building](#building-api-for-production)
 - [Environment Variables](#backend-development-environment-variables)
 - [Screenshots](#screenshots)

<br />

#### Features
 - Straightforward user registration and authentication
 - Management dashboard for administrative users
 - Responsive and dark themed front end user interface
 - Caching and memoization of frequently accessed data to reduce server and database load

<br />

#### Security Measures
 - Role based authentication for admins and regular users
 - Password hashing with **BCRYPT** combined with 64-bit integer nonce used as a **salt**
 - Simple rate limiter on sensitive **API** routes
 - **HTTPS** connection in production with appropriate **cookies** and **CORS** policies
 - **REGEX** patterns that filter every user input to prevent **XSS** attacks as well as **SQL** injection but the backend already uses prepared statements anyway
 - Writing the backend in **Rust** is a security feature in itself

<br />

> [!WARNING]
> While the security measures in place are sufficient for the data sensitivity of this app, be smart with choosing passwords. Use one that is unique to this service.

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
 - This web app is deployed for users on **AWS** via **Elastic Container Service**
 - An **Application Load Balancer** as well as **Route 53 DNS** services are employed to route users to the registered domain name

<br />

#### Building API for production
```bash
cargo build --release --features "production"
```
When building for production, it is essential to enable the *production* feature as that will enable the specialized **cookie** and **CORS** policies as well as the **rate limiter**.

Additionally, for single container monolith use with the **API** also serving the static frontend files, use the *monolith* feature.
```bash
cargo build --release --features "production monolith"
```

<br />

Next, to containerize the compiled binary run:
```
docker build -t oncology-quest-api:latest .
```
in the `oncology-quest-api` directory.


If the frontend is to be run as a seperate **NGINX** webserver, build the front end with:
```bash
docker build -t oncology-quest-web:latest .
```
in the `oncology-quest-web` directory.

To containerize as a monolith (assuming **API** was compiled with *monolith* feature) run:
```bash
docker build -t oncology-quest-monolith:latest . -f monolith.dockerfile
```
in the root project directory.

<br />

#### Backend Development Environment Variables
`.env` file sample for backend
```ini
HOST_IP=127.0.0.1
HOST_PORT=8080
DATABASE_URL=postgres://admin:password@localhost:5432/bqdev
JWT_SECRET=...
```

<br />

#### Screenshots

![1](https://github.com/connellr023/cr023/blob/main/src/assets/oncology_quest/1.png?raw=true)
![2](https://github.com/connellr023/cr023/blob/main/src/assets/oncology_quest/2.png?raw=true)
![3](https://github.com/connellr023/cr023/blob/main/src/assets/oncology_quest/3.png?raw=true)
![4](https://github.com/connellr023/cr023/blob/main/src/assets/oncology_quest/4.png?raw=true)
![5](https://github.com/connellr023/cr023/blob/main/src/assets/oncology_quest/5.png?raw=true)
![6](https://github.com/connellr023/cr023/blob/main/src/assets/oncology_quest/6.png?raw=true)

<br />

<div align="center">
    Developed and Tested by <b>Connell Reffo</b> in 2024.
</div>
