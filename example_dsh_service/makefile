# Please edit the following variables to match your project
TENANT=
TENANTUID=

# You can change the following variables, but it is not necessary as it will ask during runtime
# NOTE: it's not recommended to store CLIENT_SECRET in the makefile!
PLATFORM ?= $(shell bash -c 'read -p "Platform (dsh-dev or poc-dsh): " PLATFORM; echo $$PLATFORM')
TOPIC ?= $(shell bash -c 'read -p "Topic to read: " TOPIC; echo $$TOPIC')
CLIENT_ID ?= $(shell bash -c 'read -p "Rest API client ID: " CLIENT_ID; echo $$CLIENT_ID')
CLIENT_SECRET ?= $(shell bash -c 'read -s -p "Rest API client secret: " CLIENT_SECRET; echo $$CLIENT_SECRET')

# DO NOT EDIT BELOW THIS LINE
DOCKER_REPO_URL=registry.cp.kpn-dsh.com/$(TENANT)
VERSION=example
TAGNAME=dsh-sdk-example
IMAGE=$(DOCKER_REPO_URL)/$(TAGNAME):$(VERSION)

verify_var:
	@if [ -z "$(TENANT)" ]; then echo "TENANT is not set in make file. Please edit the makefile."; exit 1; fi
	@if [ -z "$(TENANTUID)" ]; then echo "TENANTUID is not set in make file. Please edit the makefile."; exit 1; fi
help:
	@echo "login		- login to the relevant repository"
	@echo "fix		- run dos2unix on every file"
	@echo "build		- build the image"
	@echo "rebuild		- build the image with the --no-cache flag"
	@echo "no-docker-build	- Cargo build on local machine"
	@echo "deploy		- deploy the image to DSH (requires Rest Credentials, see https://console.dsh-dev.dsh.np.aws.kpn.com/#/profiles/$(TENANT)/rest-api)"
	@echo "all		- shorthand for fix, build, push, show"
	@echo "run		- run the image in local docker"
	@echo "push		- push the  image to jfrog"
	@echo "show		- show the current make variables"
	@echo 
	@echo "Currently only supports to deploy to dsh-dev and poc-dsh on AWS"

login:
	docker login $(DOCKER_REPO_URL) 
fix: 
	find . -type f -print0 | xargs -0 dos2unix
build: verify_var 
	docker build -t $(TAGNAME) -f Dockerfile --build-arg UID=$(TENANTUID) --build-arg GID=$(TENANTUID) --platform linux/amd64 .
	docker tag  $(TAGNAME) $(IMAGE)
rebuild: verify_var
	docker build --no-cache -t $(tagname) -f Dockerfile  --build-arg UID=$(TENANTUID) --build-arg GID=$(TENANTUID) --platform linux/amd64 .
	docker tag  $(TAGNAME) $(IMAGE)
no-docker-build:
	cargo b
deploy: verify_var
	@echo
	@echo "To start the service on DSH from the command line, you need to have the Rest Credentials."
	@echo "See https://console.dsh-dev.dsh.np.aws.kpn.com/#/profiles/$(TENANT)/rest-api"
	@echo
	@echo "Else start the service from console."
	@echo 
	@read -r -p "Do you want to start the service on DSH from command line? [y/N] " DEPLOY_CHOICE; \
	if [ "$$DEPLOY_CHOICE" = "y" ]; then \
		clear; \
		make exec_deploy; \
	else \
        echo "Deployment canceled."; \
    fi
exec_deploy: 
	@clear
	@bash -c 'bash deployer.sh "$(IMAGE)" "$(PLATFORM)" "$(TENANT)" "$(TENANTUID)" "$(TOPIC)" "$(CLIENT_ID)" "$(CLIENT_SECRET)"'
all: verify_var
	make build
	make push
	make show
	make deploy
run:
	docker run -u $(TENANTUID):$(TENANTUID) -it --entrypoint "/bin/sh" $(IMAGE)
fun:
	@curl -s -H "Accept: application/json" https://icanhazdadjoke.com/ 
push:
	@echo "Pushing image to Harbor: $(IMAGE)"
	docker push $(IMAGE)
show:
	@echo "#make file configuration"
	@echo "#URL          :" $(DOCKER_REPO_URL)
	@echo "#TENANT       :" $(TENANT)
	@echo "#TENANTUID    :" $(TENANTUID)
	@echo "#TAG          :" $(TAGNAME)
	@echo "#version      :" $(VERSION)
	@echo "#image        :" $(IMAGE)
