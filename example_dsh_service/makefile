TENANT=
TENANTUID=
DSH_PLATFORM=

DOCKER_REPO_URL=registry.cp.kpn-dsh.com/$(TENANT)
VERSION=example
tagname=dsh-sdk-example
image=$(DOCKER_REPO_URL)/$(tagname):$(VERSION)

help:
	@echo "login		- login to the relevant repository"
	@echo "fix		- run dos2unix on every file"
	@echo "build		- build the image"
	@echo "rebuild		- build the image with the --no-cache flag"
	@echo "no-docker-build	- Cargo build on local machine"
	@echo "deploy		- deploy the image to DSH (requires Rest API key, see https://console.dsh-dev.dsh.np.aws.kpn.com/#/profiles/$(TENANT)/rest-api)"
	@echo "all		- shorthand for fix, build, push, show"
	@echo "run		- run the image in local docker"
	@echo "push		- push the  image to jfrog"
	@echo "show		- show the current make variables"
login:
	docker login $(DOCKER_REPO_URL) 
fix: 
	find . -type f -print0 | xargs -0 dos2unix
build:
	@make set_var
	docker build -t $(tagname) -f Dockerfile --build-arg UID=$(TENANTUID) --build-arg GID=$(TENANTUID) --platform linux/amd64 .
	docker tag  $(tagname) $(image)
rebuild:
	make set_var
	docker build --no-cache -t $(tagname) -f Dockerfile  --build-arg UID=$(TENANTUID) --build-arg GID=$(TENANTUID) --platform linux/amd64 .
	docker tag  $(tagname) $(image)
no-docker-build:
	cargo b
deploy:
	@read -p "Would you like to deploy the image to DSH? [y/N] " choice; \
	if [ "$$choice" = "y" ]; then \
		read -p "Client ID: " client_id; \
		read -p "Client secret: " client_secret;  \
		read -p "Which topic to read: " topic; \
		./deployer.sh \
			$(image) \
			$(tokenUrl) \
			"$$client_id" \
			"$$client_secret" \
			$(configUrl) \
			$(repositoryName) \
			$(tenantId); \
			"$$topic" 

	else \
        echo "Deployment canceled."; \
    fi
all:
	make set_var
	make build
	make push
	make show
run:
	docker run -u $(TENANTUID):$(TENANTUID) -it --entrypoint "/bin/sh" $(image)
fun:
	@curl -s -H "Accept: application/json" https://icanhazdadjoke.com/ 
push:
	@echo "Pushing image to Harbor: $(image)"
	docker push $(image)
set_var:
	@if [ "$(DSH_PLATFORM)" = "" ]; then \
		read -p "DSH Platform: " url; \
		export DSH_PLATFORM=$$url; \
	fi
	@if [ "$(TENANT)" = "" ]; then \
		read -p "Tenant name: " uid; \
		export TENANT=$$uid; \
	fi
	@if [ "$(TENANTUID)" = "" ]; then \
		read -p "Tenant UID: " uid; \
		export TENANTUID=$$uid; \
	fi
show:
	@echo "#make file configuration"
	@echo "#URL          :" $(DOCKER_REPO_URL)
	@echo "#TENANT       :" $(TENANT)
	@echo "#TENANTUID 	 :" $(TENANTUID)
	@echo "#TAG          :" $(tagname)
	@echo "#version      :" $(VERSION)
	@echo "#image        :" $(image)
