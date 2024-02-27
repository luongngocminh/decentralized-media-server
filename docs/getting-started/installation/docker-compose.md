# Docker-compose

We can use `docker-compose` to deploy a atm0s-media-server cluster.

## Deploy a Cluster

First you will need to clone the docker-compose repository:
```bash
git clone https://github.com/8xFF/atm0s-docker-compose.git 
cd atm0s-docker-compose
```

Now you can compose up a cluster using:
```bash
docker compose --profile cluster up -d
```

By default the cluster profile won't include the connector server, to start a cluster along with the connector:
```bash
docker compose --profile cluster --profile connector up -d
```

## Deploy individual servers

You can also deploy each server individually:
```bash
docker compose --profile <server-1> --profile <server-2> up -d
```


## Configuration
By default, every service nodes will be seeding to the gateway node at `localhost` port `4000`, if you want to customize these configs try updating the env files accordingly. These configuration will respect the environment variable options provided in [Configuration](../../user-guide/configuration.md) 

### Secret

Cluster/Server's secret should be configured in `env/shared.env` so all nodes will share the same key:
```
SECRET=supersecretkey
```

### Seeding

You can provide a seed for the cluster by modifying the `env/gateway.env` file before running `docker compose`:
```
SEEDS=0@/ip4/127.0.0.1/udp/4000/ip4/127.0.0.1/tcp/4000
```
Each service will have an env file by their name, and a `shared.env` to globally override any local config.

### Zone

When setting up the cluster in multi-zone mode, you should provide a ZONE identification string using the `SDN_ZONE` variable in `env/shared.env` for each zone:
```
SDN_ZONE=sg-ap-1
```







