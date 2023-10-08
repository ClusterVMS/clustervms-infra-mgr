# ClusterVMS Infrastructure Manager

Infrastructure manager that runs all necessary components for ClusterVMS via Docker Compose. This will likely be switched to Docker Swarm once multi-node support is implemented.

By default, the system will be available on http://clustervms.localhost.


## Usage
* (Linux): run `./run.sh`, then open your browser to http://clustervms.localhost
* (Windows): TBD; no scripts available yet.


## Runtime Dependencies
* docker
* docker compose plugin
* (Windows):
	* Windows Subsystem for Linux (needed to run Linux containers on Windows)
		* Windows Home cannot run Windows containers, so Linux containers via WSL allows us to support more Windows editions


## Debugging
* Logs from all components will be logged in the standard output of the infrastructure manager
	* You can also see logs of individual components with `docker logs container-name`, after identifying relevant container with `docker ps`
* For suspected routing issues, go to `http://clustervms.localhost:8080/` for the Traefik reverse proxy interface.


## Tips/Tricks
* The infrastructure manager needs to use the same docker network as the Traefik instance it spawns, or you'll get "Bad Gateway" when trying to talk to the infrastructure manager


## License

Licensed under the MIT License, or Apache License 2.0, at your option. Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in this application by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
