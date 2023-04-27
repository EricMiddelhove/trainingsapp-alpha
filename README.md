# Trainingsapp

This repository contains code for my App, which should be capable of managing training plans, keeping track of excercises and weights used in regular workouts. 

This repository solely the API accessing the database.
---
# Implemented Security Measures
The implemented Measures:

## Authentication and Authorisation
- Authentication is handled by another server. A simple verify request containing an authentication token will tell this server whether a user is authenticated. 
- Authorisation is handled by this server. Each database object has an owner. Only the owner is permitted to read and write to that object. To keep the data on the database anonymous a unique user id is provided by the authorisation server.

The authorisation server used is [erics-auth-server](https://github.com/EricMiddelhove/auth-server) which should also be considered for the SE_09 Hand in.

## Transport Layer Security
- This Server is currently not deployed. Therefore no TLS Setup is in place. 
- On deployment HTTPS should be considered as a mandatory requirement for deployment. The Authorisation is using HTTP Headers and HTTP Bodies to exchange secrets. Having those encrypted is essential for the security of this system.


## Network security
- On deployment a strict firewall is needed. Default should be to deny traffic to every port.
- Only expose the API port. (With the current compose configuration port 443) Do not expose the DB Port (Currently 27017)

## Hardware Security
- On deployment an encrypted Hard drive should be selected


## Still to implement features before going to production
### Secrets
- The Auth server and this server authenticate themselves via a shared secret. A Key Vault which is responsible for regularly swapping keys and storing them securely should be implemented

### Logging
- Currently the logging is not existent. A proper way of logging requests as well as errors should be implemented

### Database !IMPORTANT!
- Currently the database uses no log in. This has to be changed
- Currently the database is secured by not exposing it. But that is definitely a point that could fail easily by misconfiguring the deployment server accidentaly

---
# Creating a development environement
These instructions guide you through the setup of a development environment, enabling you to compile the code on your own machine and running it locally.

## Using Docker
Using docker you will run a pre compiled version of this repository. It is the quickest way get this repository running

1. Ensure to have docker and docker compose installed
2. Clone this repository and navigate to it in your terminal
3. Configure the environment variables in the compose.yaml file - If you are using this as a development setup the compose is configured correctly, if you are deploying, make sure to use safe secrets and safe passwords
4. Run `docker compose up`- This will download the [ericmiddelhove/trainingsapp-alpha](https://hub.docker.com/repository/docker/ericmiddelhove/trainingsapp-alpha/general) docker image as well as the required [mongod db](https://hub.docker.com/_/mongo) image.

## Using Cargo
Using cargo enables you to build this project yourself and run it locally.

1. Ensure to have the rust compiler and  cargo compiled. [Install Rust](https://www.rust-lang.org/tools/install)
2. Clone this repository and navigate to it in your terminal
3. Configure the environment variables - The setup-dev-environment.sh script will do that for you.
4. Run `cargo run` to start the compilation and execute the project.